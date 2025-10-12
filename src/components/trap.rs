use arbitrary_int::u1;

use crate::{
    components::csr::{MCAUSE, MEDELEG, MEPC, MIDELEG, MTVAL, MTVEC, SCAUSE, SEPC, STVAL, STVEC},
    cpu::{Cpu, PrivilegeMode},
};

pub enum Trap {
    /* The trap is visible to, and handled by, software running inside the execution environment. */
    Contained,
    /* The trap is a synchronous exception that is an explicit call to the execution environment
     * requesting an action on behalf of software inside the execution environment. */
    Requested,
    /* The trap is handled transparently by the execution environment
     * and execution resumes normally after the trap is handled. */
    Invisible,
    /* The trap represents a fatal failure and causes the execution environment to terminate execution. */
    Fatal,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Exception {
    InstructionAddressMisaligned = 0,
    InstructionAccessFault,
    IllegalInstruction,
    Breakpoint,
    LoadAddressMisaligned,
    LoadAccessFault,
    StoreAddressMisaligned,
    StoreAccessFault,
    EnvironmentCallFromUMode,
    EnvironmentCallFromSMode,
    EnvironmentCallFromMMode = 11,
    InstructionPageFault(u64),
    LoadPageFault(u64),
    StorePageFault(u64) = 15,
    DoubleTrap,
    SoftwareCheck = 18,
    HardwareError,
}

impl Exception {
    fn epc(&self, pc: u64) -> u64 {
        //& ECALL and EBREAK cause the receiving privilege mode’s epc register to be set to the address of the ECALL or EBREAK instruction itself,
        //& not the address of the following instruction.
        //? spec says that xepc should be written with the address of the instruction that triggered the trap,
        // tho it doesn't make much sense.
        match self {
            Exception::Breakpoint
            | Exception::EnvironmentCallFromUMode
            | Exception::EnvironmentCallFromSMode
            | Exception::EnvironmentCallFromMMode => pc.wrapping_sub(4),
            _ => pc,
        }
    }

    fn tval(&self, pc: u64) -> u64 {
        //& If mtval is written with a nonzero value when a breakpoint, address-misaligned, access-fault, page-fault,
        //& or hardware-error exception occurs on an instruction fetch, load, or store, then mtval will contain the faulting virtual address.
        match self {
            Exception::Breakpoint
            | Exception::LoadAccessFault
            | Exception::StoreAccessFault
            | Exception::InstructionAccessFault
            | Exception::LoadAddressMisaligned
            | Exception::StoreAddressMisaligned
            | Exception::InstructionAddressMisaligned => pc,
            Exception::InstructionPageFault(addr)
            | Exception::LoadPageFault(addr)
            | Exception::StorePageFault(addr) => *addr,
            _ => 0,
        }
    }
    // An enum with payloads can't be casted to an int
    pub fn code(&self) -> u64 {
        match self {
            Exception::InstructionAddressMisaligned => 0,
            Exception::InstructionAccessFault => 1,
            Exception::IllegalInstruction => 2,
            Exception::Breakpoint => 3,
            Exception::LoadAddressMisaligned => 4,
            Exception::LoadAccessFault => 5,
            Exception::StoreAddressMisaligned => 6,
            Exception::StoreAccessFault => 7,
            Exception::EnvironmentCallFromUMode => 8,
            Exception::EnvironmentCallFromSMode => 9,
            Exception::EnvironmentCallFromMMode => 11,
            Exception::InstructionPageFault(_) => 12,
            Exception::LoadPageFault(_) => 13,
            Exception::StorePageFault(_) => 15,
            Exception::DoubleTrap => 16,
            Exception::SoftwareCheck => 18,
            Exception::HardwareError => 19,
        }
    }

    pub fn take_trap(&self, cpu: &mut Cpu) {
        let cause = self.code();
        let epc = self.epc(cpu.pc);
        let pp_mode = *cpu.p_mode;
        let medeleg = cpu.csr.read(MEDELEG);
        let mut mstatus = cpu.csr.read_mstatus();
        //& Traps never transition from a more-privileged mode to a less-privileged mode.
        if pp_mode <= PrivilegeMode::Supervisor && ((medeleg >> cause) & 1) == 1 {
            // Switch to S-mode
            *cpu.p_mode = PrivilegeMode::Supervisor;
            //& When a trap is taken from privilege mode y into privilege mode x,
            //& xPIE is set to the value of xIE;
            mstatus.set_spie(mstatus.sie());
            //& xIE is set to 0;
            mstatus.set_sie(u1::new(0));
            //& and xPP is set to y.
            mstatus.set_spp(u1::new(pp_mode as u8));
            // Jump to Supervisor trap handler
            cpu.pc = cpu.csr.read(STVEC) & !3;
            //& When a trap is taken into S-mode, sepc is written with the virtual address of the instruction that was interrupted or that encountered the exception.
            //& The low bit of sepc (sepc[0]) is always zero.
            cpu.csr.write(SEPC, epc & !1);
            //& When a trap is taken into S-mode, scause is written with a code indicating the event that caused the trap.
            cpu.csr.write(SCAUSE, cause);
            cpu.csr.write(STVAL, self.tval(cpu.pc));
        } else {
            // Switch to M-mode
            *cpu.p_mode = PrivilegeMode::Supervisor;
            //& When a trap is taken from privilege mode y into privilege mode x,
            //& xPIE is set to the value of xIE;
            mstatus.set_mpie(mstatus.mie());
            //& xIE is set to 0;
            mstatus.set_mie(u1::new(0));
            //& and xPP is set to y.
            mstatus.set_mpp(pp_mode);
            // Jump to Machine trap handler
            cpu.pc = cpu.csr.read(MTVEC) & !3;
            //& When a trap is taken into M-mode, mepc is written with the virtual address of the instruction that was interrupted or that encountered the exception.
            //& The low bit of mepc (mepc[0]) is always zero.
            cpu.csr.write(MEPC, epc & !1);
            //& When a trap is taken into M-mode, mcause is written with a code indicating the event that caused the trap.
            cpu.csr.write(MCAUSE, cause);
            cpu.csr.write(MTVAL, self.tval(cpu.pc));
        }
        cpu.csr.write_mstatus(&mstatus);
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    SupervisorSoftware = 1,
    MachineSoftware = 3,
    SupervisorTimer = 5,
    MachineTimer = 7,
    SupervisorExternal = 9,
    MachineExternal = 11,
    CounterOverflow = 13,
}

impl Interrupt {
    pub fn take_trap(&self, cpu: &mut Cpu) {
        let cause = *self as u64;
        let epc = cpu.pc;
        let pp_mode = *cpu.p_mode;
        let mideleg = cpu.csr.read(MIDELEG);
        let mut mstatus = cpu.csr.read_mstatus();
        //& Traps never transition from a more-privileged mode to a less-privileged mode.
        if pp_mode <= PrivilegeMode::Supervisor && ((mideleg >> cause) & 1) == 1 {
            // Switch to S-mode
            *cpu.p_mode = PrivilegeMode::Supervisor;
            //& When a trap is taken from privilege mode y into privilege mode x,
            //& xPIE is set to the value of xIE;
            mstatus.set_spie(mstatus.sie());
            //& xIE is set to 0;
            mstatus.set_sie(u1::new(0));
            //& and xPP is set to y.
            mstatus.set_spp(u1::new(pp_mode as u8));
            // Jump to Supervisor trap handler
            let stvec = cpu.csr.read(STVEC);
            let offset = match stvec & 3 {
                // Vectored
                //& Asynchronous interrupts set pc to BASE+4×cause.
                1 => 4 * cause,
                // Direct
                //& All traps set pc to BASE.
                _ => 0,
            };
            cpu.pc = (stvec & !3) + offset;
            //& When a trap is taken into S-mode, sepc is written with the virtual address of the instruction that was interrupted or that encountered the exception.
            //& The low bit of sepc (sepc[0]) is always zero.
            cpu.csr.write(SEPC, epc & !1);
            //& When a trap is taken into S-mode, scause is written with a code indicating the event that caused the trap.
            // Set MSB to indicate an interrupt
            cpu.csr.write(SCAUSE, cause | 1 << 63);
            cpu.csr.write(STVAL, 0);
        } else {
            // Switch to M-mode
            *cpu.p_mode = PrivilegeMode::Supervisor;
            //& When a trap is taken from privilege mode y into privilege mode x,
            //& xPIE is set to the value of xIE;
            mstatus.set_mpie(mstatus.mie());
            //& xIE is set to 0;
            mstatus.set_mie(u1::new(0));
            //& and xPP is set to y.
            mstatus.set_mpp(pp_mode);
            // Jump to Machine trap handler
            let mtvec = cpu.csr.read(MTVEC);
            let offset = match mtvec & 3 {
                // Vectored
                //& Asynchronous interrupts set pc to BASE+4×cause.
                1 => 4 * cause,
                // Direct
                //& All traps set pc to BASE.
                _ => 0,
            };
            cpu.pc = (mtvec & !3) + offset;
            //& When a trap is taken into M-mode, mepc is written with the virtual address of the instruction that was interrupted or that encountered the exception.
            //& The low bit of mepc (mepc[0]) is always zero.
            cpu.csr.write(MEPC, epc & !1);
            //& When a trap is taken into M-mode, mcause is written with a code indicating the event that caused the trap.
            // Set MSB to indicate an interrupt
            cpu.csr.write(MCAUSE, cause | 1 << 63);
            cpu.csr.write(MTVAL, 0);
        }
        cpu.csr.write_mstatus(&mstatus);
    }
}
