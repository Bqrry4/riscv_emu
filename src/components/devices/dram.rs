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
}
