use std::ops::{BitAnd, BitOr, BitXor};

use super::instruction::*;
use crate::cpu::CPU;

#[repr(u8)]
#[allow(non_camel_case_types)]
enum FUNCT3 {
    ADD_SUB = 0x0,
    SLL = 0x1,
    SLT = 0x2,
    SLTU = 0x3,
    XOR = 0x4,
    SRL_SRA = 0x5,
    OR = 0x6,
    AND = 0x7,
}

type RTypeFn = fn(cpu: &mut CPU, rd: u8, rs1: u8, rs2: u8, funct7: u8);

const FUNCT3_LOOKUP_TABLE: [Option<RTypeFn>; FUNCT3_SIZE] = {
    let mut table: [Option<RTypeFn>; FUNCT3_SIZE] = [None; FUNCT3_SIZE];

    table[FUNCT3::ADD_SUB as usize] = Some(handle_add_sub);
    table[FUNCT3::SRL_SRA as usize] = Some(handle_srl_sra);
    table[FUNCT3::SLL as usize] = Some(instr_sll);
    table[FUNCT3::SLT as usize] = Some(instr_slt);
    table[FUNCT3::SLTU as usize] = Some(instr_sltu);
    table[FUNCT3::XOR as usize] = Some(instr_xor);
    table[FUNCT3::OR as usize] = Some(instr_or);
    table[FUNCT3::AND as usize] = Some(instr_and);

    table
};

pub fn handle_op(cpu: &mut CPU, instr: u32) {
    let (rd, funct3, rs1, rs2, funct7) = r_type(instr);

    let instr_fn = FUNCT3_LOOKUP_TABLE[funct3 as usize].unwrap_or_else(|| {
        //should raise a cpu exception
        panic!("Instruction not supported");
    });

    instr_fn(cpu, rd, rs1, rs2, funct7);
}

fn handle_add_sub(cpu: &mut CPU, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    match funct7 {
        0x0 => instr_add(cpu, rd, rs1, rs2),
        0x20 => instr_sub(cpu, rd, rs1, rs2),
        _ => {}
    }
}

fn handle_srl_sra(cpu: &mut CPU, rd: u8, rs1: u8, rs2: u8, funct7: u8) {
    let shamt = (cpu.x_regs.read(rs2) & 0x3f) as u8;

    match funct7 {
        0x0 => instr_srl(cpu, rd, rs1, shamt),
        0x20 => instr_sra(cpu, rd, rs1, shamt),
        _ => {}
    }
}
#[inline(always)]
fn instr_add(cpu: &mut CPU, rd: u8, rs1: u8, rs2: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).wrapping_add(cpu.x_regs.read(rs2)));
}
#[inline(always)]
fn instr_sub(cpu: &mut CPU, rd: u8, rs1: u8, rs2: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).wrapping_sub(cpu.x_regs.read(rs2)));
}
#[inline(always)]
pub fn instr_srl(cpu: &mut CPU, rd: u8, rs1: u8, shamt: u8) {
    cpu.x_regs.write(rd, cpu.x_regs.read(rs1) >> shamt);
}
#[inline(always)]
pub fn instr_sra(cpu: &mut CPU, rd: u8, rs1: u8, shamt: u8) {
    cpu.x_regs
        .write(rd, ((cpu.x_regs.read(rs1) as i64) >> shamt) as u64);
}
fn instr_sll(cpu: &mut CPU, rd: u8, rs1: u8, rs2: u8, _: u8) {
    let shamt = (cpu.x_regs.read(rs2) & 0x3f) as u8;
    cpu.x_regs.write(rd, cpu.x_regs.read(rs1) << shamt);
}
fn instr_slt(cpu: &mut CPU, rd: u8, rs1: u8, rs2: u8, _: u8) {
    cpu.x_regs.write(
        rd,
        ((cpu.x_regs.read(rs1) as i64) < (cpu.x_regs.read(rs2) as i64)) as u64,
    );
}
fn instr_sltu(cpu: &mut CPU, rd: u8, rs1: u8, rs2: u8, _: u8) {
    cpu.x_regs
        .write(rd, (cpu.x_regs.read(rs1) < cpu.x_regs.read(rs2)) as u64);
}
fn instr_xor(cpu: &mut CPU, rd: u8, rs1: u8, rs2: u8, _: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitxor(cpu.x_regs.read(rs2)));
}
fn instr_or(cpu: &mut CPU, rd: u8, rs1: u8, rs2: u8, _: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitor(cpu.x_regs.read(rs2)));
}
fn instr_and(cpu: &mut CPU, rd: u8, rs1: u8, rs2: u8, _: u8) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitand(cpu.x_regs.read(rs2)));
}
