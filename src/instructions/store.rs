use crate::{
    components::{mmu::Size, trap::Exception},
    cpu::Cpu,
    instructions::types::SType,
};

pub const SB: u8 = 0x0;
pub const SH: u8 = 0x1;
pub const SW: u8 = 0x2;
pub const SD: u8 = 0x3;

pub fn handle_store(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let stype = SType::new_with_raw_value(instr);
    let (funct3, rs1, rs2, imm) = (stype.funct3(), stype.rs1(), stype.rs2(), stype.imm());

    // Can compute the size direcly..
    // Ignore the possible values greater than SD, normally it should throw for an unknown instruction
    // but do we really need to?
    let size = 1 << (funct3.value() & 0x3);
    let size = Size::from_unchecked(size);

    let addr = cpu.x_regs.read(rs1).wrapping_add(imm.value() as u64);
    let value = cpu.x_regs.read(rs2);

    cpu.mmu.store(addr, value, size)
}
