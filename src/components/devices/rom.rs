use std::mem;

use crate::{
    components::{
        mmu::Size::{self, *},
        system_bus::DRAM_BASE,
        trap::Exception,
    },
    cpu::PrivilegeMode,
    util::any_as_u8_slice,
};

#[repr(C)]
struct FwDynamicInfo {
    ///Info magic
    magic: u64,
    ///Info version
    version: u64,
    ///Next booting stage address
    next_addr: u64,
    ///Next booting stage mode
    next_mode: u64,
    ///Options for OpenSBI library
    options: u64,
    ///Preferred boot HART id
    boot_hart: u64,
}

//TODO: Set a start_pc
const START_PC: u64 = DRAM_BASE;

pub struct Mrom {
    memory: Vec<u8>,
}

//0x8000_0000 - 0x8010_0000 - OPENSBI - 1MB

impl Mrom {
    pub fn new() -> Self {
        let mut firmware: Vec<u32> = vec![0; 7];
        //auipc  t0, 0x0
        firmware[0] = 0x00000297;
        // addi  a1, t0, &dtb(0)
        firmware[1] = 0x00028593;
        // csrrw  a0, mhartid
        firmware[2] = 0xf1401573;
        // ld  t0, 24(t0)
        firmware[3] = 0x0182b283;
        //jalr x0, 0(t0)h
        firmware[4] = 0x00028067;
        //.data
        // .dword START_PC
        firmware[5] = START_PC as u32;
        firmware[6] = (START_PC >> 32) as u32;

        let d_info = FwDynamicInfo {
            magic: 0x4942534f,
            version: 0x2,
            next_addr: 0,
            next_mode: PrivilegeMode::Supervisor as u64,
            options: 0,
            //We have only one hart
            boot_hart: 0,
        };

        let mut memory: Vec<u8> =
            Vec::with_capacity(firmware.len() * 4 + mem::size_of::<FwDynamicInfo>());

        firmware
            .into_iter()
            .for_each(|b| memory.extend_from_slice(&b.to_ne_bytes()));

        let d_info_bytes = unsafe { any_as_u8_slice(&d_info) };
        memory.extend_from_slice(d_info_bytes);
        Self { memory: memory }
    }

    pub fn read(&self, index: u64, size: Size) -> Result<u64, Exception> {
        let index = index as usize;
        let bytes = self
            .memory
            .get(index..index + (size as usize))
            .ok_or(Exception::LoadAccessFault)?;

        let data = match size {
            BYTE => bytes[0] as u64,
            HWORD => u16::from_le_bytes(bytes.try_into().unwrap()) as u64,
            WORD => u32::from_le_bytes(bytes.try_into().unwrap()) as u64,
            DWORD => u64::from_le_bytes(bytes.try_into().unwrap()),
        };

        Ok(data)
    }
}
