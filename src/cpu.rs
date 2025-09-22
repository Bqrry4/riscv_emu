use std::pin::Pin;

use arbitrary_int::u2;
use bitbybit::bitenum;

use crate::components::csr::{Csr, MSTATUS, SAPT};
use crate::components::mmu::Mmu;
use crate::components::registers::XRegisters;
use crate::components::system_bus::{DRAM_BASE, DRAM_END};
use crate::components::trap::Exception;
use crate::instructions::decode_and_execute;

#[derive(PartialEq, Eq)]
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
            pc: 0,
            mmu: Mmu::new(mstatus, sapt, p_mode.as_ref().get_ref()),
            csr: csr,
            p_mode: p_mode,
            reservation: None,
            is_idle: false,
        };
        cpu
    }

    pub fn run(&mut self) {
        loop {
            //TODO: remove hardcoded condition
            if self.pc >= DRAM_BASE + 8 as u64 {
                println!("Reached end of code");
                return;
            }
            self.tick();
        }
    }

    //TODO: handle exceptions obvs
    fn handle_exception(&self, e: Exception) {
        println!("Exception {:?}", e as u8);
        panic!()
    }

    pub fn tick(&mut self) {
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
        println!("MEM: {:?}", self.mmu.memory);
    }
}
