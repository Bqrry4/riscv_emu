mod helper;
use risc_v::{
    components::{registers::XRegisters, system_bus::DRAM_BASE},
    cpu::Cpu,
};

define_test!(add, |cpu| {
    assert_eq!(cpu.x_regs.read(XRegisters::t3), 1);
    assert_eq!(cpu.x_regs.read(XRegisters::t4), 2);
    assert_eq!(cpu.x_regs.read(XRegisters::t5), 3);
});
