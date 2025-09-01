use crate::components::registers::XRegisters;
use crate::instructions::decode;

pub struct CPU {
    pub x_regs: XRegisters,
    pub pc: u64,
    pub memory: [u32; 64],
}

impl CPU {
    pub fn new() -> Self {
        let cpu = Self {
            x_regs: XRegisters::new(),
            pc: 0,
            //Fill this with NOPs, which is 0x13 on riscv
            memory: [0x13; 64],
        };
        cpu
    }

    pub fn run(&mut self) {
        loop {
            if self.pc == self.memory.len() as u64 {
                println!("Reached end of code");
                return;
            }
            self.tick();
        }
    }

    pub fn tick(&mut self) {
        // fetch
        let enc_inst = self.memory[self.pc as usize];
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
        println!("MEM: {:?}", self.memory);
    }
}
