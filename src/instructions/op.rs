use crate::{components::trap::Exception, cpu::Cpu, instructions::types::RType};
use std::ops::{BitAnd, BitOr, BitXor, Shl, Shr};

//I extenion
const ADD: (u8, u8) = (0x0, 0x0);
const SUB: (u8, u8) = (0x0, 0x20);
const SLL: (u8, u8) = (0x1, 0x0);
const SLT: (u8, u8) = (0x2, 0x0);
const SLTU: (u8, u8) = (0x3, 0x0);
const XOR: (u8, u8) = (0x4, 0x0);
const SRL: (u8, u8) = (0x5, 0x0);
const SRA: (u8, u8) = (0x5, 0x20);
const OR: (u8, u8) = (0x6, 0x0);
const AND: (u8, u8) = (0x7, 0x0);
//M extenion
const MUL: (u8, u8) = (0x0, 0x1);
const MULH: (u8, u8) = (0x1, 0x1);
const MULHSU: (u8, u8) = (0x2, 0x1);
const MULHU: (u8, u8) = (0x3, 0x1);
const DIV: (u8, u8) = (0x4, 0x1);
const DIVU: (u8, u8) = (0x5, 0x1);
const REM: (u8, u8) = (0x6, 0x1);
const REMU: (u8, u8) = (0x7, 0x1);

#[inline(never)]
pub fn handle_op(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let rtype = RType::new_with_raw_value(instr);
    let (rd, funct3, rs1, rs2, funct7) = (
        rtype.rd(),
        rtype.funct3(),
        rtype.rs1(),
        rtype.rs2(),
        rtype.funct7(),
    );

    let lhs = cpu.x_regs.read(rs1);
    let rhs = cpu.x_regs.read(rs2);

    let shamt = (rhs & 0x3f) as u8;
    let value = match (funct3.value(), funct7.value()) {
        //-RV32I-
        ADD => lhs.wrapping_add(rhs),
        SUB => lhs.wrapping_sub(rhs),
        SLL => lhs.shl(shamt),
        SLT => (lhs as i64).lt(&(rhs as i64)) as u64,
        SLTU => lhs.lt(&rhs) as u64,
        XOR => lhs.bitxor(rhs),
        SRL => lhs.shr(shamt),
        SRA => (lhs as i64).shr(shamt) as u64,
        OR => lhs.bitor(rhs),
        AND => lhs.bitand(rhs),
        //-RV32M-
        MUL => (lhs as i64).wrapping_mul(rhs as i64) as u64,
        MULH => {
            //signed×signed
            ((lhs as i128).wrapping_mul(rhs as i128) >> 64) as u64
        }
        MULHSU => {
            //signed×unsigned
            ((lhs as i128 as u128).wrapping_mul(rhs as u128) >> 64) as u64
        }
        MULHU => {
            //unsigned×unsigned
            ((lhs as u128).wrapping_mul(rhs as u128) >> 64) as u64
        }
        //@ Note: on M extension, the divizion by zero and overflow doesn't raise exceptions,
        // it writes default values instead.
        DIV => {
            if rhs == 0 {
                -1i8 as u64
            } else {
                (lhs as i64).wrapping_div(rhs as i64) as u64
            }
        }
        DIVU => {
            if rhs == 0 {
                -1i8 as u64
            } else {
                lhs.wrapping_div(rhs)
            }
        }
        REM => {
            if rhs == 0 {
                lhs
            } else {
                (lhs as i64).wrapping_rem(rhs as i64) as u64
            }
        }
        REMU => {
            if rhs == 0 {
                lhs
            } else {
                lhs.wrapping_rem(rhs)
            }
        }
        _ => return Err(Exception::IllegalInstruction),
    };

    cpu.x_regs.write(rd, value);
    Ok(())
}
