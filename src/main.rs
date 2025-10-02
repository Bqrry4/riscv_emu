use argh::FromArgs;
use cpu::*;

use crate::components::{
    mmu::Size,
    system_bus::{DRAM_BASE, KERNEL_REGION, SBI_REGION},
};
pub mod cpu;

mod components;
mod instructions;
pub mod util;

#[derive(FromArgs)]
#[argh(description = "?╱|、
(˚ˎ 。7
 |、˜〵
 じしˍ,)ノ")]
struct Args {
    /// sbi or binary
    #[argh(option, short = 'b')]
    sbi: String,

    /// kernel
    #[argh(option, short = 'k')]
    kernel: Option<String>,
}

fn main() {
    let args: Args = argh::from_env();

    let mut cpu = Cpu::new();

    let sbi = std::fs::read(args.sbi).unwrap();
    cpu.mmu.inject(SBI_REGION, &sbi);
    args.kernel.map(|k| {
        let kernel = std::fs::read(k).unwrap();
        cpu.mmu.inject(KERNEL_REGION, &kernel);
    });

    cpu.run();
    cpu.dump_state();
}
