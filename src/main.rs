use cpu::*;
pub mod cpu;

mod components;
mod instructions;
pub mod util;

fn main() {
    let mut cpu = Cpu::new();

    /* Load the eq of
     * addi x1, x0, 1
     * addi x2, x0, 2
     * add x3, x1, x2
     */
    cpu.mmu.memory[0] = 0x00100093;
    cpu.mmu.memory[1] = 0x00200113;
    cpu.mmu.memory[2] = 0x002081b3;
    

    cpu.run();
    cpu.dump_state();
}
