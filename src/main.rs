struct Cpu {
    regs: [u32; 32],
    pc: u32,
    mem: Vec<u8>,
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
                    _ => Instruction::Illegal,
                }
            }

            _ => Instruction::Illegal,
        }
    }

    pub fn execute(&mut self, inst: Instruction) {
        match inst {
            // R-type
            Instruction::Add { rd, rs1, rs2 } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] + self.regs[rs2 as usize];
            }

            Instruction::Sub { rd, rs1, rs2 } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] - self.regs[rs2 as usize];
            }

            Instruction::Sll { rd, rs1, rs2 } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] << self.regs[rs2 as usize];
            }

            Instruction::Srl { rd, rs1, rs2 } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] >> self.regs[rs2 as usize];
            }

            Instruction::Sra { rd, rs1, rs2 } => {
                self.regs[rd as usize] =
                    (self.regs[rs1 as usize] as i32 >> self.regs[rs2 as usize]) as u32;
            }

            Instruction::Slt { rd, rs1, rs2 } => {
                self.regs[rd as usize] = 0;
                if (self.regs[rs1 as usize] as i32) < (self.regs[rs2 as usize] as i32) {
                    self.regs[rd as usize] = 1;
                }
            }

            Instruction::Sltu { rd, rs1, rs2 } => {
                self.regs[rd as usize] = 0;
                if self.regs[rs1 as usize] < self.regs[rs2 as usize] {
                    self.regs[rd as usize] = 1;
                }
            }

            Instruction::And { rd, rs1, rs2 } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] & self.regs[rs2 as usize];
            }

            Instruction::Or { rd, rs1, rs2 } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] | self.regs[rs2 as usize];
            }

            Instruction::Xor { rd, rs1, rs2 } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] ^ self.regs[rs2 as usize];
            }

            // I-type arithmetic
            // NOP is encoded as Addi x0, x0, 0
            Instruction::Addi { rd, rs1, imm } => {
                self.regs[rd as usize] = ((self.regs[rs1 as usize] as i32) + imm) as u32;
            }

            Instruction::Slti { rd, rs1, imm } => {
                self.regs[rd as usize] = 0;
                if (self.regs[rs1 as usize] as i32) < imm {
                    self.regs[rd as usize] = 1;
                }
            }

            Instruction::Sltiu { rd, rs1, imm } => {
                self.regs[rd as usize] = 0;
                if self.regs[rs1 as usize] < (imm as u32) {
                    self.regs[rd as usize] = 1;
                }
            }

            Instruction::Xori { rd, rs1, imm } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] ^ imm as u32;
            }

            Instruction::Ori { rd, rs1, imm } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] | imm as u32;
            }

            Instruction::Andi { rd, rs1, imm } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] & imm as u32;
            }

            Instruction::Slli { rd, rs1, shamt } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] << shamt;
            }

            Instruction::Srli { rd, rs1, shamt } => {
                self.regs[rd as usize] = self.regs[rs1 as usize] >> shamt;
            }

            Instruction::Srai { rd, rs1, shamt } => {
                self.regs[rd as usize] = ((self.regs[rs1 as usize] as i32) >> shamt) as u32;
            }

            // I-type loads
            Instruction::Lw { rd, rs1, imm } => {
                // convert to u32 first because we dont want sign extension
                let addr: usize = (self.regs[rs1 as usize] as i32 + imm) as u32 as usize;
                self.regs[rd as usize] = 0;
                // little-endian format, most significant byte in higher address
                self.regs[rd as usize] |= self.mem[addr] as u32;
                self.regs[rd as usize] |= (self.mem[addr + 1] as u32) << 8;
                self.regs[rd as usize] |= (self.mem[addr + 2] as u32) << 16;
                self.regs[rd as usize] |= (self.mem[addr + 3] as u32) << 24;
            }

            Instruction::Lh { rd, rs1, imm } => {
                let addr: usize = (self.regs[rs1 as usize] as i32 + imm) as u32 as usize;
                self.regs[rd as usize] = 0;

                self.regs[rd as usize] |= self.mem[addr] as u32;
                self.regs[rd as usize] |= ((self.mem[addr + 1] as i8 as i32) << 8) as u32;
            }

            Instruction::Lb { rd, rs1, imm } => {
                let addr: usize = (self.regs[rs1 as usize] as i32 + imm) as u32 as usize;
                self.regs[rd as usize] = 0;

                // i32 extends the sign, then cast back to u32 to store
                self.regs[rd as usize] = (self.mem[addr] as i8) as i32 as u32;
            }

            Instruction::Lbu { rd, rs1, imm } => {
                let addr: usize = (self.regs[rs1 as usize] as i32 + imm) as u32 as usize;
                self.regs[rd as usize] = 0;

                self.regs[rd as usize] = self.mem[addr] as u32;
            }

            Instruction::Lhu { rd, rs1, imm } => {
                let addr: usize = (self.regs[rs1 as usize] as i32 + imm) as u32 as usize;
                self.regs[rd as usize] = 0;

                self.regs[rd as usize] |= self.mem[addr] as u32;
                self.regs[rd as usize] |= (self.mem[addr + 1] as u32) << 8;
            }

            // I-type Jump
            Instruction::Jalr { rd, rs1, imm } => {
                let dest_addr: u32 = (self.regs[rs1 as usize] as i32 + imm) as u32 & 0xFFFFFFFE;
                self.regs[rd as usize] = self.pc + 4;
                // -4 because pc is incremented by 4 at the end of execute function
                self.pc = dest_addr - 4;
            }

            // S-type
            Instruction::Sw { rs1, rs2, imm } => {
                let addr: usize = (self.regs[rs1 as usize] as i32 + imm) as u32 as usize;
                let b_0 = (self.regs[rs2 as usize] & 0xFF) as u8;
                let b_1 = ((self.regs[rs2 as usize] & 0xFF00) >> 8) as u8;
                let b_2 = ((self.regs[rs2 as usize] & 0xFF0000) >> 16) as u8;
                let b_3 = ((self.regs[rs2 as usize] & 0xFF000000) >> 24) as u8;

                self.mem[addr as usize] = b_0;
                self.mem[addr as usize + 1] = b_1;
                self.mem[addr as usize + 2] = b_2;
                self.mem[addr as usize + 3] = b_3;
            }

            Instruction::Sh { rs1, rs2, imm } => {
                let addr: usize = (self.regs[rs1 as usize] as i32 + imm) as u32 as usize;
                let b_0 = (self.regs[rs2 as usize] & 0xFF) as u8;
                let b_1 = ((self.regs[rs2 as usize] & 0xFF00) >> 8) as u8;

                self.mem[addr as usize] = b_0;
                self.mem[addr as usize + 1] = b_1;
            }

            Instruction::Sb { rs1, rs2, imm } => {
                let addr: usize = (self.regs[rs1 as usize] as i32 + imm) as u32 as usize;
                let b_0 = (self.regs[rs2 as usize] & 0xFF) as u8;

                self.mem[addr as usize] = b_0;
            }

            // B-type instructions
            Instruction::Beq { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.pc as i32 + imm) as u32;

                if self.regs[rs1 as usize] == self.regs[rs2 as usize] {
                    self.pc = dest_addr - 4;
                }
            }

            Instruction::Bne { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.pc as i32 + imm) as u32;

                if self.regs[rs1 as usize] != self.regs[rs2 as usize] {
                    self.pc = dest_addr - 4;
                }
            }

            Instruction::Blt { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.pc as i32 + imm) as u32;

                if (self.regs[rs1 as usize] as i32) < (self.regs[rs2 as usize] as i32) {
                    self.pc = dest_addr - 4;
                }
            }

            Instruction::Bge { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.pc as i32 + imm) as u32;

                if (self.regs[rs1 as usize] as i32) >= (self.regs[rs2 as usize] as i32) {
                    self.pc = dest_addr - 4;
                }
            }

            Instruction::Bltu { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.pc as i32 + imm) as u32;

                if self.regs[rs1 as usize] < self.regs[rs2 as usize] {
                    self.pc = dest_addr - 4;
                }
            }

            Instruction::Bgeu { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.pc as i32 + imm) as u32;

                if self.regs[rs1 as usize] >= self.regs[rs2 as usize] {
                    self.pc = dest_addr - 4;
                }
            }

            // U-type
            Instruction::Lui { rd, imm } => {
                self.regs[rd as usize] = imm as u32;
            }

            Instruction::Auipc { rd, imm } => {
                self.regs[rd as usize] = (self.pc as i32 + imm) as u32;
            }

            // J=-type

            Instruction::Jal { rd, imm } => {
                self.regs[rd as usize] = self.pc + 4;
                self.pc = (self.pc as i32 + imm) as u32 - 4;
            }

            Instruction::Ecall => {
                println!("Ah.. parley! The code requests an audience with an operating system! whats that? no OS? well then, we're of the edge of the map, mate. Abondoning ship! savyy?");
                panic!("captains dont abandon their ship!");
            }

            Instruction::Ebreak => {
                println!("Haltt!! wait until they figure out where all the rums gone!");
                panic!("rum not found");
            }

            Instruction::Fence => {println!("fence")}

            _ => {println!("illegal")}
        }
        self.pc += 4;
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_add() {
        // ADD x1, x2, x3
        // opcode=0110011, func3=000, func7=0000000
        // rd=9, rs1=2, rs2=19
        let raw: u32 = 0b0000000_10011_00010_000_01001_0110011;
        let cpu = Cpu {
            regs: [0; 32],
            pc: 0,
            mem: Vec::from([0u8; 100]),
        };
        let decoded = cpu.decode(RawInstruction { bits: raw });

        match decoded {
            Instruction::Add { rd, rs1, rs2 } => {
                assert_eq!(rd, 9);
                assert_eq!(rs1, 2);
                assert_eq!(rs2, 19);
            }
            _ => panic!("Expected Add instruction"),
        }
    }
}
