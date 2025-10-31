use std::pin::Pin;

use arbitrary_int::u2;
use bitbybit::bitenum;

use crate::components::csr::{Csr, MIE, MIP, MSTATUS, SAPT};
use crate::components::devices::uart::IRQ_UART;
use crate::components::mmu::Mmu;
use crate::components::registers::XRegisters;
use crate::components::system_bus::MROM_BASE;
use crate::components::trap::{Exception, Interrupt};
use crate::instructions::decode_and_execute;
use crate::util::{F, T};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
#[bitenum(u2, exhaustive = true)]
pub enum PrivilegeMode {
    User = 0b00,
    Supervisor = 0b01,
    Machine = 0b11,
    //Ignored, only present for `from_unchecked` conversion
    Reserved = 0b10,
}

impl PrivilegeMode {
    #[inline(always)]
    pub fn from_unchecked(v: u2) -> Self {
        unsafe { core::mem::transmute::<u2, Self>(v) }
    }
}

pub struct Cpu {
    pub x_regs: XRegisters,
    pub pc: u64,
    pub mmu: Mmu,
    //mmu depends on those, so they live on heap
    // TODO: check if an arena over all struct components could solve the referencing issue
    pub p_mode: Pin<Box<PrivilegeMode>>,
    pub csr: Pin<Box<Csr>>,
    //& The invalidation of a hart’s reservation when it executes an LR or SC imply that a hart can only hold one reservation at a time
    pub reservation: Option<u64>,
    pub is_idle: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let csr = Box::pin(Csr::new());
        let mstatus = &csr.csrs[MSTATUS];
        let sapt = &csr.csrs[SAPT];
        let p_mode = Box::pin(PrivilegeMode::Machine);

        let cpu = Self {
            x_regs: XRegisters::new(),
            // start in firmware
            pc: MROM_BASE,
            mmu: Mmu::new(mstatus, sapt, p_mode.as_ref().get_ref()),
            csr: csr,
            p_mode: p_mode,
            reservation: None,
            is_idle: false,
        };
        cpu
    }
    pub fn run(&mut self) {
        while self.mmu.bus.test.exit == 0 {
            self.tick();
        }
        println!("Exited with {}", self.mmu.bus.test.exit);
    }

    fn handle_exception(&mut self, e: Exception) {
        println!("Exception {:?}", e.code());
        e.take_trap(self);
    }

    fn handle_interrupt(&mut self) {
        match *self.p_mode {
            PrivilegeMode::Machine => {
                if self.csr.read_mstatus().mie() == F {
                    // Machine mode interrupt disabled
                    return;
                }
            }
            PrivilegeMode::Supervisor => {
                if self.csr.read_mstatus().sie() == F {
                    // Supervisor mode interrupt disabled
                    return;
                }
            }
            _ => {}
        }

        let irq: u32 = if self.mmu.bus.uart0.is_interrupting() {
            IRQ_UART
        } else {
            return;
        };
        self.mmu.bus.plic.set_pending(irq, true);
        let pending = MIP::new_with_raw_value(self.csr.read(MIP) & self.csr.read(MIE));

        let interrupt: Option<Interrupt> = match pending {
            mut p if p.meip() == T => {
                p.set_meip(F);
                Some(Interrupt::MachineExternal)
            }
            mut p if p.msip() == T => {
                p.set_msip(F);
                Some(Interrupt::MachineSoftware)
            }
            mut p if p.mtip() == T => {
                p.set_mtip(F);
                Some(Interrupt::MachineTimer)
            }
            mut p if p.seip() == T => {
                p.set_seip(F);
                Some(Interrupt::SupervisorExternal)
            }
            mut p if p.ssip() == T => {
                p.set_ssip(F);
                Some(Interrupt::SupervisorSoftware)
            }
            mut p if p.stip() == T => {
                p.set_stip(F);
                Some(Interrupt::SupervisorTimer)
            }
            _ => None,
        };
        // Writeback MIP
        self.csr.write(MIP, pending.raw_value());

        if let Some(interrupt) = interrupt {
            interrupt.take_trap(self);
        }
    }

    pub fn tick(&mut self) {
        self.handle_interrupt();

        if self.is_idle {
            return;
        }

        //exception block
        let _ = (|| -> Result<(), Exception> {
            // IF - instruction fetch stage
            // fetch
            let enc_inst = self.mmu.fetch(self.pc)?;
            // and inc pc
            self.pc += 4;
            // decode + execute
            decode_and_execute(self, enc_inst)?;
            Ok(())
        })()
        .map_err(|e| self.handle_exception(e));
    }

    pub fn dump_state(&self) {
        println!("Xreg: {:?}", self.x_regs);
        println!("PC: {}", self.pc);
    }
}
