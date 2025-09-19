use arbitrary_int::{i13, u5};

use crate::{cpu::Cpu, instructions::types::BType};

const BEQ: u8 = 0x0;
const BNE: u8 = 0x1;
const BLT: u8 = 0x4;
const BGE: u8 = 0x5;
const BLTU: u8 = 0x6;
const BGEU: u8 = 0x7;

pub fn handle_branch(cpu: &mut Cpu, instr: u32) {
    let btype = BType::new_with_raw_value(instr);
    let (rs1, rs2, imm, funct3) = (btype.rs1(), btype.rs2(), btype.imm(), btype.funct3());

    match funct3.value() {
        BEQ => instr_beq(cpu, rs1, rs2, imm),
        BNE => instr_bne(cpu, rs1, rs2, imm),
        BLT => instr_blt(cpu, rs1, rs2, imm),
        BGE => instr_bge(cpu, rs1, rs2, imm),
        BLTU => instr_bltu(cpu, rs1, rs2, imm),
        BGEU => instr_bgeu(cpu, rs1, rs2, imm),
        _ => {}
    }
}

/* 2.5.2. Conditional Branches */
fn instr_beq(cpu: &mut Cpu, rs1: u5, rs2: u5, imm: i13) {
    if cpu.x_regs.read(rs1).ne(&cpu.x_regs.read(rs2)) {
        return;
    }
    cpu.pc = cpu.pc.wrapping_sub(4).wrapping_add(imm.value() as u64);
}
fn instr_bne(cpu: &mut Cpu, rs1: u5, rs2: u5, imm: i13) {
    if cpu.x_regs.read(rs1).eq(&cpu.x_regs.read(rs2)) {
        return;
    }
    cpu.pc = cpu.pc.wrapping_sub(4).wrapping_add(imm.value() as u64);
}
fn instr_blt(cpu: &mut Cpu, rs1: u5, rs2: u5, imm: i13) {
    if (cpu.x_regs.read(rs1) as i64).ge(&(cpu.x_regs.read(rs2) as i64)) {
        return;
    }
    cpu.pc = cpu.pc.wrapping_sub(4).wrapping_add(imm.value() as u64);
}
fn instr_bltu(cpu: &mut Cpu, rs1: u5, rs2: u5, imm: i13) {
    if cpu.x_regs.read(rs1).ge(&cpu.x_regs.read(rs2)) {
        return;
    }
    cpu.pc = cpu.pc.wrapping_sub(4).wrapping_add(imm.value() as u64);
}
fn instr_bge(cpu: &mut Cpu, rs1: u5, rs2: u5, imm: i13) {
    if (cpu.x_regs.read(rs1) as i64).lt(&(cpu.x_regs.read(rs2) as i64)) {
        return;
    }
    cpu.pc = cpu.pc.wrapping_sub(4).wrapping_add(imm.value() as u64);
}
fn instr_bgeu(cpu: &mut Cpu, rs1: u5, rs2: u5, imm: i13) {
    if cpu.x_regs.read(rs1).lt(&cpu.x_regs.read(rs2)) {
        return;
    }
    cpu.pc = cpu.pc.wrapping_sub(4).wrapping_add(imm.value() as u64);
}
