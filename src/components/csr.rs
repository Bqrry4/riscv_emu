use arbitrary_int::*;
use bitbybit::bitfield;

use crate::cpu::PrivilegeMode;

const CSR_SIZE: usize = 1 << 12;

/* --Machine-level CSR-- */
// Machine Trap Setup
/// Machine status register.
pub const MSTATUS: usize = 0x300;
/// ISA and extensions.
const MISA: usize = 0x301;
//
// Machine information registers
/// Vendor ID.
const MVENDORID: usize = 0xf11;
/// Architecture ID.
const MARCHID: usize = 0xf12;
/// Implementation ID.
const MIMPID: usize = 0xf13;
/// Hardware thread ID.
const MHARTID: usize = 0xf14;
/// Pointer to configuration data structure.
const MCONFIGPTR: usize = 0xf15;
/// Machine exception program counter.
pub const MEPC: usize = 0x341;

/* --Supervisor-level CSR-- */
/// Supervisor address translation and protection.
pub const SAPT: usize = 0x180;
/// Supervisor exception program counter.
pub const SEPC: usize = 0x141;

#[bitfield(u64)]
pub struct MStatus {
    //State is Dirty
    #[bit(63, r)]
    sd: u1,
    ///M-mode-disable-trap,
    #[bit(42, r)]
    mdt: u1,
    #[bit(41, r)]
    mpelp: u1,
    #[bit(39, r)]
    mpv: u1,
    #[bit(38, r)]
    gva: u1,
    #[bit(37, r)]
    mbe: u1,
    #[bit(36, r)]
    sbe: u1,
    ///SXLEN
    #[bits(34..=35, r)]
    sxl: u2,
    ///UXLEN
    #[bits(32..=33, r)]
    uxl: u2,
    #[bit(24, r)]
    sdt: u1,
    #[bit(23, r)]
    spelp: u1,
    #[bit(22, r)]
    tsr: u1,
    #[bit(21, r)]
    tw: u1,
    #[bit(20, r)]
    tvm: u1,
    ///Make eXecutable Readable
    #[bit(19, r)]
    mxr: u1,
    ///permit Supervisor User Memory access
    #[bit(18, r)]
    sum: u1,
    ///Modify PRiVilege
    #[bit(17, rw)]
    mprv: u1,
    #[bits(15..=16, r)]
    xs: u2,
    #[bits(13..=14, r)]
    fs: u2,
    ///M Previous Privilege
    #[bits(11..=12, rw)]
    mpp: PrivilegeMode,
    #[bits(9..=10, r)]
    vs: u2,
    ///S Previous Privilege
    // Can contain the PrivilegeMode::User and PrivilegeMode::Supervisor,
    // which needs only 1 bit.
    #[bit(8, rw)]
    spp: u1,
    ///Previous M-mode Interrupt-Enable
    #[bit(7, rw)]
    mpie: u1,
    #[bit(6, r)]
    ube: u1,
    ///Previous S-mode Interrupt-Enable
    #[bit(5, rw)]
    spie: u1,
    ///M-mode Interrupt-Enable
    #[bit(3, rw)]
    mie: u1,
    ///S-mode Interrupt-Enable
    #[bit(1, rw)]
    sie: u1,
}

///A restricted view of mstatus
#[bitfield(u64)]
pub struct SStatus {
    #[bit(63, r)]
    sd: u1,
    #[bits(32..=33, r)]
    uxl: u2,
    #[bit(24, r)]
    sdt: u1,
    #[bit(23, r)]
    spelp: u1,
    #[bit(19, r)]
    mxr: u1,
    #[bit(18, r)]
    sum: u1,
    #[bits(15..=16, r)]
    xs: u2,
    #[bits(13..=14, r)]
    fs: u2,
    #[bits(9..=10, r)]
    vs: u2,
    #[bit(8, rw)]
    spp: u1,
    #[bit(6, r)]
    ube: u1,
    #[bit(5, rw)]
    spie: u1,
    /// S-mode Interrupt-Enable
    #[bit(1, rw)]
    sie: u1,
}

#[bitfield(u64)]
pub struct Sapt {
    #[bits(60..=63, r)]
    mode: u4,
    #[bits(44..=59, r)]
    asid: u16,
    #[bits(0..=43, r)]
    ppn: u44,
}

pub struct Csr {
    pub csrs: [u64; CSR_SIZE],
}

impl Csr {
    pub fn new() -> Self {
        let mut csrs = [0; CSR_SIZE];

        csrs[MISA] = (2 << 62) | //MXL[1:0]=2 (MXLEN=XLEN=64)
                    (1 << 12) | //Extensions[12]= M(Integer Multiply/Divide)
                    (1 << 8) | //Extensions[8] = I(RV32I/64I);
                    (1 << 0); //Extensions[0] = A(RV32A/64A);

        //non-commercial implementation
        csrs[MVENDORID] = 0;
        //no specific microarch
        csrs[MARCHID] = 0;
        //generic implementation (no versioning)
        csrs[MIMPID] = 0;
        //single core with id=0
        csrs[MHARTID] = 0;

        //3.1.6.3. Base ISA Control in mstatus
        //SXL and UXL are read-only field whose value always ensures that UXLEN=SXLEN=MXLEN=64.
        csrs[MSTATUS] = 2 << 34 | 2 << 32;

        Self { csrs }
    }

    pub fn read(&self, addr: usize) -> u64 {
        self.csrs[addr]
    }
    pub fn write(&mut self, addr: usize, val: u64) {
        self.csrs[addr] = val;
    }

    pub fn read_mstatus(&self) -> MStatus {
        MStatus::new_with_raw_value(self.csrs[MSTATUS])
    }

    pub fn write_mstatus(&mut self, value: &MStatus) {
        self.csrs[MSTATUS] = value.raw_value();
    }

    pub fn read_sstatus(&self) -> SStatus {
        SStatus::new_with_raw_value(self.csrs[MSTATUS])
    }

    pub fn write_sstatus(&mut self, value: &SStatus) {
        self.csrs[MSTATUS] = value.raw_value();
    }
}
