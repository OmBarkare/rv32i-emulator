use crate::cpu::Cpu;
use crate::instructions::RawInstruction;

impl Cpu {
    pub fn fetch(&mut self) -> RawInstruction {
        let mut inst: u32 = 0;
        let mut curr_byte: u32 = 0;
        for i in 0..4 {
            curr_byte = (self.mem[self.pc as usize + i] as u32) << 8*i;
            inst |= curr_byte;
        }
        self.curr_pc = self.pc;
        self.pc += 4;
        RawInstruction { bits:inst }
    }
}