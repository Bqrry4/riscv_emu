#[derive(Debug)]
pub struct XRegisters {
    xregs: [u64; 32],
}

impl XRegisters {
    pub fn new() -> Self {
        Self { xregs: [0; 32] }
    }

    #[inline]
    pub fn read(&self, id: u8) -> u64 {
        self.xregs[id as usize]
    }
    pub fn write(&mut self, id: u8, value: u64) {
        /* @Note: x0 is hardwired to 0 and can't be written  */
        if id != 0 {
            self.xregs[id as usize] = value;
        }
    }
}
