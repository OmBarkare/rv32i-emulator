use crate::memory::Memory;
pub struct Cpu {
    pub regs: [u32; 32],
    pub pc: u32, // stores address of instruction to be fetched, which will be executed next
    pub mem: Memory,
    pub curr_pc: u32, // stores address of instruction being executed
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: [0u32; 32],
            pc: 0u32,
            mem: Memory::new(),
            curr_pc: 0u32,
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