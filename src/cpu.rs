pub struct Cpu {
    pub regs: [u32; 32],
    pub pc: u32,
    pub mem: Vec<u8>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            regs: [0u32; 32],
            pc: 0u32,
            mem: Vec::new(),
        }
    }

    pub fn load_binary(&mut self, binary: &[u8]) {
        for (i, byte) in binary.iter().enumerate() {
            self.mem[i] = *byte;
        }
    }
}