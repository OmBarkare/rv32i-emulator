use std::fs;

use crate::csrs::Csrs;
use crate::memory::Memory;

pub struct Cpu {
    pub regs: [u32; 32],
    pub pc: u32, // stores address of instruction to be fetched, which will be executed next
    pub mem: Memory,
    pub curr_pc: u32, // stores address of instruction being executed
    pub csrs: Csrs,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: [0u32; 32],
            pc: 0u32,
            mem: Memory::new(),
            curr_pc: 0u32,
            csrs: Csrs::init(),
        }
    }

    pub fn load_binary(&mut self, binary: &[u8], start_addr: u32) {
        for (i, byte) in binary.iter().enumerate() {
            self.mem.write_8(i as u32 + start_addr, *byte).unwrap();
        }
    }

    pub fn load_elf(&mut self, elf: &[u8]) -> Result<(), String> {
        if &elf[0..4] != b"\x7fELF" {
            return Err(String::from("not an elf"));
        }

        // 4th is 1 for 32 bit address space
        if elf[4] != 1 {
            return Err(String::from("elf not for 32 bit address space"));
        }

        // 5th bit is 1 for little endian encoding
        if elf[5] != 1 {
            return Err(String::from("encoding not little endian"));
        }
        // 6th, 7th and 8th byte represent version,
        // OS ABI, ABI version (almost never used). Not
        // checking these because not super important
        // next 7 bytes after that is padding

        // e_machine
        let e_machine: u16 = ((elf[19] as u16) << 8) | elf[18] as u16;
        if e_machine != 0xF3 {
            return Err(String::from("not for RISCV"));
        }

        // e_entry
        let e_entry: u32 = (elf[27] as u32) << 24
            | (elf[26] as u32) << 16
            | (elf[25] as u32) << 8
            | elf[24] as u32;
        self.pc = e_entry;

        let e_phoff: u32 = (elf[31] as u32) << 24
            | (elf[30] as u32) << 16
            | (elf[29] as u32) << 8
            | elf[28] as u32;

        let e_phentsize: u16 = (elf[43] as u16) << 8 | (elf[42] as u16);

        let e_phnum: u16 = (elf[45] as u16) << 8 | (elf[44] as u16);

        // reading the program headers and
        // loading segments
        for i in 0..e_phnum {
            let start_addr: usize = (e_phoff + (e_phentsize as u32) * (i as u32)) as usize;
            if elf[start_addr] != 1 {
                continue;
            }

            let offset = (elf[start_addr + 7] as u32) << 24
                | (elf[start_addr + 6] as u32) << 16
                | (elf[start_addr + 5] as u32) << 8
                | elf[start_addr + 4] as u32;

            let vaddr = (elf[start_addr + 11] as u32) << 24
                | (elf[start_addr + 10] as u32) << 16
                | (elf[start_addr + 9] as u32) << 8
                | elf[start_addr + 8] as u32;

            let filesz = (elf[start_addr + 19] as u32) << 24
                | (elf[start_addr + 18] as u32) << 16
                | (elf[start_addr + 17] as u32) << 8
                | elf[start_addr + 16] as u32;
            // all memory is already initialized to zero
            // so no need to read memsize

            let bytes = &elf[(offset as usize)..((offset + filesz) as usize)];
            self.load_binary(bytes, vaddr);
        }
        Ok(())
    }

    pub fn dump_registers(&self) {
        for i in 0..32 {
            log::debug!("x{:<2}: 0X{:0>8X}", i, self.regs[i]);
        }
    }

    // TODO
    // save state to csr, update PC to mvec
    pub fn trap(&mut self, cause: u32, tval: u32) {
        self.csrs.mcause = cause;
        self.csrs.mepc = self.curr_pc; // mepc saves address of trapping instruction
        self.pc = self.csrs.mtvec & !0x3;
        self.csrs.write_mstatus_mpv(false); // false is 0
        self.csrs.write_mstatus_mpp(3);
        let mie = self.csrs.read_mstatus_mie();
        self.csrs.write_mstatus_mpie(mie);
        self.csrs.write_mstatus_mie(false);
        self.csrs.mtval = tval;
        log::debug!("{:#?}", self.csrs);
    }
}
