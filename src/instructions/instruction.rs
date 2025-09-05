use std::usize;

use super::load::handle_load;
use super::op::handle_op;
use super::op_imm::handle_op_imm;
use crate::cpu::Cpu;
use crate::instructions::store::handle_store;

/*
 * @Note for `allow(non_camel_case_types)` on enums, those are used as grouped const values, using the CamelCase feels wrong.
 */

/* The 2 LSBs are used for compressed instructions so we can limit the space */
pub const OPCODE_SIZE: usize = 1 << 7 >> 2;
pub const FUNCT3_SIZE: usize = 1 << 3;

pub type InstructionFn = fn(&mut Cpu, instr: u32);

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

#[repr(usize)]
#[allow(non_camel_case_types)]
enum OPCODE {
    LOAD = 0x03,
    MISC_MEM = 0x0f,
    OP_IMM = 0x13,
    AUIPC = 0x17,
    OP_IMMW = 0x1b,
    STORE = 0x23,
    AMO = 0x2f,
    OP = 0x33,
    OPW = 0x3b,
    LUI = 0x37,
    BRANCH = 0x63,
    JALR = 0x67,
    JAL = 0x6f,
    SYSTEM = 0x73,
}

/* There are really only 32 possible values for uncompressed instructions */
const OPCODE_LOOKUP_TABLE: [Option<InstructionFn>; OPCODE_SIZE] = {
    let mut table = [None; OPCODE_SIZE];

    const fn set_entry(
        table: &mut [Option<InstructionFn>; OPCODE_SIZE],
        opcode: OPCODE,
        handler: InstructionFn,
    ) {
        let index = (opcode as u8 >> 2) as usize;
        table[index] = Some(handler);
    }

    set_entry(&mut table, OPCODE::LUI, instr_lui);
    set_entry(&mut table, OPCODE::AUIPC, instr_auipc);

    set_entry(&mut table, OPCODE::OP, handle_op);
    set_entry(&mut table, OPCODE::OP_IMM, handle_op_imm);

    // set_entry(&mut table, OPCODE::OP_32, handle_op_32);

    set_entry(&mut table, OPCODE::LOAD, handle_load);
    set_entry(&mut table, OPCODE::STORE, handle_store);
    // set_entry(&mut table, OPCODE::MISC_MEM, handle_misc_mem);
    // set_entry(&mut table, OPCODE::OP_IMM_32, handle_op_imm_32);
    // set_entry(&mut table, OPCODE::AUIPC, handle_auipc);
    // set_entry(&mut table, OPCODE::AMO, handle_amo);
    // set_entry(&mut table, OPCODE::BRANCH, handle_branch);
    // set_entry(&mut table, OPCODE::JAL, handle_jal);
    // set_entry(&mut table, OPCODE::JALR, handle_jalr);
    // set_entry(&mut table, OPCODE::SYSTEM, handle_system);

    table
};

/**
   Returns a function that represent the decoded operation.
   It executes lazily, as the actual decoding is performed only after the call.
*/
pub fn decode(instr: u32) -> Option<InstructionFn> {
    return OPCODE_LOOKUP_TABLE[((instr >> 2) & 0x1f) as usize];
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
