use crate::components::trap::Exception;

/* Internal memory map addresses */
// https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc#memory-map
const SOURCE_PRIORITY_BASE: u64 = 0x000000;
const SOURCE_PRIORITY_END: u64 = 0x000ffc;
const PENDING_BASE: u64 = 0x001000;
const PENDING_END: u64 = 0x00107c;
/* Enable bits region for contexts [0,1] */
const ENABLE_BASE: u64 = 0x002000;
const ENABLE_END: u64 = 0x0020fc;
/* Threshold & Claim regions for contexts [0,1] */
const THRESHOLD_CLAIM_BASE: u64 = 0x200000;
const THRESHOLD_CLAIM_END: u64 = 0x201004;

/* base addresses for a word addressable array */
const PENDING_W: usize = (PENDING_BASE >> 2) as usize;
const ENABLE_W: usize = (ENABLE_BASE >> 2) as usize;
const THRESHOLD_CLAIM_W: usize = (THRESHOLD_CLAIM_BASE >> 2) as usize;
/// Platform Level Interrupt Controller.
/// https://wiki.osdev.org/PLIC.
/// https://github.com/riscv/riscv-plic-spec/blob/master/riscv-plic.adoc
pub struct Plic {
    priority: [u32; 1024],
    pending: [u32; 32],
    enable: [u32; 64],
    threshold: [u32; 2],
    claim: [u32; 2],
}
impl Plic {
    pub fn new() -> Self {
        Self {
            priority: [0; 1024],
            pending: [0; 32],
            enable: [0; 64],
            threshold: [0; 2],
            claim: [0; 2],
        }
    }
    pub fn set_pending(&mut self, irq: u32, value: bool) {
        // div by 32 to find the word the irq belongs to
        let word = (irq >> 5) as usize;
        let bit = 1 << irq;
        // make the value expand to 0x0.. or 0xf..
        let value = -!(!(value as u32) as i32) as u32;
        // clear the bit, then set it to the value
        self.pending[word] = (self.pending[word] & !bit) | (value & bit);
    }

    pub fn read(&self, offset: u64) -> Result<u32, Exception> {
        // offset only applied to words
        let index = (offset >> 2) as usize;
        match offset {
            SOURCE_PRIORITY_BASE..=SOURCE_PRIORITY_END => Ok(self.priority[index]),
            PENDING_BASE..=PENDING_END => Ok(self.pending[index]),
            ENABLE_BASE..=ENABLE_END => Ok(self.enable[index]),
            THRESHOLD_CLAIM_BASE..=THRESHOLD_CLAIM_END => {
                let base = index - THRESHOLD_CLAIM_W;
                // Context occupies 4096 bytes, which is 2 + 10 shr
                let context = base >> 10;
                // Can be either first or second word
                let offset = base & 1;
                if offset == 0 {
                    Ok(self.threshold[context])
                } else {
                    Ok(self.claim[context])
                }
            }
            _ => Err(Exception::LoadAccessFault),
        }
    }

    pub fn write(&mut self, offset: u64, value: u32) -> Result<(), Exception> {
        // offset only applied to words
        let index = (offset >> 2) as usize;
        match offset {
            SOURCE_PRIORITY_BASE..=SOURCE_PRIORITY_END => {
                self.priority[index] = value;
            }
            PENDING_BASE..=PENDING_END => {
                self.pending[index - PENDING_W] = value;
            }
            ENABLE_BASE..=ENABLE_END => {
                self.enable[index - ENABLE_W] = value;
            }
            THRESHOLD_CLAIM_BASE..=THRESHOLD_CLAIM_END => {
                let base = index - THRESHOLD_CLAIM_W;
                // Context occupies 4096 bytes, which is 2 + 10 shr
                let context = base >> 10;
                // Can be either first or second word
                let offset = base & 1;
                if offset == 0 {
                    self.threshold[context] = value;
                } else {
                    self.claim[context] = value;
                }
            }
            _ => return Err(Exception::StoreAccessFault),
        }

        Ok(())
    }
}
