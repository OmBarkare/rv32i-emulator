use crate::cpu::Cpu;
use crate::instructions::Instruction;

impl Cpu {
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
                self.regs[rd as usize] = self.pc;
                self.pc = dest_addr;
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
                    self.pc = dest_addr;
                }
            }

            Instruction::Bne { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.pc as i32 + imm) as u32;

                if self.regs[rs1 as usize] != self.regs[rs2 as usize] {
                    self.pc = dest_addr;
                }
            }

            Instruction::Blt { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.pc as i32 + imm) as u32;

                if (self.regs[rs1 as usize] as i32) < (self.regs[rs2 as usize] as i32) {
                    self.pc = dest_addr;
                }
            }

            Instruction::Bge { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.pc as i32 + imm) as u32;

                if (self.regs[rs1 as usize] as i32) >= (self.regs[rs2 as usize] as i32) {
                    self.pc = dest_addr;
                }
            }

            Instruction::Bltu { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.pc as i32 + imm) as u32;

                if self.regs[rs1 as usize] < self.regs[rs2 as usize] {
                    self.pc = dest_addr;
                }
            }

            Instruction::Bgeu { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.pc as i32 + imm) as u32;

                if self.regs[rs1 as usize] >= self.regs[rs2 as usize] {
                    self.pc = dest_addr;
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
                self.pc = (self.pc as i32 + imm) as u32;
            }

            Instruction::Ecall => {
                println!(
                    "Ah.. parley! The code requests an audience with an operating system! whats that? no OS? well then, we're of the edge of the map, mate. Abondoning ship! savyy?"
                );
                panic!("captains dont abandon their ship!");
            }

            Instruction::Ebreak => {
                println!("Haltt!! wait until they figure out where all the rums gone!");
                panic!("rum not found");
            }

            Instruction::Fence => {
                println!("fence")
            }

            _ => {
                println!("illegal")
            }
        }
        self.regs[0] = 0; // fixing rs0 to 0
    }
}

// ---------------- tests -----------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cpu() -> Cpu {
        Cpu {
            regs: [0; 32],
            pc: 0,
            mem: vec![0; 1024],
        }
    }

    // ---- R-type ----

    #[test]
    fn test_add() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 10;
        cpu.regs[2] = 20;
        cpu.execute(Instruction::Add {
            rd: 3,
            rs1: 1,
            rs2: 2,
        });
        assert_eq!(cpu.regs[3], 30);
    }

    #[test]
    fn test_sub() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 20;
        cpu.regs[2] = 10;
        cpu.execute(Instruction::Sub {
            rd: 3,
            rs1: 1,
            rs2: 2,
        });
        assert_eq!(cpu.regs[3], 10);
    }

    #[test]
    fn test_sll() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 1;
        cpu.regs[2] = 4;
        cpu.execute(Instruction::Sll {
            rd: 3,
            rs1: 1,
            rs2: 2,
        });
        assert_eq!(cpu.regs[3], 16);
    }

    #[test]
    fn test_srl() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 16;
        cpu.regs[2] = 4;
        cpu.execute(Instruction::Srl {
            rd: 3,
            rs1: 1,
            rs2: 2,
        });
        assert_eq!(cpu.regs[3], 1);
    }

    #[test]
    fn test_sra_negative() {
        let mut cpu = make_cpu();
        cpu.regs[1] = (-16i32) as u32;
        cpu.regs[2] = 2;
        cpu.execute(Instruction::Sra {
            rd: 3,
            rs1: 1,
            rs2: 2,
        });
        assert_eq!(cpu.regs[3] as i32, -4); // sign bit preserved
    }

    #[test]
    fn test_slt_true() {
        let mut cpu = make_cpu();
        cpu.regs[1] = (-1i32) as u32;
        cpu.regs[2] = 1;
        cpu.execute(Instruction::Slt {
            rd: 3,
            rs1: 1,
            rs2: 2,
        });
        assert_eq!(cpu.regs[3], 1);
    }

    #[test]
    fn test_slt_false() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 1;
        cpu.regs[2] = (-1i32) as u32;
        cpu.execute(Instruction::Slt {
            rd: 3,
            rs1: 1,
            rs2: 2,
        });
        assert_eq!(cpu.regs[3], 0);
    }

    #[test]
    fn test_sltu() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 1;
        cpu.regs[2] = 0xFFFFFFFF; // large unsigned, but -1 signed
        cpu.execute(Instruction::Sltu {
            rd: 3,
            rs1: 1,
            rs2: 2,
        });
        assert_eq!(cpu.regs[3], 1); // 1 < 0xFFFFFFFF unsigned
    }

    #[test]
    fn test_and() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 0b1100;
        cpu.regs[2] = 0b1010;
        cpu.execute(Instruction::And {
            rd: 3,
            rs1: 1,
            rs2: 2,
        });
        assert_eq!(cpu.regs[3], 0b1000);
    }

    #[test]
    fn test_or() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 0b1100;
        cpu.regs[2] = 0b1010;
        cpu.execute(Instruction::Or {
            rd: 3,
            rs1: 1,
            rs2: 2,
        });
        assert_eq!(cpu.regs[3], 0b1110);
    }

    #[test]
    fn test_xor() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 0b1100;
        cpu.regs[2] = 0b1010;
        cpu.execute(Instruction::Xor {
            rd: 3,
            rs1: 1,
            rs2: 2,
        });
        assert_eq!(cpu.regs[3], 0b0110);
    }

    // ---- I-type arithmetic ----

    #[test]
    fn test_addi_positive() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 10;
        cpu.execute(Instruction::Addi {
            rd: 2,
            rs1: 1,
            imm: 5,
        });
        assert_eq!(cpu.regs[2], 15);
    }

    #[test]
    fn test_addi_negative_imm() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 10;
        cpu.execute(Instruction::Addi {
            rd: 2,
            rs1: 1,
            imm: -5,
        });
        assert_eq!(cpu.regs[2], 5);
    }

    #[test]
    fn test_addi_nop() {
        // NOP = ADDI x0, x0, 0 — x0 must always stay 0
        let mut cpu = make_cpu();
        cpu.execute(Instruction::Addi {
            rd: 0,
            rs1: 0,
            imm: 0,
        });
        assert_eq!(cpu.regs[0], 0);
    }

    #[test]
    fn test_slti_true() {
        let mut cpu = make_cpu();
        cpu.regs[1] = (-5i32) as u32;
        cpu.execute(Instruction::Slti {
            rd: 2,
            rs1: 1,
            imm: 0,
        });
        assert_eq!(cpu.regs[2], 1);
    }

    #[test]
    fn test_sltiu() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 4;
        cpu.execute(Instruction::Sltiu {
            rd: 2,
            rs1: 1,
            imm: 5,
        });
        assert_eq!(cpu.regs[2], 1);
    }

    #[test]
    fn test_xori() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 0b1100;
        cpu.execute(Instruction::Xori {
            rd: 2,
            rs1: 1,
            imm: 0b1010,
        });
        assert_eq!(cpu.regs[2], 0b0110);
    }

    #[test]
    fn test_ori() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 0b1100;
        cpu.execute(Instruction::Ori {
            rd: 2,
            rs1: 1,
            imm: 0b1010,
        });
        assert_eq!(cpu.regs[2], 0b1110);
    }

    #[test]
    fn test_andi() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 0b1100;
        cpu.execute(Instruction::Andi {
            rd: 2,
            rs1: 1,
            imm: 0b1010,
        });
        assert_eq!(cpu.regs[2], 0b1000);
    }

    #[test]
    fn test_slli() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 1;
        cpu.execute(Instruction::Slli {
            rd: 2,
            rs1: 1,
            shamt: 3,
        });
        assert_eq!(cpu.regs[2], 8);
    }

    #[test]
    fn test_srli() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 16;
        cpu.execute(Instruction::Srli {
            rd: 2,
            rs1: 1,
            shamt: 2,
        });
        assert_eq!(cpu.regs[2], 4);
    }

    #[test]
    fn test_srai_negative() {
        let mut cpu = make_cpu();
        cpu.regs[1] = (-8i32) as u32;
        cpu.execute(Instruction::Srai {
            rd: 2,
            rs1: 1,
            shamt: 1,
        });
        assert_eq!(cpu.regs[2] as i32, -4);
    }

    // ---- Loads and Stores ----

    #[test]
    fn test_sw_lw_roundtrip() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 100; // base address
        cpu.regs[2] = 0xDEADBEEF;
        cpu.execute(Instruction::Sw {
            rs1: 1,
            rs2: 2,
            imm: 0,
        });
        cpu.execute(Instruction::Lw {
            rd: 3,
            rs1: 1,
            imm: 0,
        });
        assert_eq!(cpu.regs[3], 0xDEADBEEF);
    }

    #[test]
    fn test_sb_lb_roundtrip() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 100;
        cpu.regs[2] = 0xFF; // -1 as i8
        cpu.execute(Instruction::Sb {
            rs1: 1,
            rs2: 2,
            imm: 0,
        });
        cpu.execute(Instruction::Lb {
            rd: 3,
            rs1: 1,
            imm: 0,
        });
        assert_eq!(cpu.regs[3] as i32, -1); // sign extended
    }

    #[test]
    fn test_sb_lbu_no_sign_extend() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 100;
        cpu.regs[2] = 0xFF;
        cpu.execute(Instruction::Sb {
            rs1: 1,
            rs2: 2,
            imm: 0,
        });
        cpu.execute(Instruction::Lbu {
            rd: 3,
            rs1: 1,
            imm: 0,
        });
        assert_eq!(cpu.regs[3], 255); // zero extended, not -1
    }

    #[test]
    fn test_sh_lh_roundtrip() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 100;
        cpu.regs[2] = 0x8000; // negative as i16
        cpu.execute(Instruction::Sh {
            rs1: 1,
            rs2: 2,
            imm: 0,
        });
        cpu.execute(Instruction::Lh {
            rd: 3,
            rs1: 1,
            imm: 0,
        });
        assert_eq!(cpu.regs[3] as i32, -32768); // sign extended
    }

    #[test]
    fn test_sh_lhu_no_sign_extend() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 100;
        cpu.regs[2] = 0x8000;
        cpu.execute(Instruction::Sh {
            rs1: 1,
            rs2: 2,
            imm: 0,
        });
        cpu.execute(Instruction::Lhu {
            rd: 3,
            rs1: 1,
            imm: 0,
        });
        assert_eq!(cpu.regs[3], 0x8000); // zero extended
    }

    #[test]
    fn test_load_store_with_offset() {
        let mut cpu = make_cpu();
        cpu.regs[1] = 100;
        cpu.regs[2] = 0x12345678;
        cpu.execute(Instruction::Sw {
            rs1: 1,
            rs2: 2,
            imm: 4,
        }); // store at 104
        cpu.execute(Instruction::Lw {
            rd: 3,
            rs1: 1,
            imm: 4,
        }); // load from 104
        assert_eq!(cpu.regs[3], 0x12345678);
    }

    // ---- Branches ----

    #[test]
    fn test_beq_taken() {
        let mut cpu = make_cpu();
        cpu.pc = 100;
        cpu.regs[1] = 5;
        cpu.regs[2] = 5;
        cpu.execute(Instruction::Beq {
            rs1: 1,
            rs2: 2,
            imm: 8,
        });
        assert_eq!(cpu.pc, 108); // 100 + 8, then +4 from pc increment
    }

    #[test]
    fn test_beq_not_taken() {
        let mut cpu = make_cpu();
        cpu.pc = 100;
        cpu.regs[1] = 5;
        cpu.regs[2] = 6;
        cpu.execute(Instruction::Beq {
            rs1: 1,
            rs2: 2,
            imm: 8,
        });
        assert_eq!(cpu.pc, 104); // not taken, normal increment
    }

    #[test]
    fn test_bne_taken() {
        let mut cpu = make_cpu();
        cpu.pc = 100;
        cpu.regs[1] = 5;
        cpu.regs[2] = 6;
        cpu.execute(Instruction::Bne {
            rs1: 1,
            rs2: 2,
            imm: 8,
        });
        assert_eq!(cpu.pc, 108);
    }

    #[test]
    fn test_blt_taken() {
        let mut cpu = make_cpu();
        cpu.pc = 100;
        cpu.regs[1] = (-1i32) as u32;
        cpu.regs[2] = 1;
        cpu.execute(Instruction::Blt {
            rs1: 1,
            rs2: 2,
            imm: 8,
        });
        assert_eq!(cpu.pc, 108);
    }

    #[test]
    fn test_bge_taken() {
        let mut cpu = make_cpu();
        cpu.pc = 100;
        cpu.regs[1] = 5;
        cpu.regs[2] = 5;
        cpu.execute(Instruction::Bge {
            rs1: 1,
            rs2: 2,
            imm: 8,
        });
        assert_eq!(cpu.pc, 108);
    }

    #[test]
    fn test_bltu_taken() {
        let mut cpu = make_cpu();
        cpu.pc = 100;
        cpu.regs[1] = 1;
        cpu.regs[2] = 0xFFFFFFFF; // large unsigned
        cpu.execute(Instruction::Bltu {
            rs1: 1,
            rs2: 2,
            imm: 8,
        });
        assert_eq!(cpu.pc, 108);
    }

    #[test]
    fn test_bgeu_taken() {
        let mut cpu = make_cpu();
        cpu.pc = 100;
        cpu.regs[1] = 0xFFFFFFFF;
        cpu.regs[2] = 1;
        cpu.execute(Instruction::Bgeu {
            rs1: 1,
            rs2: 2,
            imm: 8,
        });
        assert_eq!(cpu.pc, 108);
    }

    // ---- U-type ----

    #[test]
    fn test_lui() {
        let mut cpu = make_cpu();
        cpu.execute(Instruction::Lui {
            rd: 1,
            imm: 0x12345000u32 as i32,
        });
        assert_eq!(cpu.regs[1], 0x12345000);
    }

    #[test]
    fn test_auipc() {
        let mut cpu = make_cpu();
        cpu.pc = 100;
        cpu.execute(Instruction::Auipc { rd: 1, imm: 0x1000 });
        assert_eq!(cpu.regs[1], 100 + 0x1000);
    }

    // ---- J-type ----

    #[test]
    fn test_jal() {
        let mut cpu = make_cpu();
        cpu.pc = 100;
        cpu.execute(Instruction::Jal { rd: 1, imm: 8 });
        assert_eq!(cpu.regs[1], 104); // return address = pc + 4
        assert_eq!(cpu.pc, 108); // jumped to 100 + 8, then +4
    }

    #[test]
    fn test_jalr() {
        let mut cpu = make_cpu();
        cpu.pc = 100;
        cpu.regs[1] = 200;
        cpu.execute(Instruction::Jalr {
            rd: 2,
            rs1: 1,
            imm: 4,
        });
        assert_eq!(cpu.regs[2], 100); // return address
        assert_eq!(cpu.pc, 204); // jumped to 200 + 4, then +4
    }

    // ---- x0 hardwired to zero ----

    #[test]
    fn test_x0_always_zero() {
        let mut cpu = make_cpu();
        cpu.execute(Instruction::Addi {
            rd: 0,
            rs1: 0,
            imm: 42,
        });
        assert_eq!(cpu.regs[0], 0); // x0 must never change
    }
}
