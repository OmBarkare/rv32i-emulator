use crate::cpu::Cpu;
use crate::instructions::{Instruction, RawInstruction};

impl Cpu {
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
        let imm_11_5_s: i32 = ((raw_bits as i32) >> 20) & (0xFFFFFFE0u32 as i32);
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

        // ----------- return Instruciton ----------------
        match (opcode_bits, func3, func7) {
            // R-type
            (0b0110011, 0b000, 0b0000000) => Instruction::Add { rd, rs1, rs2 },
            (0b0110011, 0b000, 0b0100000) => Instruction::Sub { rd, rs1, rs2 },
            (0b0110011, 0b001, 0b0000000) => Instruction::Sll { rd, rs1, rs2 },
            (0b0110011, 0b101, 0b0000000) => Instruction::Srl { rd, rs1, rs2 },
            (0b0110011, 0b101, 0b0100000) => Instruction::Sra { rd, rs1, rs2 },
            (0b0110011, 0b010, 0b0000000) => Instruction::Slt { rd, rs1, rs2 },
            (0b0110011, 0b011, 0b0000000) => Instruction::Sltu { rd, rs1, rs2 },
            (0b0110011, 0b111, 0b0000000) => Instruction::And { rd, rs1, rs2 },
            (0b0110011, 0b110, 0b0000000) => Instruction::Or { rd, rs1, rs2 },
            (0b0110011, 0b100, 0b0000000) => Instruction::Xor { rd, rs1, rs2 },

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
            (0b0010011, 0b001, 0b0000000) => Instruction::Slli {
                rd,
                rs1,
                shamt: rs2,
            },
            (0b0010011, 0b101, 0b0000000) => Instruction::Srli {
                rd,
                rs1,
                shamt: rs2,
            },
            (0b0010011, 0b101, 0b0100000) => Instruction::Srai {
                rd,
                rs1,
                shamt: rs2,
            },

            // I-type loads
            (0b0000011, 0b010, _) => Instruction::Lw {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0000011, 0b001, _) => Instruction::Lh {
                rd,
                rs1,
                imm: imm_i,
            },
            (0b0000011, 0b000, _) => Instruction::Lb {
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

            // I-type CSR
            (0b1110011, 0b001, _) => Instruction::Csrrw {
                csr: (imm_i & 0xFFF) as u16,
                rs1,
                rd,
            },
            (0b1110011, 0b010, _) => Instruction::Csrrs {
                csr: (imm_i & 0xFFF) as u16,
                rs1,
                rd,
            },
            (0b1110011, 0b011, _) => Instruction::Csrrc {
                csr: (imm_i & 0xFFF) as u16,
                rs1,
                rd,
            },
            (0b1110011, 0b101, _) => Instruction::Csrrwi {
                csr: (imm_i & 0xFFF) as u16,
                uimm: rs1,
                rd,
            },
            (0b1110011, 0b110, _) => Instruction::Csrrsi {
                csr: (imm_i & 0xFFF) as u16,
                uimm: rs1,
                rd,
            },
            (0b1110011, 0b111, _) => Instruction::Csrrci {
                csr: (imm_i & 0xFFF) as u16,
                uimm: rs1,
                rd,
            },

            // S-type
            (0b0100011, 0b010, _) => Instruction::Sw {
                rs1,
                rs2,
                imm: imm_s,
            },
            (0b0100011, 0b001, _) => Instruction::Sh {
                rs1,
                rs2,
                imm: imm_s,
            },
            (0b0100011, 0b000, _) => Instruction::Sb {
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
                    0x0302 => Instruction::Mret,
                    _ => Instruction::Illegal,
                }
            }

            _ => Instruction::Illegal,
        }
    }
}

// -------------- tests --------------------
#[cfg(test)]
mod tests {
    use super::*;

    // ---- R-type ----

    #[test]
    fn test_decode_add() {
        // ADD x9, x2, x19
        let raw: u32 = 0b0000000_10011_00010_000_01001_0110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Add { rd, rs1, rs2 } => {
                assert_eq!(rd, 9);
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 19);
            }
            _ => panic!("Expected Add"),
        }
    }

    #[test]
    fn test_decode_sub() {
        // SUB x1, x2, x3
        let raw: u32 = 0b0100000_00011_00010_000_00001_0110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Sub { rd, rs1, rs2 } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
            }
            _ => panic!("Expected Sub"),
        }
    }

    #[test]
    fn test_decode_sll() {
        // SLL x1, x2, x3
        let raw: u32 = 0b0000000_00011_00010_001_00001_0110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Sll { rd, rs1, rs2 } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
            }
            _ => panic!("Expected Sll"),
        }
    }

    #[test]
    fn test_decode_srl() {
        // SRL x1, x2, x3
        let raw: u32 = 0b0000000_00011_00010_101_00001_0110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Srl { rd, rs1, rs2 } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
            }
            _ => panic!("Expected Srl"),
        }
    }

    #[test]
    fn test_decode_sra() {
        // SRA x1, x2, x3
        let raw: u32 = 0b0100000_00011_00010_101_00001_0110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Sra { rd, rs1, rs2 } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
            }
            _ => panic!("Expected Sra"),
        }
    }

    #[test]
    fn test_decode_slt() {
        let raw: u32 = 0b0000000_00011_00010_010_00001_0110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Slt { rd, rs1, rs2 } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
            }
            _ => panic!("Expected Slt"),
        }
    }

    #[test]
    fn test_decode_sltu() {
        let raw: u32 = 0b0000000_00011_00010_011_00001_0110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Sltu { rd, rs1, rs2 } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
            }
            _ => panic!("Expected Sltu"),
        }
    }

    #[test]
    fn test_decode_and() {
        let raw: u32 = 0b0000000_00011_00010_111_00001_0110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::And { rd, rs1, rs2 } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
            }
            _ => panic!("Expected And"),
        }
    }

    #[test]
    fn test_decode_or() {
        let raw: u32 = 0b0000000_00011_00010_110_00001_0110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Or { rd, rs1, rs2 } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
            }
            _ => panic!("Expected Or"),
        }
    }

    #[test]
    fn test_decode_xor() {
        let raw: u32 = 0b0000000_00011_00010_100_00001_0110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Xor { rd, rs1, rs2 } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
            }
            _ => panic!("Expected Xor"),
        }
    }

    // ---- I-type arithmetic ----

    #[test]
    fn test_decode_addi_positive() {
        // ADDI x1, x2, 42
        let raw: u32 = 0b000000101010_00010_000_00001_0010011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Addi { rd, rs1, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(imm, 42);
            }
            _ => panic!("Expected Addi"),
        }
    }

    #[test]
    fn test_decode_addi_negative() {
        // ADDI x1, x2, -1
        // imm = 0b111111111111
        let raw: u32 = 0b111111111111_00010_000_00001_0010011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Addi { rd, rs1, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(imm, -1); // sign extended
            }
            _ => panic!("Expected Addi"),
        }
    }

    #[test]
    fn test_decode_slti() {
        let raw: u32 = 0b000000000001_00001_010_00010_0010011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Slti { rd, rs1, imm } => {
                assert_eq!(rd, 2);
                assert_eq!(rs1, 1);
                assert_eq!(imm, 1);
            }
            _ => panic!("Expected Slti"),
        }
    }

    #[test]
    fn test_decode_sltiu() {
        let raw: u32 = 0b000000000001_00001_011_00010_0010011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Sltiu { rd, rs1, imm } => {
                assert_eq!(rd, 2);
                assert_eq!(rs1, 1);
                assert_eq!(imm, 1);
            }
            _ => panic!("Expected Sltiu"),
        }
    }

    #[test]
    fn test_decode_xori() {
        let raw: u32 = 0b000000001111_00001_100_00010_0010011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Xori { rd, rs1, imm } => {
                assert_eq!(rd, 2);
                assert_eq!(rs1, 1);
                assert_eq!(imm, 15);
            }
            _ => panic!("Expected Xori"),
        }
    }

    #[test]
    fn test_decode_ori() {
        let raw: u32 = 0b000000001111_00001_110_00010_0010011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Ori { rd, rs1, imm } => {
                assert_eq!(rd, 2);
                assert_eq!(rs1, 1);
                assert_eq!(imm, 15);
            }
            _ => panic!("Expected Ori"),
        }
    }

    #[test]
    fn test_decode_andi() {
        let raw: u32 = 0b000000001111_00001_111_00010_0010011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Andi { rd, rs1, imm } => {
                assert_eq!(rd, 2);
                assert_eq!(rs1, 1);
                assert_eq!(imm, 15);
            }
            _ => panic!("Expected Andi"),
        }
    }

    #[test]
    fn test_decode_slli() {
        // SLLI x1, x2, shamt=4
        let raw: u32 = 0b0000000_00100_00010_001_00001_0010011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Slli { rd, rs1, shamt } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(shamt, 4);
            }
            _ => panic!("Expected Slli"),
        }
    }

    #[test]
    fn test_decode_srli() {
        let raw: u32 = 0b0000000_00100_00010_101_00001_0010011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Srli { rd, rs1, shamt } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(shamt, 4);
            }
            _ => panic!("Expected Srli"),
        }
    }

    #[test]
    fn test_decode_srai() {
        let raw: u32 = 0b0100000_00100_00010_101_00001_0010011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Srai { rd, rs1, shamt } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(shamt, 4);
            }
            _ => panic!("Expected Srai"),
        }
    }

    // ---- Loads ----

    #[test]
    fn test_decode_lw() {
        // LW x1, 8(x2)
        let raw: u32 = 0b000000001000_00010_010_00001_0000011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Lw { rd, rs1, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(imm, 8);
            }
            _ => panic!("Expected Lw"),
        }
    }

    #[test]
    fn test_decode_lw_negative_offset() {
        // LW x1, -4(x2)
        let raw: u32 = 0b111111111100_00010_010_00001_0000011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Lw { rd, rs1, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(imm, -4);
            }
            _ => panic!("Expected Lw with negative offset"),
        }
    }

    #[test]
    fn test_decode_lh() {
        let raw: u32 = 0b000000000100_00010_001_00001_0000011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Lh { rd, rs1, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(imm, 4);
            }
            _ => panic!("Expected Lh"),
        }
    }

    #[test]
    fn test_decode_lb() {
        let raw: u32 = 0b000000000100_00010_000_00001_0000011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Lb { rd, rs1, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(imm, 4);
            }
            _ => panic!("Expected Lb"),
        }
    }

    #[test]
    fn test_decode_lbu() {
        let raw: u32 = 0b000000000100_00010_100_00001_0000011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Lbu { rd, rs1, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(imm, 4);
            }
            _ => panic!("Expected Lbu"),
        }
    }

    #[test]
    fn test_decode_lhu() {
        let raw: u32 = 0b000000000100_00010_101_00001_0000011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Lhu { rd, rs1, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(imm, 4);
            }
            _ => panic!("Expected Lhu"),
        }
    }

    // ---- Stores ----

    #[test]
    fn test_decode_sw() {
        // SW x3, 8(x2)  — imm=8, rs1=2, rs2=3
        // imm[11:5]=0000000, imm[4:0]=01000
        let raw: u32 = 0b0000000_00011_00010_010_01000_0100011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Sw { rs1, rs2, imm } => {
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
                assert_eq!(imm, 8);
            }
            _ => panic!("Expected Sw"),
        }
    }

    #[test]
    fn test_decode_sw_negative_offset() {
        // SW x3, -4(x2)
        // imm=-4 = 0b111111111100
        // imm[11:5]=1111111, imm[4:0]=11100
        let raw: u32 = 0b1111111_00011_00010_010_11100_0100011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Sw { rs1, rs2, imm } => {
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
                assert_eq!(imm, -4);
            }
            _ => panic!("Expected Sw with negative offset"),
        }
    }

    #[test]
    fn test_decode_sh() {
        let raw: u32 = 0b0000000_00011_00010_001_01000_0100011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Sh { rs1, rs2, imm } => {
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
                assert_eq!(imm, 8);
            }
            _ => panic!("Expected Sh"),
        }
    }

    #[test]
    fn test_decode_sb() {
        let raw: u32 = 0b0000000_00011_00010_000_01000_0100011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Sb { rs1, rs2, imm } => {
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 3);
                assert_eq!(imm, 8);
            }
            _ => panic!("Expected Sb"),
        }
    }

    // ---- Branches ----

    #[test]
    fn test_decode_beq() {
        // BEQ x1, x2, +8
        // imm=8=0b0000000001000
        // bit12=0,bit11=0,imm[10:5]=000000,imm[4:1]=0100
        // bit31=0,bits30:25=000000,bits11:8=0100,bit7=0
        let raw: u32 = 0b0_000000_00010_00001_000_0100_0_1100011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Beq { rs1, rs2, imm } => {
                assert_eq!(rs1, 1);
                assert_eq!(rs2, 2);
                assert_eq!(imm, 8);
            }
            _ => panic!("Expected Beq"),
        }
    }

    #[test]
    fn test_decode_bne() {
        let raw: u32 = 0b0_000000_00010_00001_001_0100_0_1100011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Bne { rs1, rs2, imm } => {
                assert_eq!(rs1, 1);
                assert_eq!(rs2, 2);
                assert_eq!(imm, 8);
            }
            _ => panic!("Expected Bne"),
        }
    }

    #[test]
    fn test_decode_blt() {
        let raw: u32 = 0b0_000000_00010_00001_100_0100_0_1100011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Blt { rs1, rs2, imm } => {
                assert_eq!(rs1, 1);
                assert_eq!(rs2, 2);
                assert_eq!(imm, 8);
            }
            _ => panic!("Expected Blt"),
        }
    }

    #[test]
    fn test_decode_bge() {
        let raw: u32 = 0b0_000000_00010_00001_101_0100_0_1100011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Bge { rs1, rs2, imm } => {
                assert_eq!(rs1, 1);
                assert_eq!(rs2, 2);
                assert_eq!(imm, 8);
            }
            _ => panic!("Expected Bge"),
        }
    }

    #[test]
    fn test_decode_bltu() {
        let raw: u32 = 0b0_000000_00010_00001_110_0100_0_1100011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Bltu { rs1, rs2, imm } => {
                assert_eq!(rs1, 1);
                assert_eq!(rs2, 2);
                assert_eq!(imm, 8);
            }
            _ => panic!("Expected Bltu"),
        }
    }

    #[test]
    fn test_decode_bgeu() {
        let raw: u32 = 0b0_000000_00010_00001_111_0100_0_1100011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Bgeu { rs1, rs2, imm } => {
                assert_eq!(rs1, 1);
                assert_eq!(rs2, 2);
                assert_eq!(imm, 8);
            }
            _ => panic!("Expected Bgeu"),
        }
    }

    #[test]
    fn test_decode_branch_negative_offset() {
        // BEQ x1, x2, -4
        let raw: u32 = 0b1_111111_00010_00001_000_1110_1_1100011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Beq { rs1, rs2, imm } => {
                assert_eq!(rs1, 1);
                assert_eq!(rs2, 2);
                assert_eq!(imm, -4);
            }
            _ => panic!("Expected Beq with negative offset"),
        }
    }

    // ---- U-type ----

    #[test]
    fn test_decode_lui() {
        // LUI x1, 0x12345
        let raw: u32 = 0b00010010001101000101_00001_0110111;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Lui { rd, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(imm, 0x12345000u32 as i32);
            }
            _ => panic!("Expected Lui"),
        }
    }

    #[test]
    fn test_decode_auipc() {
        let raw: u32 = 0b00010010001101000101_00001_0010111;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Auipc { rd, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(imm, 0x12345000u32 as i32);
            }
            _ => panic!("Expected Auipc"),
        }
    }

    // ---- J-type ----

    #[test]
    fn test_decode_jal() {
        // JAL x1, +8
        // imm=8, rd=1
        let raw: u32 = 0b0_0000000100_0_00000000_00001_1101111;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Jal { rd, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(imm, 8);
            }
            _ => panic!("Expected Jal"),
        }
    }

    #[test]
    fn test_decode_jalr() {
        // JALR x1, x2, 4
        let raw: u32 = 0b000000000100_00010_000_00001_1100111;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Jalr { rd, rs1, imm } => {
                assert_eq!(rd, 1);
                assert_eq!(rs1, 2);
                assert_eq!(imm, 4);
            }
            _ => panic!("Expected Jalr"),
        }
    }

    // ---- System ----

    #[test]
    fn test_decode_ecall() {
        let raw: u32 = 0b000000000000_00000_000_00000_1110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Ecall => {}
            _ => panic!("Expected Ecall"),
        }
    }

    #[test]
    fn test_decode_ebreak() {
        let raw: u32 = 0b000000000001_00000_000_00000_1110011;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Ebreak => {}
            _ => panic!("Expected Ebreak"),
        }
    }

    // ---- Fence ----

    #[test]
    fn test_decode_fence() {
        let raw: u32 = 0b0000_0000_0000_00000_000_00000_0001111;
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Fence => {}
            _ => panic!("Expected Fence"),
        }
    }

    // ---- Illegal ----

    #[test]
    fn test_decode_illegal() {
        let raw: u32 = 0x00000000; // all zeros — not a valid instruction
        let decoded = Cpu::new().decode(RawInstruction { bits: raw });
        match decoded {
            Instruction::Illegal => {}
            _ => panic!("Expected Illegal"),
        }
    }
}
