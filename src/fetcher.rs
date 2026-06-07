use crate::cpu::Cpu;
use crate::instructions::RawInstruction;

impl Cpu {
    pub fn fetch(&mut self) -> RawInstruction {
        self.curr_pc = self.pc;
        self.pc += 4;
        RawInstruction {
            bits: self.mem.read_32(self.curr_pc).unwrap(),
        }
    }
}

