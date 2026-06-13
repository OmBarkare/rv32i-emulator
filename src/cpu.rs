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

    pub fn load_binary(&mut self, binary: &[u8]) {
        for (i, byte) in binary.iter().enumerate() {
            self.mem.write_8(i as u32, *byte).unwrap();
        }
    }

    pub fn dump_registers(&self) {
        for i in 0..32 {
            println!("x{:<2}: 0X{:0>8X}", i, self.regs[i]);
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
    }

    pub fn get_csr(&mut self, csr: u16) -> Result<&mut u32, ()> {
        match csr {
            0x300 => Ok(&mut self.csrs.mstatus),
            0x304 => Ok(&mut self.csrs.mie),
            0x341 => Ok(&mut self.csrs.mepc),
            0x342 => Ok(&mut self.csrs.mcause),
            0x305 => Ok(&mut self.csrs.mtvec),
            0x343 => Ok(&mut self.csrs.mtval),
            0x340 => Ok(&mut self.csrs.mscratch),
            0x344 => Ok(&mut self.csrs.mip),
            _ => Err(()),
        }
    }
}
