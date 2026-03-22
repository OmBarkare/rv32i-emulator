use std::default;

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
    pub fn fetch(&self) -> RawInstruction {
        todo!()
    }

    pub fn decode(&self, raw_inst: RawInstruction) -> Instruction {
        todo!()
    }

    fn get_opcode(raw_bits: u32) -> String {
    let opcode_bits: u8 = (raw_bits & 0x7F) as u8;
    let func3: u8 = ((raw_bits >> 12) & 0x07) as u8;
    let func7: u8 = ((raw_bits >> 25) & 0x7F) as u8;
    let imm: u16 = ((raw_bits >> 20) & 0x0FFF) as u16;

    match (opcode_bits, func3, func7, imm) {
        // U-type
        (0b0110111, _, _, _) => String::from("LUI"),
        (0b0010111, _, _, _) => String::from("AUIPC"),

        // J-type
        (0b1101111, _, _, _) => String::from("JAL"),

        // I-type
        (0b1100111, 0b000, _, _) => String::from("JALR"),

        // B-type
        (0b1100011, 0b000, _, _) => String::from("BEQ"),
        (0b1100011, 0b001, _, _) => String::from("BNE"),
        (0b1100011, 0b100, _, _) => String::from("BLT"),
        (0b1100011, 0b101, _, _) => String::from("BGE"),
        (0b1100011, 0b110, _, _) => String::from("BLTU"),
        (0b1100011, 0b111, _, _) => String::from("BGEU"),

        // I-type loads
        (0b0000011, 0b000, _, _) => String::from("LB"),
        (0b0000011, 0b001, _, _) => String::from("LH"),
        (0b0000011, 0b010, _, _) => String::from("LW"),
        (0b0000011, 0b100, _, _) => String::from("LBU"),
        (0b0000011, 0b101, _, _) => String::from("LHU"),

        // S-type
        (0b0100011, 0b000, _, _) => String::from("SB"),
        (0b0100011, 0b001, _, _) => String::from("SH"),
        (0b0100011, 0b010, _, _) => String::from("SW"),

        // I-type immediate arithmetic
        (0b0010011, 0b000, _, _) => String::from("ADDI"),
        (0b0010011, 0b010, _, _) => String::from("SLTI"),
        (0b0010011, 0b011, _, _) => String::from("SLTIU"),
        (0b0010011, 0b100, _, _) => String::from("XORI"),
        (0b0010011, 0b110, _, _) => String::from("ORI"),
        (0b0010011, 0b111, _, _) => String::from("ANDI"),

        // I-type shifts (func7 distinguishes, imm ignored)
        (0b0010011, 0b001, 0b0000000, _) => String::from("SLLI"),
        (0b0010011, 0b101, 0b0000000, _) => String::from("SRLI"),
        (0b0010011, 0b101, 0b0100000, _) => String::from("SRAI"),

        // R-type
        (0b0110011, 0b000, 0b0000000, _) => String::from("ADD"),
        (0b0110011, 0b000, 0b0100000, _) => String::from("SUB"),
        (0b0110011, 0b001, 0b0000000, _) => String::from("SLL"),
        (0b0110011, 0b010, 0b0000000, _) => String::from("SLT"),
        (0b0110011, 0b011, 0b0000000, _) => String::from("SLTU"),
        (0b0110011, 0b100, 0b0000000, _) => String::from("XOR"),
        (0b0110011, 0b101, 0b0000000, _) => String::from("SRL"),
        (0b0110011, 0b101, 0b0100000, _) => String::from("SRA"),
        (0b0110011, 0b110, 0b0000000, _) => String::from("OR"),
        (0b0110011, 0b111, 0b0000000, _) => String::from("AND"),

        // FENCE
        (0b0001111, 0b000, _, _) => String::from("FENCE"),

        // SYSTEM
        (0b1110011, 0b000, _, 0b000000000000) => String::from("ECALL"),
        (0b1110011, 0b000, _, 0b000000000001) => String::from("EBREAK"),

        // returning empty string for now
        _ => String::new(),
    }
}
}

fn main() {
    println!("Hello, world!");
}
