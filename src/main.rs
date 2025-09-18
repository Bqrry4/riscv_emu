use cpu::*;

use crate::components::{mmu::Size, system_bus::DRAM_BASE};
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
    let _ = cpu.mmu.store(DRAM_BASE + 0, 0x00100093, Size::WORD);
    let _ = cpu.mmu.store(DRAM_BASE + 4, 0x00200113, Size::WORD);
    let _ = cpu.mmu.store(DRAM_BASE + 8, 0x002081b3, Size::WORD);
    cpu.pc = DRAM_BASE;

    cpu.run();
    cpu.dump_state();
}
