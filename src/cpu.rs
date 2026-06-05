use crate::memory::Memory;
pub struct Csrs {
    pub mtvec: u32,    // trap vector base address
    pub mepc: u32,     // machine exception PC
    pub mcause: u32,   // machine exception cause
    pub mtval: u32,    // machine trap value
    pub mstatus: u32,  // machine status register
    pub mie: u32,      // machine interrupt enable
    pub mip: u32,      // machine interrupt pending
    pub mscratch: u32, // temparary stratch word
}

impl Csrs {
    fn init() -> Self {
        Csrs {
            mtvec: 0,
            mepc: 0,
            mcause: 0,
            mtval: 0,
            mstatus: 0,
            mie: 0,
            mip: 0,
            mscratch: 0,
        }
    }
}
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
}

