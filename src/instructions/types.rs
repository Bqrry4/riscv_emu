use arbitrary_int::{i12, i13, i21, u1, u3, u5, u7};
use bitbybit::bitfield;

/* -Instruction types- */
#[bitfield(u32)]
pub struct RType {
    #[bits(25..=31, r)]
    funct7: u7,
    #[bits(20..=24, r)]
    rs2: u5,
    #[bits(15..=19, r)]
    rs1: u5,
    #[bits(12..=14, r)]
    funct3: u3,
    #[bits(7..=11, r)]
    rd: u5,
}

#[bitfield(u32)]
pub struct IType {
    #[bits(20..=31, r)]
    imm: i12,
    #[bits(15..=19, r)]
    rs1: u5,
    #[bits(12..=14, r)]
    funct3: u3,
    #[bits(7..=11, r)]
    rd: u5,
}

#[bitfield(u32)]
pub struct SType {
    #[bits(20..=24, r)]
    rs2: u5,
    #[bits(15..=19, r)]
    rs1: u5,
    #[bits(12..=14, r)]
    funct3: u3,
    #[bits([7..=11, 25..=31], r)]
    imm: i12,
}

#[bitfield(u32)]
pub struct BType {
    #[bits(20..=24, r)]
    rs2: u5,
    #[bits(15..=19, r)]
    rs1: u5,
    #[bits(12..=14, r)]
    funct3: u3,
}
impl BType {
    pub fn imm(&self) -> i13 {
        let raw = self.raw_value;
        let imm = (
            // imm[4:1]|0
            (raw >> 7 & 0x1e)
            // imm[10:5]
            | (raw >> 20 & 0x7e0)
            // imm[11]
            | (raw << 4 & 0x800)
            // imm[12] sign
            | (raw as i32 >> 19 & 0xf000) as u32
        ) as i16;
        //manually constructed
        unsafe { i13::new_unchecked(imm) }
    }
}

#[bitfield(u32)]
pub struct UType {
    #[bits(7..=11, r)]
    rd: u5,
}
impl UType {
    pub fn imm(&self) -> i32 {
        (self.raw_value & 0xfffff000) as i32
    }
}

#[bitfield(u32)]
pub struct JType {
    #[bits(7..=11, r)]
    rd: u5,
}
impl JType {
    pub fn imm(&self) -> i21 {
        let raw = self.raw_value;
        let imm = (
            // imm[10:1]|0
            (raw >> 20 & 0x7fe)
            // imm[11]
            | (raw >> 9 & 0x800)
            // imm[19:12]
            | (raw & 0xff000)
            // imm[20] sign
            | ((raw as i32 >> 11) as u32 & 0xfff00000) as u32
        ) as i32;
        //manually constructed
        unsafe { i21::new_unchecked(imm) }
    }
}

///The RType instruction specialized for A extension
#[bitfield(u32)]
pub struct ARType {
    #[bits(27..=31, r)]
    funct5: u5,
    #[bit(26, r)]
    aq: u1,
    #[bit(25, r)]
    rl: u1,
    #[bits(20..=24, r)]
    rs2: u5,
    #[bits(15..=19, r)]
    rs1: u5,
    #[bits(12..=14, r)]
    funct3: u3,
    #[bits(7..=11, r)]
    rd: u5,
}
