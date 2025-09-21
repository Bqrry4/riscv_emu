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

    let lhs = cpu.x_regs.read(rs1);
    let rhs = cpu.x_regs.read(rs2);

    /* 2.5.2. Conditional Branches */
    let take_branch = match funct3.value() {
        BEQ => lhs.eq(&rhs),
        BNE => lhs.ne(&rhs),
        BLT => (lhs as i64).lt(&(rhs as i64)),
        BGE => (lhs as i64).ge(&(rhs as i64)),
        BLTU => lhs.lt(&rhs),
        BGEU => lhs.ge(&rhs),
        _ => false,
    };
    if take_branch {
        cpu.pc = cpu.pc.wrapping_sub(4).wrapping_add(imm.value() as u64);
    }
}
