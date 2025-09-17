use std::usize;

use super::load::handle_load;
use super::op::handle_op;
use super::op_imm::handle_op_imm;
use crate::components::trap::Exception;
use crate::cpu::Cpu;
use crate::instructions::store::handle_store;
use crate::instructions::system::handle_system;

//Opcodes, remove the last 2 bits for C extension
const LOAD: u8 = 0x03 >> 2;
//Zifencei not supported
const MISC_MEM: u8 = 0x0f >> 2;
const OP_IMM: u8 = 0x13 >> 2;
const AUIPC: u8 = 0x17 >> 2;
const OP_IMMW: u8 = 0x1b >> 2;
const STORE: u8 = 0x23 >> 2;
const AMO: u8 = 0x2f >> 2;
const OP: u8 = 0x33 >> 2;
const OPW: u8 = 0x3b >> 2;
const LUI: u8 = 0x37 >> 2;
const BRANCH: u8 = 0x63 >> 2;
const JALR: u8 = 0x67 >> 2;
const JAL: u8 = 0x6f >> 2;
const SYSTEM: u8 = 0x73 >> 2;

/* -Instruction types- */
pub const fn r_type(instr: u32) -> (u8, u8, u8, u8, u8) {
    let rd = ((instr >> 7) & 0x1f) as u8;
    let funct3 = ((instr >> 12) & 0x7) as u8;
    let rs1 = ((instr >> 15) & 0x1f) as u8;
    let rs2 = ((instr >> 20) & 0x1f) as u8;
    let funct7 = (instr >> 25) as u8;

    (rd, funct3, rs1, rs2, funct7)
}

pub const fn i_type(instr: u32) -> (u8, u8, u8, i16) {
    let rd = ((instr >> 7) & 0x1f) as u8;
    let funct3 = ((instr >> 12) & 0x7) as u8;
    let rs1 = ((instr >> 15) & 0x1f) as u8;
    //preserve the sign
    let imm = ((instr as i32) >> 20) as i16;

    (rd, funct3, rs1, imm)
}

pub const fn s_type(instr: u32) -> (u8, u8, u8, i16) {
    let funct3 = ((instr >> 12) & 0x7) as u8;
    let rs1 = ((instr >> 15) & 0x1f) as u8;
    let rs2 = ((instr >> 20) & 0x1f) as u8;
    //preserve the sign
    let imm = ((instr as i32 >> 20) & 0xfe0) as i16 | ((instr >> 7) & 0x1F) as i16;

    (funct3, rs1, rs2, imm)
}

pub const fn b_type(instr: u32) -> (u8, u8, u8, i16) {
    let funct3 = ((instr >> 12) & 0x07) as u8;
    let rs1 = ((instr >> 15) & 0x1f) as u8;
    let rs2 = ((instr >> 20) & 0x1f) as u8;
    let imm = (
        // imm[4:1]|0
        (instr >> 7 & 0x1e)
        // imm[10:5]
        | (instr >> 20 & 0x7e0)
        // imm[11]
        | (instr << 4 & 0x800)
        // imm[12] sign
        | (instr as i32 >> 19 & 0xf000) as u32
    ) as i16;

    (funct3, rs1, rs2, imm)
}

/* @Note: there's only 2 U-type instructions defined, both zeroes the lowest 12 bits */
pub const fn u_type(instr: u32) -> (u8, i32) {
    let rd = ((instr >> 7) & 0x1f) as u8;
    let imm = (instr & 0xfffff000) as i32;

    (rd, imm)
}

const fn j_type(_cpu: &mut Cpu, instr: u32) -> (u8, i32) {
    let rd = ((instr >> 7) & 0x1f) as u8;
    let imm = (
        // imm[10:1]|0
        (instr >> 20 & 0x7fe)
        // imm[11]
        | (instr >> 9 & 0x800)
        // imm[19:12]
        | (instr & 0xff000)
        // imm[20] sign
        | ((instr as i32 >> 11) as u32 & 0xfff00000) as u32
    ) as i32;

    (rd, imm)
}

pub fn decode_and_execute(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let opcode = ((instr >> 2) & 0x1f) as u8;
    match opcode {
        LOAD => handle_load(cpu, instr)?,
        OP_IMM => handle_op_imm(cpu, instr)?,
        OP_IMMW => {}
        STORE => handle_store(cpu, instr)?,
        AMO => {}
        OP => handle_op(cpu, instr)?,
        OPW => {}
        LUI => instr_lui(cpu, instr),
        AUIPC => instr_auipc(cpu, instr),
        BRANCH => {}
        JALR => {}
        JAL => {}
        SYSTEM => handle_system(cpu, instr),
        _ => return Err(Exception::IllegalInstruction),
    }
    Ok(())
}

/* Single class instructions */
fn instr_lui(cpu: &mut Cpu, instr: u32) {
    let (rd, imm) = u_type(instr);
    cpu.x_regs.write(rd, imm as u64);
}

fn instr_auipc(cpu: &mut Cpu, instr: u32) {
    let (rd, imm) = u_type(instr);
    cpu.x_regs.write(rd, cpu.pc + (imm as u64));
}
