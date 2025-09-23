use std::ops::BitAnd;

use super::load::handle_load;
use super::op::handle_op;
use super::op_imm::handle_op_imm;
use crate::components::trap::Exception;
use crate::cpu::Cpu;
use crate::instructions::amo::handle_amo;
use crate::instructions::branch::handle_branch;
use crate::instructions::op_immw::handle_op_immw;
use crate::instructions::opw::handle_opw;
use crate::instructions::store::handle_store;
use crate::instructions::system::handle_system;
use crate::instructions::types::{IType, JType, UType};

//Opcodes, remove the last 2 bits for C extension
const LOAD: u8 = 0x03 >> 2;
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

pub fn decode_and_execute(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let opcode = ((instr >> 2) & 0x1f) as u8;
    match opcode {
        LOAD => handle_load(cpu, instr)?,
        MISC_MEM => {
            // Zifencei not supported,
            // treated as a NO-OP as this implementation is cache-less
        }
        OP_IMM => handle_op_imm(cpu, instr)?,
        OP_IMMW => handle_op_immw(cpu, instr)?,
        STORE => handle_store(cpu, instr)?,
        AMO => handle_amo(cpu, instr)?,
        OP => handle_op(cpu, instr)?,
        OPW => handle_opw(cpu, instr)?,
        LUI => instr_lui(cpu, instr),
        AUIPC => instr_auipc(cpu, instr),
        BRANCH => handle_branch(cpu, instr),
        JALR => instr_jalr(cpu, instr),
        JAL => instr_jal(cpu, instr),
        SYSTEM => handle_system(cpu, instr)?,
        _ => return Err(Exception::IllegalInstruction),
    }
    Ok(())
}

/* Single class instructions */
fn instr_lui(cpu: &mut Cpu, instr: u32) {
    let utype = UType::new_with_raw_value(instr);
    let (rd, imm) = (utype.rd(), utype.imm());

    cpu.x_regs.write(rd, imm as u64);
}

fn instr_auipc(cpu: &mut Cpu, instr: u32) {
    let utype = UType::new_with_raw_value(instr);
    let (rd, imm) = (utype.rd(), utype.imm());

    cpu.x_regs.write(rd, cpu.pc + (imm as u64));
}

/* 2.5.1. Unconditional Jumps */
fn instr_jal(cpu: &mut Cpu, instr: u32) {
    let jtype = JType::new_with_raw_value(instr);
    let (rd, imm) = (jtype.rd(), jtype.imm());
    //the current pc is supposed to be of the next instruction
    cpu.x_regs.write(rd, cpu.pc);

    cpu.pc = cpu
        .pc
        //the address of the jump is 4 bytes behind
        .wrapping_sub(4)
        .wrapping_add(imm.value() as u64);
}
fn instr_jalr(cpu: &mut Cpu, instr: u32) {
    let itype = IType::new_with_raw_value(instr);
    let (rd, rs1, imm) = (itype.rd(), itype.rs1(), itype.imm());

    cpu.x_regs.write(rd, cpu.pc);
    cpu.pc = cpu
        .x_regs
        .read(rs1)
        //the address of the jump is 4 bytes behind
        .wrapping_sub(4)
        .wrapping_add(imm.value() as u64)
        //clear the lsb
        .bitand(!1);
}
