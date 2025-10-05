use arbitrary_int::u5;

#[derive(Debug)]
pub struct XRegisters {
    xregs: [u64; 32],
}

impl XRegisters {
    pub const zero: u5 = u5::new(0);
    pub const ra: u5 = u5::new(1);
    pub const sp: u5 = u5::new(2);
    pub const gp: u5 = u5::new(3);
    pub const tp: u5 = u5::new(4);
    pub const t0: u5 = u5::new(5);
    pub const t1: u5 = u5::new(6);
    pub const t2: u5 = u5::new(7);
    pub const s0: u5 = u5::new(8);
    pub const s1: u5 = u5::new(9);
    pub const a0: u5 = u5::new(10);
    pub const a1: u5 = u5::new(11);
    pub const a2: u5 = u5::new(12);
    pub const a3: u5 = u5::new(13);
    pub const a4: u5 = u5::new(14);
    pub const a5: u5 = u5::new(15);
    pub const a6: u5 = u5::new(16);
    pub const a7: u5 = u5::new(17);
    pub const s2: u5 = u5::new(18);
    pub const s3: u5 = u5::new(19);
    pub const s4: u5 = u5::new(20);
    pub const s5: u5 = u5::new(21);
    pub const s6: u5 = u5::new(22);
    pub const s7: u5 = u5::new(23);
    pub const s8: u5 = u5::new(24);
    pub const s9: u5 = u5::new(25);
    pub const s10: u5 = u5::new(26);
    pub const s11: u5 = u5::new(27);
    pub const t3: u5 = u5::new(28);
    pub const t4: u5 = u5::new(29);
    pub const t5: u5 = u5::new(30);
    pub const t6: u5 = u5::new(31);

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
