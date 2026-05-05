use crate::cpu::Cpu;
use crate::instructions::RawInstruction;

// TODO: pc shuold update in fetch instead of execute
// Then we can also remove the akward (-4) everwhere in execute where 
// PC is updated in the branch instructions

impl Cpu {
    pub fn fetch(&mut self) -> RawInstruction {
        let mut inst: u32 = 0;
        let mut curr_byte: u32 = 0;
        for i in 0..4 {
            curr_byte = (self.mem[self.pc as usize + i] as u32) << 8*i;
            inst |= curr_byte;
        }
        self.pc += 4;
        RawInstruction { bits:inst }
    }
}