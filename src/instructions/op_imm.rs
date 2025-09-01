use super::instruction::*;
use crate::cpu::CPU;

type ITypeFn = fn(cpu: &mut CPU, rd: &u8, rs1: &u8, imm: &i16);

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
    table[FUNCT3::SLTI as usize] = None;
    table[FUNCT3::SLTIU as usize] = None;
    table[FUNCT3::XORI as usize] = None;
    table[FUNCT3::ORI as usize] = None;
    table[FUNCT3::ANDI as usize] = None;
    table[FUNCT3::SLLI as usize] = None;
    table[FUNCT3::SRLI_SRAI as usize] = None;

    table
};

pub fn handle_op_imm(cpu: &mut CPU, instr: u32) {
    let (rd, funct3, rs1, imm) = i_type(instr);

    let instr_fn = FUNCT3_LOOKUP_TABLE[funct3 as usize].unwrap_or_else(|| {
        //should raise a cpu exception
        panic!("Instruction not supported");
    });

    instr_fn(cpu, &rd, &rs1, &imm);
}

fn instr_addi(cpu: &mut CPU, rd: &u8, rs1: &u8, imm: &i16) {
    cpu.x_regs
        .write(*rd, cpu.x_regs.read(*rs1).wrapping_add(*imm as u64));
}
