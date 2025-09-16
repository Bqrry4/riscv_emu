use super::instruction::*;
use crate::{components::{mmu::Size, trap::Exception}, cpu::Cpu};

pub const LB: u8 = 0x0;
pub const LH: u8 = 0x1;
pub const LW: u8 = 0x2;
pub const LD: u8 = 0x3;
//Interesting pattern of the bit[2], which tells if the sign extension is performed
pub const LBU: u8 = 0x4;
pub const LHU: u8 = 0x5;
pub const LWU: u8 = 0x6;

pub fn handle_load(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let (rd, funct3, rs1, imm) = i_type(instr);

    let size = 1 << (funct3 & 0x3);
    let addr = cpu.x_regs.read(rs1).wrapping_add(imm as u64);
    let val = cpu.mmu.load(
        addr,
        //(>ᴗ•)
        Size::from_unchecked(size),
    )?;

    //Sign extend by cast
    let value = match funct3 {
        LB => val as i8 as u64,
        LH => val as i16 as u64,
        LW => val as i32 as u64,
        //@Note: the 0x7 case is ignored and treated like a normal unsign value,
        //like a potential LDU that doesn't exist..
        _ => val,
    };
    cpu.x_regs.write(rd, value);
    Ok(())
}
