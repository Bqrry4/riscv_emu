use std::ops::{BitAnd, BitOr, BitXor};

use super::instruction::*;
use crate::cpu::Cpu;

#[repr(u8)]
#[allow(non_camel_case_types)]
enum FUNCT3 {
    ADD_SUB_MUL = 0x0,
    SLL_MULH = 0x1,
    SLT_MULHSU = 0x2,
    SLTU_MULHU = 0x3,
    XOR_DIV = 0x4,
    SRL_SRA_DIVU = 0x5,
    OR_REM = 0x6,
    AND_REMU = 0x7,
}

type RTypeFn = fn(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, funct7: u8);

const FUNCT3_LOOKUP_TABLE: [Option<RTypeFn>; FUNCT3_SIZE] = {
    let mut table: [Option<RTypeFn>; FUNCT3_SIZE] = [None; FUNCT3_SIZE];

    table[FUNCT3::ADD_SUB_MUL as usize] = Some(handle_add_sub);
    table[FUNCT3::SRL_SRA_DIVU as usize] = Some(handle_srl_sra);
    table[FUNCT3::SLL_MULH as usize] = Some(instr_sll);
    table[FUNCT3::SLT_MULHSU as usize] = Some(instr_slt);
    table[FUNCT3::SLTU_MULHU as usize] = Some(instr_sltu);
    table[FUNCT3::XOR_DIV as usize] = Some(instr_xor);
    table[FUNCT3::OR_REM as usize] = Some(instr_or);
    table[FUNCT3::AND_REMU as usize] = Some(instr_and);

    table
};

pub fn handle_op(cpu: &mut Cpu, instr: u32) {
    let (rd, funct3, rs1, rs2, funct7) = r_type(instr);

    let instr_fn = FUNCT3_LOOKUP_TABLE[funct3 as usize].unwrap_or_else(|| {
        //should raise a cpu exception
        panic!("Instruction not supported");
    });

    instr_fn(cpu, rd, rs1, rs2, funct7);
}

fn handle_add_sub(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    match funct7 {
        0x0 => instr_add(cpu, rd, rs1, rs2),
        0x20 => instr_sub(cpu, rd, rs1, rs2),
        _ => {}
    }
}

fn handle_srl_sra(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    let shamt = (cpu.x_regs.read(rs2) & 0x3f) as u8;

    match funct7 {
        0x0 => instr_srl(cpu, rd, rs1, shamt),
        0x20 => instr_sra(cpu, rd, rs1, shamt),
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
fn instr_sll(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
    let shamt = (cpu.x_regs.read(rs2) & 0x3f) as u8;
    cpu.x_regs.write(rd, cpu.x_regs.read(rs1) << shamt);
}
#[inline(always)]
fn instr_slt(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
    cpu.x_regs.write(
        rd,
        ((cpu.x_regs.read(rs1) as i64) < (cpu.x_regs.read(rs2) as i64)) as u64,
    );
}
#[inline(always)]
fn instr_sltu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
    cpu.x_regs
        .write(rd, (cpu.x_regs.read(rs1) < cpu.x_regs.read(rs2)) as u64);
}
#[inline(always)]
fn instr_xor(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitxor(cpu.x_regs.read(rs2)));
}
#[inline(always)]
fn instr_or(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitor(cpu.x_regs.read(rs2)));
}
#[inline(always)]
fn instr_and(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitand(cpu.x_regs.read(rs2)));
}
//-RV32M-
#[inline(always)]
fn instr_mul(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
    cpu.x_regs.write(
        rd,
        (cpu.x_regs.read(rs1) as i64).wrapping_mul(cpu.x_regs.read(rs2) as i64) as u64,
    );
}
#[inline(always)]
fn instr_mulh(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
    //signed×signed
    cpu.x_regs.write(
        rd,
        ((cpu.x_regs.read(rs1) as i128).wrapping_mul(cpu.x_regs.read(rs2) as i128) >> 64) as u64,
    );
}
#[inline(always)]
fn instr_mulhsu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
    //signed×unsigned
    cpu.x_regs.write(
        rd,
        ((cpu.x_regs.read(rs1) as i128 as u128).wrapping_mul(cpu.x_regs.read(rs2) as u128) >> 64)
            as u64,
    );
}
#[inline(always)]
fn instr_mulhu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
    //unsigned×unsigned
    cpu.x_regs.write(
        rd,
        ((cpu.x_regs.read(rs1) as u128).wrapping_mul(cpu.x_regs.read(rs2) as u128) >> 64) as u64,
    );
}

//@ Note: on M extension, the divizion by zero and overflow doesn't raise exceptions,
// it writes default values instead.
#[inline(always)]
fn instr_div(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
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
fn instr_divu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
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
fn instr_rem(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
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
fn instr_remu(cpu: &mut Cpu, rd: u8, rs1: u8, rs2: u8, _: u8) {
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
