struct Cpu {
    regs: [u32; 32], 
    pc: u32,
}

struct Instruction {
    opcode: String,
    rd: u8,
    rs1: u8,
    rs2: u8,
    func3: u8,
    func7: u8,
    imm: u32,
}

struct RawInstruction {
    bits: u32,
}

impl Cpu {
    fn fetch(&self) -> RawInstruction {
        todo!()
    }

    fn decode(&self) -> Instruction {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
