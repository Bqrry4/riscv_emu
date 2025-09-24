use crate::components::{
    mmu::Size::{self, *},
    trap::Exception,
};

//TODO: Set a start_pc
const START_PC: u64 = 0;

pub struct Mrom {
    memory: Vec<u8>,
}

impl Mrom {
    pub fn new() -> Self {
        let mut memory: Vec<u32> = vec![0; 64];
        //auipc  t0, 0x0
        memory[0] = 0x00000297;
        // addi  a1, t0, &dtb
        memory[1] = 0;
        // csrrw  a0, mhartid
        memory[2] = 0xf1401573;
        // ld  t0, 24(t0)
        memory[3] = 0;
        //jalr x0, 0(t0)
        memory[4] = 0x00028067;
        //.data
        // .dword START_PC
        memory[5] = START_PC as u32;
        memory[6] = (START_PC >> 32) as u32;
        Self {
            memory: memory.into_iter().flat_map(|b| b.to_ne_bytes()).collect(),
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
}
