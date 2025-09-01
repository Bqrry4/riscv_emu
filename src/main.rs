use cpu::*;
pub mod cpu;

// use instructions::*;
mod instructions;

use util::*;
pub mod util;

fn main() {
    let mut cpu = CPU::new();

    /* Load the eq of
     * addi x1, x0, 1
     * addi x2, x0, 2
     * add x3, x1, x2
     */
    cpu.memory[0] = 0x00100093;
    cpu.memory[1] = 0x00200113;
    cpu.memory[2] = 0x002081b3;

    cpu.run();
    cpu.dump_state();
}
