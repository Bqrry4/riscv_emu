#[repr(u8)]
pub enum Size {
    BYTE = 1,
    HALFWORD = 2,
    WORD = 4,
    DOUBLEWORD = 8,
}

impl Size {
    #[inline(always)]
    pub fn from_unchecked(v: u8) -> Self {
        unsafe { core::mem::transmute::<u8, Self>(v) }
    }
}

pub struct Mmu {
    pub memory: [u32; 64],
}

impl Mmu {
    pub fn new() -> Self {
        Self {
            //Fill this with NOPs, which is 0x13 on riscv
            memory: [0x13; 64],
        }
    }

    pub fn load(&self, vaddr: u64, size: Size) -> u64 {
        0
    }

    pub fn store(&mut self, vaddr: u64, value: u64, size: Size) {}
}
