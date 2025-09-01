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

type RTypeFn = fn(cpu: &mut CPU, rd: &u8, rs1: &u8, rs2: &u8, funct7: &u8);

const FUNCT3_LOOKUP_TABLE: [Option<RTypeFn>; FUNCT3_SIZE] = {
    let mut table: [Option<RTypeFn>; FUNCT3_SIZE] = [None; FUNCT3_SIZE];

    table[FUNCT3::ADD_SUB as usize] = Some(handle_add_sub);
    table[FUNCT3::SRL_SRA as usize] = None;
    table[FUNCT3::SLL as usize] = None;
    table[FUNCT3::SLT as usize] = None;
    table[FUNCT3::SLTU as usize] = None;
    table[FUNCT3::XOR as usize] = None;
    table[FUNCT3::OR as usize] = None;
    table[FUNCT3::AND as usize] = None;

    table
};

pub fn handle_op(cpu: &mut CPU, instr: u32) {
    let (rd, funct3, rs1, rs2, funct7) = r_type(instr);

    let instr_fn = FUNCT3_LOOKUP_TABLE[funct3 as usize].unwrap_or_else(|| {
        //should raise a cpu exception
        panic!("Instruction not supported");
    });

    instr_fn(cpu, &rd, &rs1, &rs2, &funct7);
}

fn handle_add_sub(cpu: &mut CPU, rd: &u8, rs1: &u8, rs2: &u8, funct7: &u8) {
    match funct7 {
        0x0 => instr_add(cpu, rd, rs1, rs2),
        0x20 => instr_sub(cpu, rd, rs1, rs2),
        _ => {}
    }
}

fn instr_add(cpu: &mut CPU, rd: &u8, rs1: &u8, rs2: &u8) {
    cpu.x_regs[*rd as usize] = cpu.x_regs[*rs1 as usize] + cpu.x_regs[*rs2 as usize];
}
fn instr_sub(cpu: &mut CPU, rd: &u8, rs1: &u8, rs2: &u8) {
    cpu.x_regs[*rd as usize] = cpu.x_regs[*rs1 as usize] - cpu.x_regs[*rs2 as usize];
}
