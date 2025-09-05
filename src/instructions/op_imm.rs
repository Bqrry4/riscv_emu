use std::ops::{BitAnd, BitOr, BitXor};

use super::instruction::*;
use crate::{
    cpu::Cpu,
    instructions::op::{instr_sra, instr_srl},
};

type ITypeFn = fn(cpu: &mut Cpu, rd: u8, rs1: u8, imm: i16);

#[repr(u8)]
#[allow(non_camel_case_types)]
enum FUNCT3 {
    ADDI = 0x0,
    SLTI = 0x2,
    SLTIU = 0x3,
    XORI = 0x4,
    ORI = 0x6,
    ANDI = 0x7,
    SLLI = 0x1,
    SRLI_SRAI = 0x5,
}

const FUNCT3_LOOKUP_TABLE: [Option<ITypeFn>; FUNCT3_SIZE] = {
    let mut table: [Option<ITypeFn>; FUNCT3_SIZE] = [None; FUNCT3_SIZE];

    table[FUNCT3::ADDI as usize] = Some(instr_addi);
    table[FUNCT3::SLTI as usize] = Some(instr_slti);
    table[FUNCT3::SLTIU as usize] = Some(instr_sltiu);
    table[FUNCT3::XORI as usize] = Some(instr_xori);
    table[FUNCT3::ORI as usize] = Some(instr_ori);
    table[FUNCT3::ANDI as usize] = Some(instr_andi);
    table[FUNCT3::SLLI as usize] = Some(instr_slli);
    table[FUNCT3::SRLI_SRAI as usize] = Some(handle_srli_srai);

    table
};

pub fn handle_op_imm(cpu: &mut Cpu, instr: u32) {
    let (rd, funct3, rs1, imm) = i_type(instr);

    let instr_fn = FUNCT3_LOOKUP_TABLE[funct3 as usize].unwrap_or_else(|| {
        //should raise a cpu exception
        panic!("Instruction not supported");
    });

    instr_fn(cpu, rd, rs1, imm);
}

fn handle_srli_srai(cpu: &mut Cpu, rd: u8, rs1: u8, imm: i16) {
    let shamt = (imm & 0x3f) as u8;

    match imm >> 10 {
        //reuse functions from op
        0x0 => instr_srl(cpu, rd, rs1, shamt),
        0x1 => instr_sra(cpu, rd, rs1, shamt),
        _ => {}
    }
}

fn instr_addi(cpu: &mut Cpu, rd: u8, rs1: u8, imm: i16) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).wrapping_add(imm as u64));
}
fn instr_slti(cpu: &mut Cpu, rd: u8, rs1: u8, imm: i16) {
    cpu.x_regs
        .write(rd, ((cpu.x_regs.read(rs1) as i64) < (imm as i64)) as u64);
}
fn instr_sltiu(cpu: &mut Cpu, rd: u8, rs1: u8, imm: i16) {
    cpu.x_regs
        .write(rd, (cpu.x_regs.read(rs1) < (imm as u64)) as u64);
}
fn instr_xori(cpu: &mut Cpu, rd: u8, rs1: u8, imm: i16) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitxor(imm as u64));
}
fn instr_ori(cpu: &mut Cpu, rd: u8, rs1: u8, imm: i16) {
    cpu.x_regs.write(rd, cpu.x_regs.read(rs1).bitor(imm as u64));
}
fn instr_andi(cpu: &mut Cpu, rd: u8, rs1: u8, imm: i16) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitand(imm as u64));
}

fn instr_slli(cpu: &mut Cpu, rd: u8, rs1: u8, imm: i16) {
    let shamt = (imm & 0x3f) as u8;
    cpu.x_regs.write(rd, cpu.x_regs.read(rs1) << shamt);
}
