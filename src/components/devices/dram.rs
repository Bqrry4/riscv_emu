pub const DRAM_SIZE: u64 = 512 * 1024;

pub struct Dram {
    memory: [u64; DRAM_SIZE as usize],
}

impl Dram {
    pub fn new() -> Self {
        Self {
            memory: [0; DRAM_SIZE as usize],
        }
    }
}
