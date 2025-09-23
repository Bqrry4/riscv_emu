use std::ops::{Shl, Shr};

use crate::{components::trap::Exception, cpu::Cpu, instructions::types::RType};

//I extenion
const ADDW: (u8, u8) = (0x0, 0x0);
const SUBW: (u8, u8) = (0x0, 0x20);
const SLLW: (u8, u8) = (0x1, 0x0);
const SRLW: (u8, u8) = (0x5, 0x0);
const SRAW: (u8, u8) = (0x5, 0x20);
//M extenion
const MULW: (u8, u8) = (0x0, 0x1);
const DIVW: (u8, u8) = (0x4, 0x1);
const DIVUW: (u8, u8) = (0x5, 0x1);
const REMW: (u8, u8) = (0x6, 0x1);
const REMUW: (u8, u8) = (0x7, 0x1);

pub fn handle_opw(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let rtype = RType::new_with_raw_value(instr);
    let (rd, funct3, rs1, rs2, funct7) = (
        rtype.rd(),
        rtype.funct3(),
        rtype.rs1(),
        rtype.rs2(),
        rtype.funct7(),
    );

    let lhs = cpu.x_regs.read(rs1) as u32;
    let rhs = cpu.x_regs.read(rs2) as u32;

    let shamt = (rhs & 0x3f) as u8;
    let value = match (funct3.value(), funct7.value()) {
        //-RV64I-
        ADDW => lhs.wrapping_add(rhs),
        SUBW => lhs.wrapping_sub(rhs),
        SLLW => lhs.shl(shamt),
        SRLW => lhs.shr(shamt),
        SRAW => (lhs as i32).shr(shamt) as u32,
        //-RV64M-
        MULW => (lhs as i32).wrapping_mul(rhs as i32) as u32,
        DIVW => {
            if rhs == 0 {
                -1i8 as u32
            } else {
                (lhs as i32).wrapping_div(rhs as i32) as u32
            }
        }
        DIVUW => {
            if rhs == 0 {
                -1i8 as u32
            } else {
                lhs.wrapping_div(rhs)
            }
        }
        REMW => {
            if rhs == 0 {
                lhs
            } else {
                (lhs as i32).wrapping_rem(rhs as i32) as u32
            }
        }
        REMUW => {
            if rhs == 0 {
                lhs
            } else {
                lhs.wrapping_rem(rhs)
            }
        }
        _ => return Err(Exception::IllegalInstruction),
    };

    cpu.x_regs.write(rd, value as u64);
    Ok(())
}
