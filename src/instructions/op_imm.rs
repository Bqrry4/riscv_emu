use std::ops::{BitAnd, BitOr, BitXor};

use super::instruction::*;
use crate::{
    components::trap::Exception,
    cpu::Cpu,
    instructions::op::{instr_sra, instr_srl},
};

const ADDI: u8 = 0x0;
const SLTI: u8 = 0x2;
const SLTIU: u8 = 0x3;
const XORI: u8 = 0x4;
const ORI: u8 = 0x6;
const ANDI: u8 = 0x7;
const SLLI: u8 = 0x1;
const SRLI_SRAI: u8 = 0x5;

pub fn handle_op_imm(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let (rd, funct3, rs1, imm) = i_type(instr);

    match funct3 {
        ADDI => instr_addi(cpu, rd, rs1, imm),
        SLTI => instr_slti(cpu, rd, rs1, imm),
        SLTIU => instr_sltiu(cpu, rd, rs1, imm),
        XORI => instr_xori(cpu, rd, rs1, imm),
        ORI => instr_ori(cpu, rd, rs1, imm),
        ANDI => instr_andi(cpu, rd, rs1, imm),
        SLLI => instr_slli(cpu, rd, rs1, imm),
        SRLI_SRAI => handle_srli_srai(cpu, rd, rs1, imm),
        _ => return Err(Exception::IllegalInstruction),
    }
    Ok(())
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
