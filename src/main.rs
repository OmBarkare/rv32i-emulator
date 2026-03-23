
struct Cpu {
    regs: [u32; 32],
    pc: u32,
}

enum Instruction {
    // R-type
    Add { rd: u8, rs1: u8, rs2: u8 },
    Sub { rd: u8, rs1: u8, rs2: u8 },
    Sll { rd: u8, rs1: u8, rs2: u8 },
    Slt { rd: u8, rs1: u8, rs2: u8 },
    Sltu { rd: u8, rs1: u8, rs2: u8 },
    Xor { rd: u8, rs1: u8, rs2: u8 },
    Srl { rd: u8, rs1: u8, rs2: u8 },
    Sra { rd: u8, rs1: u8, rs2: u8 },
    Or { rd: u8, rs1: u8, rs2: u8 },
    And { rd: u8, rs1: u8, rs2: u8 },

    // I-type arithmetic
    Addi { rd: u8, rs1: u8, imm: i32 },
    Slti { rd: u8, rs1: u8, imm: i32 },
    Sltiu { rd: u8, rs1: u8, imm: i32 },
    Xori { rd: u8, rs1: u8, imm: i32 },
    Ori { rd: u8, rs1: u8, imm: i32 },
    Andi { rd: u8, rs1: u8, imm: i32 },
    Slli { rd: u8, rs1: u8, shamt: u8 },
    Srli { rd: u8, rs1: u8, shamt: u8 },
    Srai { rd: u8, rs1: u8, shamt: u8 },
    Jalr { rd: u8, rs1: u8, imm: i32 },

    // I-type loads
    Lb { rd: u8, rs1: u8, imm: i32 },
    Lh { rd: u8, rs1: u8, imm: i32 },
    Lw { rd: u8, rs1: u8, imm: i32 },
    Lbu { rd: u8, rs1: u8, imm: i32 },
    Lhu { rd: u8, rs1: u8, imm: i32 },

    // S-type
    Sb { rs1: u8, rs2: u8, imm: i32 },
    Sh { rs1: u8, rs2: u8, imm: i32 },
    Sw { rs1: u8, rs2: u8, imm: i32 },

    // B-type
    Beq { rs1: u8, rs2: u8, imm: i32 },
    Bne { rs1: u8, rs2: u8, imm: i32 },
    Blt { rs1: u8, rs2: u8, imm: i32 },
    Bge { rs1: u8, rs2: u8, imm: i32 },
    Bltu { rs1: u8, rs2: u8, imm: i32 },
    Bgeu { rs1: u8, rs2: u8, imm: i32 },

    // U-type
    Lui { rd: u8, imm: i32 },
    Auipc { rd: u8, imm: i32 },

    // J-type
    Jal { rd: u8, imm: i32 },

    // System
    Ecall,
    Ebreak,

    // Fence
    Fence,

    // Illegal
    Illegal,
}
struct RawInstruction {
    bits: u32,
}

impl Cpu {
    pub fn fetch(&self) -> RawInstruction {
        todo!()
    }

    pub fn decode(&self, raw_inst: RawInstruction) -> Instruction {
        let raw_bits = raw_inst.bits;

        let opcode_bits: u8 = (raw_bits & 0x7F) as u8;
        let rd: u8 = ((raw_bits >> 7) & 0x1F) as u8;
        let rs1: u8 = ((raw_bits >> 15) & 0x1F) as u8;
        let rs2: u8 = ((raw_bits >> 20) & 0x1F) as u8;
        let func3: u8 = ((raw_bits >> 12) & 0x7) as u8;
        let func7: u8 = ((raw_bits >> 25) & 0x7F) as u8;

        // --------- immediates ----------
        // numbers in variable name represent the bit position in the "immediate"
        // not the instruction

        // I-immediate
        let imm_i: i32 = (raw_bits as i32) >> 20;

        // S-immediate
        let imm_11_5_s: i32 = ((raw_bits as i32) >> 20) & (0xFFFFFFE0u32  as i32);
        let imm_4_0_s: i32 = ((raw_bits as i32) >> 7) & 0x1F;

        let imm_s: i32 = imm_11_5_s | imm_4_0_s;

        // B-immediate
        let imm_10_5_b: i32 = ((raw_bits as i32) >> 20) & (0xFFFFF7E0u32 as i32);
        let imm_4_1_b: i32 = ((raw_bits as i32) >> 7) & 0x1E;
        let imm_11_b: i32 = (((raw_bits as i32) >> 7) & 0x1) << 11;

        let imm_b: i32 = imm_10_5_b | imm_4_1_b | imm_11_b;

        // U-immediate
        let imm_u: i32 = (raw_bits & 0xFFFFF000) as i32;

        // J-immediate
        let imm_10_1_j: i32 = ((raw_bits as i32) >> 20) & (0xFFF007FEu32 as i32);
        let imm_11_j: i32 = ((raw_bits as i32) >> 9) & 0x800;
        let imm_19_12_j: i32 = (raw_bits as i32) & 0xFF000;

        let imm_j: i32 = imm_10_1_j | imm_11_j | imm_19_12_j;

        match (opcode_bits, func3, func7) {
            // R-type
            (0b0110011, 0b000, 0b0000000) => Instruction::Add { rd, rs1, rs2 },
            (0b0110011, 0b000, 0b0100000) => Instruction::Sub { rd, rs1, rs2 },
            (0b0110011, 0b001, 0b0000000) => Instruction::Sll { rd, rs1, rs2 },
            (0b0110011, 0b010, 0b0000000) => Instruction::Slt { rd, rs1, rs2 },
            (0b0110011, 0b011, 0b0000000) => Instruction::Sltu { rd, rs1, rs2 },
            (0b0110011, 0b100, 0b0000000) => Instruction::Xor { rd, rs1, rs2 },
            (0b0110011, 0b101, 0b0000000) => Instruction::Srl { rd, rs1, rs2 },
            (0b0110011, 0b101, 0b0100000) => Instruction::Sra { rd, rs1, rs2 },
            (0b0110011, 0b110, 0b0000000) => Instruction::Or { rd, rs1, rs2 },
            (0b0110011, 0b111, 0b0000000) => Instruction::And { rd, rs1, rs2 },

            // I-type arithmetic
            (0b0010011, 0b000, _) => Instruction::Addi {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0010011, 0b010, _) => Instruction::Slti {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0010011, 0b011, _) => Instruction::Sltiu {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0010011, 0b100, _) => Instruction::Xori {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0010011, 0b110, _) => Instruction::Ori {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0010011, 0b111, _) => Instruction::Andi {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0010011, 0b001, 0b0000000) => Instruction::Slli { rd, rs1, shamt:rs2 },
            (0b0010011, 0b101, 0b0000000) => Instruction::Srli { rd, rs1, shamt:rs2 },
            (0b0010011, 0b101, 0b0100000) => Instruction::Srai { rd, rs1, shamt:rs2 },

            // I-type loads
            (0b0000011, 0b000, _) => Instruction::Lb {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0000011, 0b001, _) => Instruction::Lh {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0000011, 0b010, _) => Instruction::Lw {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0000011, 0b100, _) => Instruction::Lbu {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0000011, 0b101, _) => Instruction::Lhu {
                rd,
                rs1,
                imm: imm_i,
            },

            // I-type jump
            (0b1100111, 0b000, _) => Instruction::Jalr {
                rd,
                rs1,
                imm: imm_i,
            },

            // S-type
            (0b0100011, 0b000, _) => Instruction::Sb {
                rs1,
                rs2,
                imm: imm_s,
            },
            (0b0100011, 0b001, _) => Instruction::Sh {
                rs1,
                rs2,
                imm: imm_s,
            },
            (0b0100011, 0b010, _) => Instruction::Sw {
                rs1,
                rs2,
                imm: imm_s,
            },

            // B-type
            (0b1100011, 0b000, _) => Instruction::Beq {
                rs1,
                rs2,
                imm: imm_b,
            },
            (0b1100011, 0b001, _) => Instruction::Bne {
                rs1,
                rs2,
                imm: imm_b,
            },
            (0b1100011, 0b100, _) => Instruction::Blt {
                rs1,
                rs2,
                imm: imm_b,
            },
            (0b1100011, 0b101, _) => Instruction::Bge {
                rs1,
                rs2,
                imm: imm_b,
            },
            (0b1100011, 0b110, _) => Instruction::Bltu {
                rs1,
                rs2,
                imm: imm_b,
            },
            (0b1100011, 0b111, _) => Instruction::Bgeu {
                rs1,
                rs2,
                imm: imm_b,
            },

            // U-type
            (0b0110111, _, _) => Instruction::Lui { rd, imm: imm_u },
            (0b0010111, _, _) => Instruction::Auipc { rd, imm: imm_u },

            // J-type
            (0b1101111, _, _) => Instruction::Jal { rd, imm: imm_j },

            // Fence
            (0b0001111, 0b000, _) => Instruction::Fence,

            // System
            (0b1110011, 0b000, _) => {
                let imm12 = (raw_bits >> 20) & 0xFFF;
                match imm12 {
                    0x000 => Instruction::Ecall,
                    0x001 => Instruction::Ebreak,
                    _ => Instruction::Illegal,
                }
            }

            _ => Instruction::Illegal,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
