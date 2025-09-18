use arbitrary_int::{u1, u2};

use crate::{
    cpu::{Cpu, PrivilegeMode},
    instructions::{instruction::i_type, types::IType},
};

#[allow(dead_code)]
/// Read/Write CSR
pub const CSRRW: u8 = 0x1;
/// Read and Set Bits in CSR
pub const CSRRS: u8 = 0x2;
/// Read and Clear Bits in CSR
pub const CSRRC: u8 = 0x3;
/// Same as the above, except bit[2] tells the rsi is an imm value
pub const CSRRWI: u8 = 0x5;
pub const CSRRSI: u8 = 0x6;
pub const CSRRCI: u8 = 0x7;

pub const SRET: u16 = 0x102;
/// Used to return from a trap taken into M-mode
pub const MRET: u16 = 0x302;
pub const MNRET: u16 = 0x702;
pub const WFI: u16 = 0x105;

#[inline(never)]
//@Note: in theory those instructions should execute atomically
pub fn handle_system(cpu: &mut Cpu, instr: u32) {
    let itype = IType::new_with_raw_value(instr);
    let (rd, funct3, rsi, csr) = (itype.rd(), itype.funct3(), itype.rs1(), itype.imm());

    //get rid of the sign extended bytes
    let csr_addr = (csr.value() as u16) & 0xfff;

    //handle the funct3=0 subclass
    if funct3.value() == 0 {
        match csr_addr {
            //3.3.2. Trap-Return Instructions
            SRET => {
                //x=S
                let mut mstatus = cpu.csr.read_mstatus();

                //xIE is set to xPIE
                mstatus.set_sie(mstatus.spie());
                //the privilege mode is changed to xPP
                cpu.p_mode.set(PrivilegeMode::from_unchecked(u2::new(
                    mstatus.spp().value(),
                )));
                //xPIE is set to 1
                mstatus.set_spie(u1::new(1));
                //xPP is set to the least-privileged supported mode (U if U-mode is implemented, else M)
                mstatus.set_spp(u1::new(PrivilegeMode::User as u8));
                //If xPP!=M, xRET also sets MPRV=0
                if *cpu.p_mode != PrivilegeMode::Machine {
                    //SRET clears the mprv, so assume it uses mstatus instead of sstatus
                    mstatus.set_mprv(u1::new(0));
                }
                cpu.csr.write_mstatus(&mstatus);
            }
            MRET => {
                //x=M
                //3.1.6.1. Privilege and Global Interrupt-Enable Stack
                let mut mstatus = cpu.csr.read_mstatus();
                //xIE is set to xPIE
                mstatus.set_mie(mstatus.mpie());
                //the privilege mode is changed to xPP
                cpu.p_mode.set(mstatus.mpp());
                //xPIE is set to 1
                mstatus.set_mpie(u1::new(1));
                //xPP is set to the least-privileged supported mode (U if U-mode is implemented, else M)
                mstatus.set_mpp(PrivilegeMode::User);
                //If xPP!=M, xRET also sets MPRV=0
                if *cpu.p_mode != PrivilegeMode::Machine {
                    mstatus.set_mprv(u1::new(0));
                }

                cpu.csr.write_mstatus(&mstatus);
            }
            MNRET => {}
            WFI => {}
            _ => {}
        }
        return;
    }

    // check for bit[2]
    let rs_val = if (funct3.value() >> 2) != 0 {
        //rsi is an imm
        rsi.value() as u64
    } else {
        //rsi is a reg
        cpu.x_regs.read(rsi)
    };
    let csr_val = cpu.csr.read(csr_addr);

    // TODO: check for rsi==x0
    //handle Zicsr extension
    match funct3.value() & 0x3 {
        CSRRW => {
            //swap the values
            cpu.csr.write(csr_addr, rs_val);
        }
        CSRRS => {
            //reg_val act as a set bit mask
            cpu.csr.write(csr_addr, csr_val | rs_val);
        }
        CSRRC => {
            //reg_val act as a clear bit mask
            cpu.csr.write(csr_addr, csr_val & (!rs_val));
        }
        _ => {
            //throw unknown instruction
            // The hypervisor instructions would land here for now, as there isn't a planned support for it.
        }
    }
    cpu.x_regs.write(rd, csr_val);
}
