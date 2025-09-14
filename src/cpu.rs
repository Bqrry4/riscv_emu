use std::pin::Pin;

use arbitrary_int::u2;
use bitbybit::bitenum;

use crate::components::csr::{Csr, MSTATUS, SAPT};
use crate::components::mmu::Mmu;
use crate::components::registers::XRegisters;
use crate::instructions::decode;

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
        };
        cpu
    }

    pub fn run(&mut self) {
        loop {
            if self.pc == self.mmu.memory.len() as u64 {
                println!("Reached end of code");
                return;
            }
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        // fetch
        let enc_inst = self.mmu.memory[self.pc as usize];
        // decode
        let inst_fn = decode(enc_inst).unwrap_or_else(|| {
            //should raise a cpu exception
            panic!("Instruction not supported");
        });
        // execute
        inst_fn(self, enc_inst);
        // inc pc
        self.pc = self.pc + 1;
    }

    pub fn dump_state(&self) {
        println!("Xreg: {:?}", self.x_regs);
        println!("PC: {}", self.pc);
        println!("MEM: {:?}", self.mmu.memory);
    }
}
