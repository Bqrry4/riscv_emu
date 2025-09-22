use std::ops::{BitAnd, BitOr, BitXor};

use arbitrary_int::{u3, u5};

use crate::{
    components::{mmu::Size, trap::Exception},
    cpu::Cpu,
    instructions::types::ARType,
};

const LR: u8 = 0x2;
const SC: u8 = 0x3;
const AMOSWAP: u8 = 0x1;
const AMOADD: u8 = 0x0;
const AMOXOR: u8 = 0x4;
const AMOAND: u8 = 0xC;
const AMOOR: u8 = 0x8;
const AMOMIN: u8 = 0x10;
const AMOMAX: u8 = 0x14;
const AMOMINU: u8 = 0x18;
const AMOMAXU: u8 = 0x1C;

const AMMO_W: u8 = 0x2;
const AMMO_D: u8 = 0x3;

//& Zalrsc and Zaamo extensions requires that the address held in rs1 be naturally aligned to the size of the operand.
//& If the address is not naturally aligned, an address-misaligned exception or an access-fault exception will be generated.
fn check_alignment(address: u64, size: Size) -> Result<(), Exception> {
    if address % (size as u64) != 0 {
        return Err(Exception::LoadAddressMisaligned);
    }
    Ok(())
}

fn amo_load_value(cpu: &mut Cpu, funct3: u3, address: u64) -> Result<(u64, Size), Exception> {
    match funct3.value() {
        AMMO_W => {
            const SIZE: Size = Size::WORD;
            check_alignment(address, SIZE)?;
            //sign extend
            Ok((cpu.mmu.load(address, SIZE)? as i32 as u64, SIZE))
        }
        AMMO_D => {
            const SIZE: Size = Size::DWORD;
            check_alignment(address, SIZE)?;
            Ok((cpu.mmu.load(address, SIZE)?, SIZE))
        }
        _ => return Err(Exception::IllegalInstruction),
    }
}
pub fn handle_amo(cpu: &mut Cpu, instr: u32) -> Result<(), Exception> {
    let rtype = ARType::new_with_raw_value(instr);
    //aq/rl ignored as this implementation executes sequentionally
    let (rd, rs1, rs2, funct3, funct5) = (
        rtype.rd(),
        rtype.rs1(),
        rtype.rs2(),
        rtype.funct3(),
        rtype.funct5(),
    );

    match funct5.value() {
        LR => instr_lr(cpu, rd, rs1, funct3)?,
        SC => instr_sc(cpu, rd, rs1, rs2, funct3)?,
        AMOSWAP => amo_op(cpu, rd, rs1, rs2, funct3, |_, rhs| rhs)?,
        AMOADD => amo_op(cpu, rd, rs1, rs2, funct3, |lhs, rhs| lhs.wrapping_add(rhs))?,
        AMOXOR => amo_op(cpu, rd, rs1, rs2, funct3, |lhs, rhs| lhs.bitxor(rhs))?,
        AMOAND => amo_op(cpu, rd, rs1, rs2, funct3, |lhs, rhs| lhs.bitand(rhs))?,
        AMOOR => amo_op(cpu, rd, rs1, rs2, funct3, |lhs, rhs| lhs.bitor(rhs))?,
        AMOMIN => amo_op(cpu, rd, rs1, rs2, funct3, |lhs, rhs| {
            (lhs as i64).min(rhs as i64) as u64
        })?,
        AMOMAX => amo_op(cpu, rd, rs1, rs2, funct3, |lhs, rhs| {
            (lhs as i64).max(rhs as i64) as u64
        })?,
        AMOMINU => amo_op(cpu, rd, rs1, rs2, funct3, |lhs, rhs| lhs.min(rhs))?,
        AMOMAXU => amo_op(cpu, rd, rs1, rs2, funct3, |lhs, rhs| lhs.max(rhs))?,
        _ => return Err(Exception::IllegalInstruction),
    }

    Ok(())
}

fn instr_lr(cpu: &mut Cpu, rd: u5, rs1: u5, funct3: u3) -> Result<(), Exception> {
    //Load the data value from the address in rs1
    let address = cpu.x_regs.read(rs1);
    let (value, _) = amo_load_value(cpu, funct3, address)?;

    cpu.reservation = Some(address);
    cpu.x_regs.write(rd, value);

    Ok(())
}

fn instr_sc(cpu: &mut Cpu, rd: u5, rs1: u5, rs2: u5, funct3: u3) -> Result<(), Exception> {
    let address = cpu.x_regs.read(rs1);

    let size = match funct3.value() {
        AMMO_W => Size::WORD,
        AMMO_D => Size::DWORD,
        _ => return Err(Exception::IllegalInstruction),
    };
    check_alignment(address, size)?;

    let success = match cpu.reservation.is_some_and(|raddr| raddr == address) {
        true => {
            cpu.mmu.store(address, cpu.x_regs.read(rs2), size)?;
            1
        }
        false => 0,
    };

    cpu.x_regs.write(rd, success);
    //& Regardless of success or failure, executing an SC.W instruction invalidates any reservation held by this hart.
    cpu.reservation = None;
    Ok(())
}

fn amo_op<F>(cpu: &mut Cpu, rd: u5, rs1: u5, rs2: u5, funct3: u3, op: F) -> Result<(), Exception>
where
    F: Fn(u64, u64) -> u64,
{
    let address = cpu.x_regs.read(rs1);
    let (value, size) = amo_load_value(cpu, funct3, address)?;

    let new_val = op(value, cpu.x_regs.read(rs2));
    cpu.mmu.store(address, new_val, size)?;
    cpu.x_regs.write(rd, value);

    Ok(())
}
