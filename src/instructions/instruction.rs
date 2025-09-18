use super::load::handle_load;
use super::op::handle_op;
use super::op_imm::handle_op_imm;
use crate::components::trap::Exception;
use crate::cpu::Cpu;
use crate::instructions::store::handle_store;
use crate::instructions::system::handle_system;
use crate::instructions::types::UType;

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
    let utype = UType::new_with_raw_value(instr);
    let (rd, imm) = (utype.rd(), utype.imm());

    cpu.x_regs.write(rd, imm as u64);
}

fn instr_auipc(cpu: &mut Cpu, instr: u32) {
    let utype = UType::new_with_raw_value(instr);
    let (rd, imm) = (utype.rd(), utype.imm());

    cpu.x_regs.write(rd, cpu.pc + (imm as u64));
}
