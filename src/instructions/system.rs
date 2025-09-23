use crate::{
    components::{
        csr::{MEPC, SEPC},
        trap::Exception,
    },
    cpu::{Cpu, PrivilegeMode},
    instructions::types::IType,
};
use arbitrary_int::{u1, u2, u3, u5};

#[allow(dead_code)]
/// Read/Write CSR
pub const CSRRW: u8 = 0x1;
/// Read and Set Bits in CSR
pub const CSRRS: u8 = 0x2;
/// Read and Clear Bits in CSR
pub const CSRRC: u8 = 0x3;
/// Same as the above, except bit[2] tells the rsi is an imm value
pub const CSRRWI: u8 = 0x5;
pub const CSRRSI: u8 = 0x6;
pub const CSRRCI: u8 = 0x7;
/// Used to return from a trap taken into S-mode
pub const SRET: u16 = 0x102;
/// Used to return from a trap taken into M-mode
pub const MRET: u16 = 0x302;
pub const WFI: u16 = 0x105;
pub const ECALL: u16 = 0x0;
pub const EBREAK: u16 = 0x1;

//@Note: in theory those instructions should execute atomically
pub fn handle_system(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let itype = IType::new_with_raw_value(instr);
    let (rd, funct3, rsi, csr) = (itype.rd(), itype.funct3(), itype.rs1(), itype.imm());

    //get rid of the sign extended bytes
    let csr_addr = (csr.value() as u16) & 0xfff;

    //handle the funct3=0 subclass
    if funct3.value() == 0 {
        match csr_addr {
            ECALL => instr_ecall(cpu)?,
            EBREAK => instr_ebreak()?,
            SRET => instr_sret(cpu),
            MRET => instr_mret(cpu),
            WFI => instr_wfi(cpu),
            _ => {}
        }
        return Ok(());
    }

    handle_zicsr(cpu, rd, rsi, csr_addr, funct3)
}

fn handle_zicsr(
    cpu: &mut Cpu,
    rd: u5,
    rsi: u5,
    csr_addr: u16,
    funct3: u3,
) -> Result<(), Exception> {
    // TODO: handle the cases of rs1=x0 (?)
    // handle Zicsr extension
    // check for bit[2]
    let rs_val = if (funct3.value() >> 2) != 0 {
        //rsi is an imm
        rsi.value() as u64
    } else {
        //rsi is a reg
        cpu.x_regs.read(rsi)
    };
    let csr_val = cpu.csr.read(csr_addr as usize);
    match funct3.value() & 0x3 {
        CSRRW => {
            //swap the values
            cpu.csr.write(csr_addr as usize, rs_val);
        }
        CSRRS => {
            //reg_val act as a set bit mask
            cpu.csr.write(csr_addr as usize, csr_val | rs_val);
        }
        CSRRC => {
            //reg_val act as a clear bit mask
            cpu.csr.write(csr_addr as usize, csr_val & (!rs_val));
        }
        _ => {
            // The hypervisor instructions would land here for now, as there isn't a planned support for it.
            return Err(Exception::IllegalInstruction);
        }
    }
    cpu.x_regs.write(rd, csr_val);
    Ok(())
}

fn instr_ecall(cpu: &Cpu) -> Result<(), Exception> {
    match *cpu.p_mode {
        PrivilegeMode::User => Err(Exception::EnvironmentCallFromUMode),
        PrivilegeMode::Supervisor => Err(Exception::EnvironmentCallFromSMode),
        PrivilegeMode::Machine => Err(Exception::EnvironmentCallFromMMode),
        PrivilegeMode::Reserved => Err(Exception::IllegalInstruction),
    }
}
fn instr_ebreak() -> Result<(), Exception> {
    Err(Exception::Breakpoint)
}
/* 3.3.2. Trap-Return Instructions */
fn instr_sret(cpu: &mut Cpu) {
    //Restore pc
    cpu.pc = cpu.csr.read(SEPC);
    //x=S
    let mut mstatus = cpu.csr.read_mstatus();
    //& xIE is set to xPIE
    mstatus.set_sie(mstatus.spie());
    //& the privilege mode is changed to xPP
    cpu.p_mode.set(PrivilegeMode::from_unchecked(u2::new(
        mstatus.spp().value(),
    )));
    //& xPIE is set to 1
    mstatus.set_spie(u1::new(1));
    //& xPP is set to the least-privileged supported mode (U if U-mode is implemented, else M)
    mstatus.set_spp(u1::new(PrivilegeMode::User as u8));
    //& If xPP!=M, xRET also sets MPRV=0
    if *cpu.p_mode != PrivilegeMode::Machine {
        //& SRET clears the mprv, so assume it uses mstatus instead of sstatus
        mstatus.set_mprv(u1::new(0));
    }
    cpu.csr.write_mstatus(&mstatus);
}
fn instr_mret(cpu: &mut Cpu) {
    //Restore pc
    cpu.pc = cpu.csr.read(MEPC);
    //x=M
    /* 3.1.6.1. Privilege and Global Interrupt-Enable Stack */
    let mut mstatus = cpu.csr.read_mstatus();
    //& xIE is set to xPIE
    mstatus.set_mie(mstatus.mpie());
    //& the privilege mode is changed to xPP
    cpu.p_mode.set(mstatus.mpp());
    //& xPIE is set to 1
    mstatus.set_mpie(u1::new(1));
    //& xPP is set to the least-privileged supported mode (U if U-mode is implemented, else M)
    mstatus.set_mpp(PrivilegeMode::User);
    //& If xPP!=M, xRET also sets MPRV=0
    if *cpu.p_mode != PrivilegeMode::Machine {
        mstatus.set_mprv(u1::new(0));
    }
    cpu.csr.write_mstatus(&mstatus);
}
fn instr_wfi(cpu: &mut Cpu) {
    //& The Wait for Interrupt instruction (WFI) informs the implementation
    //& that the current hart can be stalled until an interrupt might need servicing.
    cpu.is_idle = true;
}
