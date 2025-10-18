use crate::components::{
    devices::{
        dram::{DRAM_SIZE, Dram},
        rom::Mrom,
        test::Test,
        uart::{UART_SIZE, Uart},
    },
    mmu::Size,
    trap::Exception,
};

/* Device memory mapping */
//& The execution environment determines the mapping of hardware resources into a hart’s address space.
// Use the ones defined by qemu:
// https://github.com/qemu/qemu/blob/master/hw/riscv/virt.c
pub const MROM_BASE: u64 = 0x1000;
pub const MROM_END: u64 = MROM_BASE + 0xf000;
pub const TEST_BASE: u64 = 0x10_0000;
pub const TEST_END: u64 = TEST_BASE + 0x1000;
pub const UART0_BASE: u64 = 0x1000_0000;
pub const UART0_END: u64 = UART0_BASE + UART_SIZE;
pub const DRAM_BASE: u64 = 0x8000_0000;
pub const DRAM_END: u64 = DRAM_BASE + DRAM_SIZE;
/* Known memory regions */
pub const SBI_REGION: u64 = DRAM_BASE;
pub const KERNEL_REGION: u64 = DRAM_BASE + 0x4_000;

pub struct SystemBus {
    rom: Mrom,
    pub test: Test,
    dram: Dram,
    uart0: Uart,
}

impl SystemBus {
    pub fn new() -> Self {
        Self {
            rom: Mrom::new(),
            test: Test::new(),
            dram: Dram::new(),
            uart0: Uart::new(),
        }
    }

    pub fn read(&mut self, address: u64, size: Size) -> Result<u64, Exception> {
        match address {
            MROM_BASE..=MROM_END => Ok(self.rom.read(address - MROM_BASE, size)?),
            DRAM_BASE..=DRAM_END => Ok(self.dram.read(address - DRAM_BASE, size)?),
            UART0_BASE..=UART0_END => match size {
                Size::BYTE => Ok(self.uart0.read(address - UART0_BASE)? as u64),
                _ => Err(Exception::LoadAccessFault),
            },
            _ => Err(Exception::LoadAccessFault),
        }
    }
    pub fn write(&mut self, address: u64, size: Size, value: u64) -> Result<(), Exception> {
        match address {
            TEST_BASE..=TEST_END => Ok(self.test.write(address - TEST_BASE, size, value)),
            DRAM_BASE..=DRAM_END => Ok(self.dram.write(address - DRAM_BASE, size, value)?),
            UART0_BASE..=UART0_END => match size {
                Size::BYTE => Ok(self.uart0.write(address - UART0_BASE, value as u8)?),
                _ => Err(Exception::StoreAccessFault),
            },
            _ => Err(Exception::StoreAccessFault),
        }
    }

    pub fn inject(&mut self, address: u64, bin: &[u8]) {
        self.dram.write_bytes(address - DRAM_BASE, bin);
    }
}
