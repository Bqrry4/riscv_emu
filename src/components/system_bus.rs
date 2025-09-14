use crate::components::{
    devices::dram::{DRAM_SIZE, Dram},
    mmu::Size,
    trap::Exception,
};

// The execution environment determines the mapping of hardware resources into a hart’s address space.
// Use the ones defined by qemu:
//https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c
pub const DRAM_BASE: u64 = 0x8000_0000;
pub const DRAM_END: u64 = DRAM_BASE + DRAM_SIZE;
pub struct SystemBus {
    dram: Dram,
}

impl SystemBus {
    pub fn new() -> Self {
        Self { dram: Dram::new() }
    }

    pub fn read(&self, address: u64, size: Size) -> Result<u64, Exception> {
        Ok(0)
    }
    pub fn write(&self, address: u64, size: Size, value: u64) -> Result<(), Exception> {
        Ok(())
    }
}
