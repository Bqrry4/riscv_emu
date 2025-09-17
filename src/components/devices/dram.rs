use crate::components::{
    mmu::Size::{self, *},
    trap::Exception,
};

pub const DRAM_SIZE: u64 = 512 * 1024;

pub struct Dram {
    memory: Vec<u8>,
}
 
impl Dram {
    pub fn new() -> Self {
        Self {
            memory: vec![0; DRAM_SIZE as usize],
        }
    }

    pub fn read(&self, index: u64, size: Size) -> Result<u64, Exception> {
        let index = index as usize;
        let bytes = self
            .memory
            .get(index..index + (size as usize))
            .ok_or(Exception::LoadAccessFault)?;

        let data = match size {
            BYTE => bytes[0] as u64,
            HWORD => u16::from_le_bytes(bytes.try_into().unwrap()) as u64,
            WORD => u32::from_le_bytes(bytes.try_into().unwrap()) as u64,
            DWORD => u64::from_le_bytes(bytes.try_into().unwrap()),
        };
        Ok(data)
    }

    pub fn write(&mut self, index: u64, size: Size, value: u64) -> Result<(), Exception> {
        let index = index as usize;
        let size = size as usize;
        let slice = self
            .memory
            .get_mut(index..index + (size))
            .ok_or(Exception::StoreAccessFault)?;

        let bytes = value.to_le_bytes();
        slice.copy_from_slice(&bytes[..size]);

        Ok(())
    }
}
