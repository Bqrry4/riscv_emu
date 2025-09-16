use arbitrary_int::{u1, u2, u9, u12, u26, u39, u44, u56};
use bitbybit::bitfield;

use crate::{
    components::{
        csr::{MStatus, Sapt},
        system_bus::SystemBus,
        trap::Exception,
    },
    cpu::PrivilegeMode,
};

// (For Sv39, PAGESIZE=2^12 LEVELS=3 PTESIZE=8)
const PAGESIZE: u64 = 4096;
const LEVELS: u8 = 3;
///Page Table Entry size
const PTESIZE: u64 = 8;

#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Size {
    BYTE = 1,
    HWORD = 2,
    WORD = 4,
    DWORD = 8,
}

impl Size {
    #[inline(always)]
    pub fn from_unchecked(v: u8) -> Self {
        unsafe { core::mem::transmute::<u8, Self>(v) }
    }
}

#[derive(PartialEq, Eq)]
pub enum MemoryAccessType {
    ///X
    Instruction,
    ///R
    Load,
    ///W
    Store,
}
impl MemoryAccessType {
    ///Return coresponding page-fault exception
    pub fn page_fault(&self) -> Exception {
        match self {
            MemoryAccessType::Instruction => return Exception::InstructionPageFault,
            MemoryAccessType::Load => return Exception::LoadPageFault,
            MemoryAccessType::Store => return Exception::StorePageFault,
        }
    }
}

///Sv39 virtual address
#[bitfield(u39)]
pub struct Sv39va {
    ///Virtual Page Number
    #[bits(12..=20, rw)]
    vpn: [u9; 3],
    //Offset within the page
    #[bits(0..=11, r)]
    offset: u12,
}

///Sv39 physical address
#[bitfield(u56)]
pub struct Sv39pa {
    ///Physical Page Number[2]
    #[bits(30..=55, rw)]
    ppn2: u26,
    ///Physical Page Number[1]
    #[bits(21..=29, rw)]
    ppn1: u9,
    ///Physical Page Number[0]
    #[bits(12..=20, rw)]
    ppn0: u9,
    //Offset within the page
    #[bits(0..=11, rw)]
    offset: u12,
}

///Sv39 page table entry
#[bitfield(u64)]
pub struct Sv39pte {
    #[bit(63, r)]
    n: u1,
    #[bits(61..=62, r)]
    pbmt: u2,
    ///Physical Page Number[2]
    #[bits(28..=53, r)]
    ppn2: u26,
    ///Physical Page Number[1]
    #[bits(19..=27, r)]
    ppn1: u9,
    ///Physical Page Number[0]
    #[bits(10..=18, r)]
    ppn0: u9,
    ///Physical Page Number full view
    #[bits(10..=53, r)]
    ppn: u44,
    ///Reserved for Software
    #[bits(8..=9, r)]
    rsw: u2,
    ///Dirty
    #[bit(7, rw)]
    d: u1,
    ///Accessed
    #[bit(6, rw)]
    a: u1,
    #[bit(5, r)]
    g: u1,
    ///User, accessible in U-mode
    #[bit(4, r)]
    u: u1,
    ///Executable
    #[bit(3, r)]
    x: u1,
    ///Writable
    #[bit(2, r)]
    w: u1,
    ///Readable
    #[bit(1, r)]
    r: u1,
    ///Valid
    #[bit(0, r)]
    v: u1,
}

pub struct Mmu {
    pub memory: [u32; 64],
    pub bus: SystemBus,
    //Use raw pointers for now, as self-referencing is a pita
    mstatus: *const u64,
    sapt: *const u64,
    p_mode: *const PrivilegeMode,
}

impl Mmu {
    pub fn new(mstatus: *const u64, sapt: *const u64, p_mode: *const PrivilegeMode) -> Self {
        Self {
            //Fill this with NOPs, which is 0x13 on riscv
            memory: [0x13; 64],
            bus: SystemBus::new(),
            mstatus,
            sapt,
            p_mode,
        }
    }

    //TODO: caching of addresses?
    #[inline(always)]
    //12.3.2. Virtual Address Translation Process
    pub fn translate(&mut self, vaddr: u64, access: MemoryAccessType) -> Result<u64, Exception> {
        // 12.1.11. Supervisor Address Translation and Protection
        // The satp CSR is considered active when the effective privilege mode is S-mode or U-mode.
        // Executions of the address-translation algorithm may only begin using a given value of satp when satp is active.
        let p_mode = unsafe { *self.p_mode };
        if p_mode == PrivilegeMode::Machine {
            return Ok(vaddr);
        }

        let satp = unsafe { Sapt::new_with_raw_value(*self.sapt) };
        //3.1.6.4. Memory Privilege in mstatus Register
        let mstatus = unsafe { MStatus::new_with_raw_value(*self.mstatus) };
        //When MPRV=1, load and store memory addresses are translated and protected, and endianness is applied, as though the current privilege mode were set to MPP.
        let p_mode = if mstatus.mprv() == u1::new(1) && access != MemoryAccessType::Instruction {
            mstatus.mpp()
        } else {
            p_mode
        };

        // 1. Let a be satp.ppn×PAGESIZE, and let i=LEVELS-1.
        // The satp register must be active, i.e., the effective privilege mode must be S-mode or U-mode.
        let mut a: u64 = satp.ppn().value() * PAGESIZE;
        let mut i: i8 = (LEVELS - 1) as i8;
        let mut pte;
        let mut pte_address;
        let va = Sv39va::new_with_raw_value(u39::new(vaddr));
        loop {
            // 2. Let pte be the value of the PTE at address a+va.vpn[i]×PTESIZE.
            pte_address = a + (va.vpn(i as usize).value() as u64) * PTESIZE;
            // If accessing pte violates a PMA or PMP check, raise an access-fault exception corresponding to the original access type.
            let pte_value = self.bus.read(pte_address, Size::DWORD)?;
            pte = Sv39pte::new_with_raw_value(pte_value);
            // 3. If pte.v=0, or if pte.r=0 and pte.w=1,
            //? or if any bits or encodings that are reserved for future standard use are set within pte,
            if pte.v() == u1::new(0) || (pte.r() == u1::new(0) && pte.w() == u1::new(1)) {
                // stop and raise a page-fault exception corresponding to the original access type.
                return Err(access.page_fault());
            }
            // 4. Otherwise, the PTE is valid.
            // If pte.r=1 or pte.x=1, go to step 5.
            if pte.r() == u1::new(1) || pte.x() == u1::new(1) {
                break;
            };
            // Otherwise, this PTE is a pointer to the next level of the page table.
            // Let i=i-1.
            i = i - 1;
            // If i<0, stop and raise a page-fault exception corresponding to the original access type.
            if i < 0 {
                return Err(access.page_fault());
            }
            // Otherwise, let a=pte.ppn×PAGESIZE and go to step 2.
            a = pte.ppn().value() * PAGESIZE;
        }
        // 5. A leaf PTE has been reached.
        let mut ppn = [
            pte.ppn0().value() as u32,
            pte.ppn1().value() as u32,
            pte.ppn2().value() as u32,
        ];
        // If i>0 and pte.ppn[i-1:0] ≠ 0, this is a misaligned superpage;
        if i > 0 {
            for j in (i - 1)..=0 {
                if ppn[j as usize] == 0 {
                    continue;
                }
                // stop and raise a page-fault exception corresponding to the original access type.
                return Err(access.page_fault());
            }
        }
        // 6. Determine if the requested memory access is allowed by the pte.u bit,
        // given the current privilege mode and the value of the SUM and MXR fields of the mstatus register.
        match p_mode {
            PrivilegeMode::User => {
                if pte.u() == u1::new(0) {
                    // If not, stop and raise a page-fault exception corresponding to the original access type.
                    return Err(access.page_fault());
                }
            }
            PrivilegeMode::Supervisor => {
                if pte.u() == u1::new(1) {
                    //When SUM=1, load and store access are permitted for S-mode on U pages.
                    if mstatus.sum() == u1::new(0) || access == MemoryAccessType::Instruction {
                        return Err(access.page_fault());
                    }
                }
            }
            _ => {}
        }
        // 7. Determine if the requested memory access is allowed by the pte.r, pte.w, and pte.x bits, given the Shadow Stack Memory Protection rules.
        //  If not, stop and raise an access-fault exception.
        /* ---Skip Zicfiss extension for now. */

        // 8. Determine if the requested memory access is allowed by the pte.r, pte.w, and pte.x bits.
        //  If not, stop and raise a page-fault exception corresponding to the original access type.
        match access {
            MemoryAccessType::Load => {
                if pte.r() == u1::new(0)
                    // MXR check from step 6.
                    // When MXR=1, allow load on X pages.
                    && (mstatus.mxr() == u1::new(0) || pte.x() == u1::new(0))
                {
                    return Err(Exception::LoadPageFault);
                }
            }
            MemoryAccessType::Store => {
                if pte.w() == u1::new(0) {
                    return Err(Exception::StorePageFault);
                }
            }
            MemoryAccessType::Instruction => {
                if pte.x() == u1::new(0) {
                    return Err(Exception::InstructionPageFault);
                }
            }
        };

        // 9. If pte.a=0, or if the original memory access is a store and pte.d=0:
        if pte.a() == u1::new(0) || (access == MemoryAccessType::Store && pte.d() == u1::new(0)) {
            // If the Svade extension is implemented, stop and raise a page-fault exception corresponding to the original access type.
            //? ---Not yet.

            /*  ---This part is skipped as this implementation uses only 1 hart. */
            // Perform the following steps atomically:
            // *Compare pte to the value of the PTE at address a+va.vpn[i]×PTESIZE.
            // *If the comparison fails, return to step 2.
            //
            // *If the values match, set pte.a to 1 and, if the original memory access is a store, also set pte.d to 1.
            pte.set_a(u1::new(1));
            if access == MemoryAccessType::Store && pte.d() == u1::new(0) {
                pte.set_d(u1::new(1));
            }
            // If a store to pte would violate a PMA or PMP check, raise an access-fault exception corresponding to the original access type.
            self.bus.write(pte_address, Size::DWORD, pte.raw_value())?;
        }

        // 10. The translation is successful. The translated physical address is given as follows:
        let mut pa = Sv39pa::ZERO;
        // pa.pgoff = va.pgoff.
        pa.set_offset(va.offset());
        // If i>0, then this is a superpage translation and pa.ppn[i-1:0] = va.vpn[i-1:0].
        // pa.ppn[LEVELS-1:i] = pte.ppn[LEVELS-1:i].
        if i > 0 {
            //modify pte.ppn
            for j in (i - 1)..=0 {
                ppn[j as usize] = va.vpn(j as usize).value() as u32;
            }
        }
        pa.set_ppn0(u9::new(ppn[0] as u16));
        pa.set_ppn1(u9::new(ppn[1] as u16));
        pa.set_ppn2(u26::new(ppn[2] as u32));

        Ok(pa.raw_value)
    }

    pub fn load(&mut self, vaddr: u64, size: Size) -> Result<u64, Exception> {
        let paddr = self.translate(vaddr, MemoryAccessType::Load)?;
        let value = self.bus.read(paddr, size)?;
        Ok(value)
    }

    pub fn store(&mut self, vaddr: u64, value: u64, size: Size) -> Result<(), Exception> {
        let paddr = self.translate(vaddr, MemoryAccessType::Load)?;
        self.bus.write(paddr, size, value)?;
        Ok(())
    }
}
