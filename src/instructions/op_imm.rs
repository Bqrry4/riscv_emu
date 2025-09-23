use crate::{components::trap::Exception, cpu::Cpu, instructions::types::IType};
use arbitrary_int::i12;
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

const ADDI: u8 = 0x0;
const SLTI: u8 = 0x2;
const SLTIU: u8 = 0x3;
const XORI: u8 = 0x4;
const ORI: u8 = 0x6;
const ANDI: u8 = 0x7;
const SLLI: u8 = 0x1;
const SRLI_SRAI: u8 = 0x5;

pub fn handle_op_imm(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let itype = IType::new_with_raw_value(instr);
    let (rd, funct3, rs1, imm) = (itype.rd(), itype.funct3(), itype.rs1(), itype.imm());

    let lhs = cpu.x_regs.read(rs1);
    let rhs = imm.value() as u64;

    let shamt = (rhs & 0x3f) as u8;
    let value = match funct3.value() {
        ADDI => lhs.wrapping_add(rhs),
        SLTI => ((lhs as i64).lt(&(rhs as i64))) as u64,
        SLTIU => lhs.lt(&rhs) as u64,
        XORI => lhs.bitxor(rhs),
        ORI => lhs.bitor(rhs),
        ANDI => lhs.bitand(rhs),
        SLLI => lhs.shl(shamt),
        SRLI_SRAI => handle_srli_srai(lhs, imm, shamt)?,
        _ => return Err(Exception::IllegalInstruction),
    };
    cpu.x_regs.write(rd, value);
    Ok(())
}

fn handle_srli_srai(lhs: u64, imm: i12, shamt: u8) -> Result<u64, Exception> {
    //discard the shamt and unneeded bits
    match imm.value() >> 10 {
        0x0 => Ok(lhs.shr(shamt)),
        0x1 => Ok((lhs as i64).shr(shamt) as u64),
        _ => return Err(Exception::IllegalInstruction),
    }
}
