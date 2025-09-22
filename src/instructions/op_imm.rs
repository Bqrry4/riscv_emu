use std::ops::{BitAnd, BitOr, BitXor};

use arbitrary_int::{i12, u5, u6};

use crate::{
    components::trap::Exception,
    cpu::Cpu,
    instructions::{
        op::{instr_sra, instr_srl},
        types::IType,
    },
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
    let itype = IType::new_with_raw_value(instr);
    let (rd, funct3, rs1, imm) = (itype.rd(), itype.funct3(), itype.rs1(), itype.imm());

    match funct3.value() {
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

fn handle_srli_srai(cpu: &mut Cpu, rd: u5, rs1: u5, imm: i12) {
    let shamt = unsafe { u6::new_unchecked((imm.value() & 0x3f) as u8) };

    match imm.value() >> 10 {
        //reuse functions from op
        0x0 => instr_srl(cpu, rd, rs1, shamt),
        0x1 => instr_sra(cpu, rd, rs1, shamt),
        _ => {}
    }
}

fn instr_addi(cpu: &mut Cpu, rd: u5, rs1: u5, imm: i12) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).wrapping_add(imm.value() as u64));
}
fn instr_slti(cpu: &mut Cpu, rd: u5, rs1: u5, imm: i12) {
    cpu.x_regs.write(
        rd,
        ((cpu.x_regs.read(rs1) as i64) < (imm.value() as i64)) as u64,
    );
}
fn instr_sltiu(cpu: &mut Cpu, rd: u5, rs1: u5, imm: i12) {
    cpu.x_regs
        .write(rd, (cpu.x_regs.read(rs1) < (imm.value() as u64)) as u64);
}
fn instr_xori(cpu: &mut Cpu, rd: u5, rs1: u5, imm: i12) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitxor(imm.value() as u64));
}
fn instr_ori(cpu: &mut Cpu, rd: u5, rs1: u5, imm: i12) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitor(imm.value() as u64));
}
fn instr_andi(cpu: &mut Cpu, rd: u5, rs1: u5, imm: i12) {
    cpu.x_regs
        .write(rd, cpu.x_regs.read(rs1).bitand(imm.value() as u64));
}

fn instr_slli(cpu: &mut Cpu, rd: u5, rs1: u5, imm: i12) {
    let shamt = (imm.value() & 0x3f) as u8;
    cpu.x_regs.write(rd, cpu.x_regs.read(rs1) << shamt);
}
