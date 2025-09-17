use std::ops::{BitAnd, BitOr, BitXor};

use super::instruction::*;
use crate::{components::trap::Exception, cpu::Cpu};

//I+M extenions
const ADD_SUB_MUL: u8 = 0x0;
const SLL_MULH: u8 = 0x1;
const SLT_MULHSU: u8 = 0x2;
const SLTU_MULHU: u8 = 0x3;
const XOR_DIV: u8 = 0x4;
const SRL_SRA_DIVU: u8 = 0x5;
const OR_REM: u8 = 0x6;
const AND_REMU: u8 = 0x7;

pub fn handle_op(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let (rd, funct3, rs1, rs2, funct7) = r_type(instr);

    match funct3 {
        ADD_SUB_MUL => handle_add_sub_mul(cpu, rd, rs1, rs2, funct7),
        SLL_MULH => handle_sll_mulh(cpu, rd, rs1, rs2, funct7),
        SLT_MULHSU => handle_slt_mulhsu(cpu, rd, rs1, rs2, funct7),
        SLTU_MULHU => handle_sltu_mulhu(cpu, rd, rs1, rs2, funct7),
        XOR_DIV => handle_xor_div(cpu, rd, rs1, rs2, funct7),
        SRL_SRA_DIVU => handle_srl_sra_divu(cpu, rd, rs1, rs2, funct7),
        OR_REM => handle_or_rem(cpu, rd, rs1, rs2, funct7),
        AND_REMU => handle_and_remu(cpu, rd, rs1, rs2, funct7),
        _ => return Err(Exception::IllegalInstruction),
    }
    Ok(())
}

fn handle_add_sub_mul(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    match funct7 {
        0x0 => instr_add(cpu, rd, rs1, rs2),
        0x20 => instr_sub(cpu, rd, rs1, rs2),
        0x01 => instr_mul(cpu, rd, rs1, rs2),
        _ => {}
    }
}
fn handle_srl_sra_divu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    let shamt = (cpu.x_regs.read(rs2) & 0x3f) as u8;

    match funct7 {
        0x0 => instr_srl(cpu, rd, rs1, shamt),
        0x20 => instr_sra(cpu, rd, rs1, shamt),
        0x1 => instr_divu(cpu, rd, rs1, rs2),
        _ => {}
    }
}
fn handle_sll_mulh(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    match funct7 {
        0x0 => instr_sll(cpu, rd, rs1, rs2),
        0x1 => instr_mulh(cpu, rd, rs1, rs2),
        _ => {}
    }
}
fn handle_slt_mulhsu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    match funct7 {
        0x0 => instr_slt(cpu, rd, rs1, rs2),
        0x1 => instr_mulhsu(cpu, rd, rs1, rs2),
        _ => {}
    }
}
fn handle_sltu_mulhu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    match funct7 {
        0x0 => instr_sltu(cpu, rd, rs1, rs2),
        0x1 => instr_mulhu(cpu, rd, rs1, rs2),
        _ => {}
    }
}
fn handle_xor_div(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    match funct7 {
        0x0 => instr_xor(cpu, rd, rs1, rs2),
        0x1 => instr_div(cpu, rd, rs1, rs2),
        _ => {}
    }
}
fn handle_or_rem(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    match funct7 {
        0x0 => instr_or(cpu, rd, rs1, rs2),
        0x1 => instr_rem(cpu, rd, rs1, rs2),
        _ => {}
    }
}
fn handle_and_remu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    match funct7 {
        0x0 => instr_and(cpu, rd, rs1, rs2),
        0x1 => instr_remu(cpu, rd, rs1, rs2),
        _ => {}
    }
}

//-RV32I-
#[inline(always)]
fn instr_add(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).wrapping_add(cpu.x_regs.read(rs2)));
}
#[inline(always)]
fn instr_sub(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).wrapping_sub(cpu.x_regs.read(rs2)));
}
#[inline(always)]
pub fn instr_srl(cpu: &mut Cpu, rd: u8, rs1: u8, shamt: u8) {
    cpu.x_regs.write(rd, cpu.x_regs.read(rs1) >> shamt);
}
#[inline(always)]
pub fn instr_sra(cpu: &mut Cpu, rd: u8, rs1: u8, shamt: u8) {
    cpu.x_regs
        .write(rd, ((cpu.x_regs.read(rs1) as i64) >> shamt) as u64);
}
#[inline(always)]
fn instr_sll(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    let shamt = (cpu.x_regs.read(rs2) & 0x3f) as u8;
    cpu.x_regs.write(rd, cpu.x_regs.read(rs1) << shamt);
}
#[inline(always)]
fn instr_slt(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    cpu.x_regs.write(
        rd,
        ((cpu.x_regs.read(rs1) as i64) < (cpu.x_regs.read(rs2) as i64)) as u64,
    );
}
#[inline(always)]
fn instr_sltu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    cpu.x_regs
        .write(rd, (cpu.x_regs.read(rs1) < cpu.x_regs.read(rs2)) as u64);
}
#[inline(always)]
fn instr_xor(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitxor(cpu.x_regs.read(rs2)));
}
#[inline(always)]
fn instr_or(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitor(cpu.x_regs.read(rs2)));
}
#[inline(always)]
fn instr_and(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitand(cpu.x_regs.read(rs2)));
}
//-RV32M-
#[inline(always)]
fn instr_mul(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    cpu.x_regs.write(
        rd,
        (cpu.x_regs.read(rs1) as i64).wrapping_mul(cpu.x_regs.read(rs2) as i64) as u64,
    );
}
#[inline(always)]
fn instr_mulh(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    //signed×signed
    cpu.x_regs.write(
        rd,
        ((cpu.x_regs.read(rs1) as i128).wrapping_mul(cpu.x_regs.read(rs2) as i128) >> 64) as u64,
    );
}
#[inline(always)]
fn instr_mulhsu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    //signed×unsigned
    cpu.x_regs.write(
        rd,
        ((cpu.x_regs.read(rs1) as i128 as u128).wrapping_mul(cpu.x_regs.read(rs2) as u128) >> 64)
            as u64,
    );
}
#[inline(always)]
fn instr_mulhu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    //unsigned×unsigned
    cpu.x_regs.write(
        rd,
        ((cpu.x_regs.read(rs1) as u128).wrapping_mul(cpu.x_regs.read(rs2) as u128) >> 64) as u64,
    );
}

//@ Note: on M extension, the divizion by zero and overflow doesn't raise exceptions,
// it writes default values instead.
#[inline(always)]
fn instr_div(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    let divident = cpu.x_regs.read(rs1);
    let divisor = cpu.x_regs.read(rs2);

    cpu.x_regs.write(
        rd,
        if divisor == 0 {
            -1i8 as u64
        } else {
            (divident as i64).wrapping_div(divisor as i64) as u64
        },
    );
}
#[inline(always)]
fn instr_divu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    let divident = cpu.x_regs.read(rs1);
    let divisor = cpu.x_regs.read(rs2);

    cpu.x_regs.write(
        rd,
        if divisor == 0 {
            -1i8 as u64
        } else {
            divident.wrapping_div(divisor)
        },
    );
}
#[inline(always)]
fn instr_rem(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    let divident = cpu.x_regs.read(rs1);
    let divisor = cpu.x_regs.read(rs2);

    cpu.x_regs.write(
        rd,
        if divisor == 0 {
            divident
        } else {
            (divident as i64).wrapping_rem(divisor as i64) as u64
        },
    );
}
#[inline(always)]
fn instr_remu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8) {
    let divident = cpu.x_regs.read(rs1);
    let divisor = cpu.x_regs.read(rs2);

    cpu.x_regs.write(
        rd,
        if divisor == 0 {
            divident
        } else {
            divident.wrapping_rem(divisor)
        },
    );
}
