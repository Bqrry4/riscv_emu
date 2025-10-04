use crate::components::mmu::Size;

pub struct Test {
    pub exit: u64,
}

impl Test {
    const FINISHER_FAIL: u64 = 0x3333;
    const FINISHER_PASS: u64 = 0x5555;
    const FINISHER_RESET: u64 = 0x7777;

    pub fn new() -> Test {
        Self { exit: 0 }
    }

    pub fn write(&mut self, index: u64, _: Size, value: u64) {
        match index {
            0 => match value {
                Test::FINISHER_FAIL => {
                    self.exit = 1;
                }
                Test::FINISHER_PASS => {
                    self.exit = 2;
                }
                Test::FINISHER_RESET => {
                    self.exit = 3;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
