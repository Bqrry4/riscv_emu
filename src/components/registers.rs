use arbitrary_int::u5;

#[derive(Debug)]
pub struct XRegisters {
    xregs: [u64; 32],
}

impl XRegisters {
    pub fn new() -> Self {
        Self { xregs: [0; 32] }
    }

    #[inline]
    pub fn read(&self, id: u5) -> u64 {
        self.xregs[id.value() as usize]
    }

    #[inline]
    pub fn write(&mut self, id: u5, value: u64) {
        let id = id.value();
        /* @Note: x0 is hardwired to 0 and can't be written  */
        if id == 0 {
            return;
        };
        self.xregs[id as usize] = value;
    }
}
