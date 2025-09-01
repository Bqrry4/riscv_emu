use super::instruction::*;
use crate::cpu::CPU;

#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum FUNCT3 {
    /* RV32I */
    LB = 0x0,
    LH = 0x1,
    LW = 0x2,
    LBU = 0x4,
    LHU = 0x5,
    /* RV64I */
    LWU = 0x6,
    LD = 0x3,
}

const FUNCT3_LOOKUP_TABLE: [Option<InstructionFn>; FUNCT3_SIZE] = {
    let mut table: [Option<InstructionFn>; FUNCT3_SIZE] = [None; FUNCT3_SIZE];

    // table[FUNCT3::LB as usize] = Some(instr_lb);
    // table[FUNCT3::LH as usize] = Some(instr_lh);
    // table[FUNCT3::LW as usize] = Some(instr_lw);
    // table[FUNCT3::LBU as usize] = Some(instr_lbu);
    // table[FUNCT3::LHU as usize] = Some(instr_lhu);
    // table[FUNCT3::LWU as usize] = Some(instr_lwu);
    // table[FUNCT3::LD as usize] = Some(instr_ld);

    table
};
pub fn handle_load(cpu: &mut CPU, instr: u32) {
    let instr_fn = FUNCT3_LOOKUP_TABLE[((instr >> 2) & 0x4f) as usize];
}

// fn instr_lb(cpu: &mut CPU) {}
// fn instr_lh(cpu: &mut CPU) {}
// fn instr_lw(cpu: &mut CPU) {}
// fn instr_lbu(cpu: &mut CPU) {}
// fn instr_lhu(cpu: &mut CPU) {}
// fn instr_lwu(cpu: &mut CPU) {}
// fn instr_ld(cpu: &mut CPU) {}
