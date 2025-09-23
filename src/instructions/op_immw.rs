use std::ops::{Shl, Shr};

use crate::{components::trap::Exception, cpu::Cpu, instructions::types::IType};
use arbitrary_int::i12;

const ADDIW: u8 = 0x0;
const SLLIW: u8 = 0x1;
const SRLIW_SRAIW: u8 = 0x5;

pub fn handle_op_immw(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let itype = IType::new_with_raw_value(instr);
    let (rd, funct3, rs1, imm) = (itype.rd(), itype.funct3(), itype.rs1(), itype.imm());

    let lhs = cpu.x_regs.read(rs1) as u32;
    let rhs = imm.value() as u32;

    let shamt = (rhs & 0x1f) as u8;
    let value = match funct3.value() {
        ADDIW => lhs.wrapping_add(rhs),
        SLLIW => lhs.shl(shamt),
        SRLIW_SRAIW => handle_srliw_sraiw(lhs, imm, shamt)?,
        _ => return Err(Exception::IllegalInstruction),
    };

    cpu.x_regs.write(rd, value as u64);
    Ok(())
}
fn handle_srliw_sraiw(lhs: u32, imm: i12, shamt: u8) -> Result<u32, Exception> {
    //discard the shamt and unneeded bits
    match imm.value() >> 10 {
        0x0 => Ok(lhs.shr(shamt)),
        0x1 => Ok((lhs as i32).shr(shamt) as u32),
        _ => return Err(Exception::IllegalInstruction),
    }
}
