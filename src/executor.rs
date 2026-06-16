use crate::cpu::Cpu;
use crate::instructions::Instruction;

impl Cpu {
    pub fn execute(&mut self, inst: Instruction) {
        match inst {
            // R-type
            Instruction::Add { rd, rs1, rs2 } => {
                self.regs[rd as usize] =
                    self.regs[rs1 as usize].wrapping_add(self.regs[rs2 as usize]);
            }

            Instruction::Sub { rd, rs1, rs2 } => {
                self.regs[rd as usize] =
                    self.regs[rs1 as usize].wrapping_sub(self.regs[rs2 as usize]);
            }

            Instruction::Sll { rd, rs1, rs2 } => {
                self.regs[rd as usize] =
                    self.regs[rs1 as usize].wrapping_shl(self.regs[rs2 as usize]);
            }

            Instruction::Srl { rd, rs1, rs2 } => {
                self.regs[rd as usize] =
                    self.regs[rs1 as usize].wrapping_shr(self.regs[rs2 as usize]);
            }

            Instruction::Sra { rd, rs1, rs2 } => {
                self.regs[rd as usize] =
                    (self.regs[rs1 as usize] as i32).wrapping_shr(self.regs[rs2 as usize]) as u32;
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
                self.regs[rd as usize] = self.regs[rs1 as usize].wrapping_add(imm as u32)
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
                let addr = (self.regs[rs1 as usize] as i32 + imm) as u32;
                self.regs[rd as usize] = 0;

                self.regs[rd as usize] = self.mem.read_32(addr).unwrap();
            }

            Instruction::Lh { rd, rs1, imm } => {
                let addr = (self.regs[rs1 as usize] as i32 + imm) as u32;

                // loading half word does not preserve any higher bits, so we can
                // assing directly
                // casting for sign extension
                self.regs[rd as usize] = self.mem.read_16(addr).unwrap() as i16 as i32 as u32;
            }

            Instruction::Lb { rd, rs1, imm } => {
                let addr = (self.regs[rs1 as usize] as i32 + imm) as u32;
                self.regs[rd as usize] = 0;

                // i32 extends the sign, then cast back to u32 to load
                self.regs[rd as usize] = (self.mem.read_8(addr).unwrap() as i8) as i32 as u32;
            }

            Instruction::Lbu { rd, rs1, imm } => {
                let addr = (self.regs[rs1 as usize] as i32 + imm) as u32;
                self.regs[rd as usize] = 0;

                self.regs[rd as usize] = self.mem.read_8(addr).unwrap() as u32;
            }

            Instruction::Lhu { rd, rs1, imm } => {
                let addr = (self.regs[rs1 as usize] as i32 + imm) as u32;
                self.regs[rd as usize] = 0;

                self.regs[rd as usize] = self.mem.read_16(addr).unwrap() as u32;
            }

            // I-type Jump
            Instruction::Jalr { rd, rs1, imm } => {
                let dest_addr: u32 = (self.regs[rs1 as usize] as i32 + imm) as u32 & 0xFFFFFFFE;
                self.regs[rd as usize] = self.curr_pc + 4;
                self.pc = dest_addr;
            }

            // I-type CSR
            Instruction::Csrrw { csr, rs1, rd } => {
                let access_res = self
                    .csrs
                    .access_write(csr, rd != 0, true, self.regs[rs1 as usize])
                    .unwrap();

                if let Some(val) = access_res.read_val {
                    self.regs[rd as usize] = val;
                }
            }
            Instruction::Csrrs { csr, rs1, rd } => {
                let access_res = self
                    .csrs
                    .access_set(csr, true, rs1 != 0, self.regs[rs1 as usize])
                    .unwrap();
                self.regs[rd as usize] = access_res.read_val.unwrap();
            }

            Instruction::Csrrc { csr, rs1, rd } => {
                let access_res = self
                    .csrs
                    .access_clear(csr, true, rs1 != 0, self.regs[rs1 as usize])
                    .unwrap();
                self.regs[rd as usize] = access_res.read_val.unwrap();
            }

            Instruction::Csrrwi { csr, uimm, rd } => {
                let access_res = self
                    .csrs
                    .access_write(csr, rd != 0, true, uimm as u32)
                    .unwrap();

                if let Some(val) = access_res.read_val {
                    self.regs[rd as usize] = val;
                }
            }
            Instruction::Csrrsi { csr, uimm, rd } => {
                let access_res = self
                    .csrs
                    .access_set(csr, true, uimm != 0, uimm as u32)
                    .unwrap();
                self.regs[rd as usize] = access_res.read_val.unwrap();
            }
            Instruction::Csrrci { csr, uimm, rd } => {
                let access_res = self
                    .csrs
                    .access_clear(csr, true, uimm != 0, uimm as u32)
                    .unwrap();
                self.regs[rd as usize] = access_res.read_val.unwrap();
            }

            // S-type
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = (self.regs[rs1 as usize] as i32 + imm) as u32;

                self.mem.write_32(addr, self.regs[rs2 as usize]).unwrap();
            }

            Instruction::Sh { rs1, rs2, imm } => {
                let addr = (self.regs[rs1 as usize] as i32 + imm) as u32;

                self.mem
                    .write_16(addr, self.regs[rs2 as usize] as u16)
                    .unwrap();
            }

            Instruction::Sb { rs1, rs2, imm } => {
                let addr = (self.regs[rs1 as usize] as i32 + imm) as u32;

                self.mem
                    .write_8(addr, self.regs[rs2 as usize] as u8)
                    .unwrap();
            }

            // B-type instructions
            Instruction::Beq { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.curr_pc as i32 + imm) as u32;

                if self.regs[rs1 as usize] == self.regs[rs2 as usize] {
                    self.pc = dest_addr;
                }
            }

            Instruction::Bne { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.curr_pc as i32 + imm) as u32;

                if self.regs[rs1 as usize] != self.regs[rs2 as usize] {
                    self.pc = dest_addr;
                }
            }

            Instruction::Blt { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.curr_pc as i32 + imm) as u32;

                if (self.regs[rs1 as usize] as i32) < (self.regs[rs2 as usize] as i32) {
                    self.pc = dest_addr;
                }
            }

            Instruction::Bge { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.curr_pc as i32 + imm) as u32;

                if (self.regs[rs1 as usize] as i32) >= (self.regs[rs2 as usize] as i32) {
                    self.pc = dest_addr;
                }
            }

            Instruction::Bltu { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.curr_pc as i32 + imm) as u32;

                if self.regs[rs1 as usize] < self.regs[rs2 as usize] {
                    self.pc = dest_addr;
                }
            }

            Instruction::Bgeu { rs1, rs2, imm } => {
                let dest_addr: u32 = (self.curr_pc as i32 + imm) as u32;

                if self.regs[rs1 as usize] >= self.regs[rs2 as usize] {
                    self.pc = dest_addr;
                }
            }

            // U-type
            Instruction::Lui { rd, imm } => {
                self.regs[rd as usize] = imm as u32;
            }

            Instruction::Auipc { rd, imm } => {
                self.regs[rd as usize] = (self.curr_pc as i32 + imm) as u32;
            }

            // J=-type
            Instruction::Jal { rd, imm } => {
                self.regs[rd as usize] = self.curr_pc + 4;
                self.pc = (self.curr_pc as i32 + imm) as u32;
            }

            // REVIEW: ecall has different cause codes when
            // called from different privilege modes.
            // 11 is when it returns from M-mode.
            Instruction::Ecall => {
                self.trap(11, 0);
            }

            Instruction::Ebreak => {
                self.trap(3, 0);
            }

            Instruction::Mret => {
                // restore MIE from MPIE
                let mpie = self.csrs.read_mstatus_mpie();
                self.csrs.write_mstatus_mie(mpie);
                self.csrs.write_mstatus_mpie(true);
                self.csrs.write_mstatus_mpp(3);
                self.pc = self.csrs.mepc;
            }

            Instruction::Fence => {
                log::debug!("fence")
            }

            Instruction::Illegal => {
                self.trap(2, self.mem.read_32(self.curr_pc).unwrap());
            }
        }
        self.regs[0] = 0; // fixing rs0 to 0
    }
}

// ---------------- tests -----------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- R-type ----

    #[test]
    fn test_add() {
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
        cpu.execute(Instruction::Addi {
            rd: 0,
            rs1: 0,
            imm: 0,
        });
        assert_eq!(cpu.regs[0], 0);
    }

    #[test]
    fn test_slti_true() {
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
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
        let mut cpu = Cpu::new();
        cpu.mem.allocate_page(0, 1).unwrap();
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
        let mut cpu = Cpu::new();
        cpu.mem.allocate_page(0, 1).unwrap();
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
        let mut cpu = Cpu::new();
        cpu.mem.allocate_page(0, 1).unwrap();
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
        let mut cpu = Cpu::new();
        cpu.mem.allocate_page(0, 1).unwrap();
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
        let mut cpu = Cpu::new();
        cpu.mem.allocate_page(0, 1).unwrap();
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
        let mut cpu = Cpu::new();
        cpu.mem.allocate_page(0, 1).unwrap();
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
        let mut cpu = Cpu::new();
        cpu.curr_pc = 100;
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
        let mut cpu = Cpu::new();
        cpu.curr_pc = 100;
        cpu.pc = 104; // pc incremented by fetch unit before it reaches execute
        cpu.regs[1] = 5;
        cpu.regs[2] = 6;
        cpu.execute(Instruction::Beq {
            rs1: 1,
            rs2: 2,
            imm: 8,
        });
        assert_eq!(cpu.pc, 104); // PC is updated by fetch unit, so will stay same if branch not taken
    }

    #[test]
    fn test_bne_taken() {
        let mut cpu = Cpu::new();
        cpu.curr_pc = 100;
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
        let mut cpu = Cpu::new();
        cpu.curr_pc = 100;
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
        let mut cpu = Cpu::new();
        cpu.curr_pc = 100;
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
        let mut cpu = Cpu::new();
        cpu.curr_pc = 100;
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
        let mut cpu = Cpu::new();
        cpu.curr_pc = 100;
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
        let mut cpu = Cpu::new();
        cpu.execute(Instruction::Lui {
            rd: 1,
            imm: 0x12345000u32 as i32,
        });
        assert_eq!(cpu.regs[1], 0x12345000);
    }

    #[test]
    fn test_auipc() {
        let mut cpu = Cpu::new();
        cpu.curr_pc = 100;
        cpu.execute(Instruction::Auipc { rd: 1, imm: 0x1000 });
        assert_eq!(cpu.regs[1], 100 + 0x1000);
    }

    // ---- J-type ----

    #[test]
    fn test_jal() {
        let mut cpu = Cpu::new();
        cpu.curr_pc = 100;
        cpu.execute(Instruction::Jal { rd: 1, imm: 8 });
        assert_eq!(cpu.regs[1], 104); // return address = pc + 4
        assert_eq!(cpu.pc, 108); // jumped to 100 + 8, then +4
    }

    #[test]
    fn test_jalr() {
        let mut cpu = Cpu::new();
        cpu.curr_pc = 100;
        cpu.regs[1] = 200;
        cpu.execute(Instruction::Jalr {
            rd: 2,
            rs1: 1,
            imm: 4,
        });
        assert_eq!(cpu.regs[2], 100 + 4); // return address
        assert_eq!(cpu.pc, 204); // jumped to 200 + 4, then +4
    }

    // ---- x0 hardwired to zero ----

    #[test]
    fn test_x0_always_zero() {
        let mut cpu = Cpu::new();
        cpu.execute(Instruction::Addi {
            rd: 0,
            rs1: 0,
            imm: 42,
        });
        assert_eq!(cpu.regs[0], 0); // x0 must never change
    }

    #[test]
    fn test_csrrw_store() {
        let mut cpu = Cpu::new();
        cpu.regs[8] = 0xABCD1234;
        cpu.execute(Instruction::Csrrw {
            csr: 0x340,
            rs1: 8,
            rd: 0,
        });
        assert_eq!(cpu.csrs.mscratch, 0xABCD1234);
    }
}

#[cfg(test)]
mod csr_tests {
    use super::*;
    use crate::cpu::Cpu;
    use crate::instructions::Instruction;

    // =========================================================================
    // CSRRW tests
    // =========================================================================
    // CSRRW: rd = old CSR value, CSR = rs1
    // The OLD value must be returned in rd BEFORE the CSR is modified.
    // =========================================================================

    #[test]
    fn test_csrrw_writes_csr() {
        // Basic write: does the CSR get the value from rs1?
        let mut cpu = Cpu::new();
        cpu.regs[5] = 0xABCD1234; // t0 = 0xABCD1234
        cpu.execute(Instruction::Csrrw {
            csr: 0x340, // mscratch
            rs1: 5,
            rd: 0, // discard old value (rd=x0)
        });
        assert_eq!(
            cpu.csrs.mscratch, 0xABCD1234,
            "CSRRW must write rs1 into CSR"
        );
    }

    #[test]
    fn test_csrrw_returns_old_value() {
        // CSRRW must return the OLD value of the CSR into rd,
        // not the new value, and not zero.
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0x11111111; // pre-load CSR with a known value
        cpu.regs[5] = 0x22222222; // new value to write

        cpu.execute(Instruction::Csrrw {
            csr: 0x340,
            rs1: 5,
            rd: 6, // rd = t1, should receive OLD value
        });

        assert_eq!(
            cpu.regs[6], 0x11111111,
            "CSRRW rd must hold the OLD CSR value"
        );
        assert_eq!(
            cpu.csrs.mscratch, 0x22222222,
            "CSRRW must write new value to CSR"
        );
    }

    #[test]
    fn test_csrrw_rd_zero_no_read() {
        // When rd = x0, the spec says the CSR read can be skipped
        // (no side effects from reading). For mscratch this doesn't matter,
        // but the write must still happen.
        let mut cpu = Cpu::new();
        cpu.regs[5] = 0xDEADBEEF;
        cpu.execute(Instruction::Csrrw {
            csr: 0x340,
            rs1: 5,
            rd: 0, // x0 — discard old value
        });
        assert_eq!(
            cpu.csrs.mscratch, 0xDEADBEEF,
            "CSRRW with rd=x0 must still write CSR"
        );
        assert_eq!(cpu.regs[0], 0, "x0 must stay 0 even as CSRRW rd");
    }

    // This is the exact sequence from section 8 of the binary:
    // csrw mscratch, t0   (t0 = 0xABCD1234)
    // csrr t1, mscratch
    // bne  t0, t1, _fail
    #[test]
    fn test_csrrw_then_csrrs_read_back() {
        // csrw = csrrw x0, csr, rs1
        // csrr = csrrs rd, csr, x0  (rs1=x0 means no bits set, pure read)
        let mut cpu = Cpu::new();
        cpu.regs[5] = 0xABCD1234; // t0

        // csrw mscratch, t0
        cpu.execute(Instruction::Csrrw {
            csr: 0x340,
            rs1: 5,
            rd: 0,
        });

        // csrr t1, mscratch  (implemented as csrrs with rs1=x0)
        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 0, // x0 — set no bits, pure read
            rd: 6,  // t1
        });

        assert_eq!(
            cpu.regs[6], 0xABCD1234,
            "csrr (csrrs x0) must read back written value"
        );
    }

    // =========================================================================
    // CSRRS tests
    // =========================================================================
    // CSRRS: rd = old CSR value, CSR = CSR | rs1
    // Bits that are 1 in rs1 get SET in the CSR.
    // Bits that are 0 in rs1 are unchanged.
    // =========================================================================

    #[test]
    fn test_csrrs_sets_bits() {
        // Start with 0x000000FF, set upper bits with 0xFFFFFF00
        // Result should be 0xFFFFFFFF
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0x000000FF;
        cpu.regs[6] = 0xFFFFFF00; // t1 = bits to set

        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 6,
            rd: 7, // t2 = old value
        });

        assert_eq!(
            cpu.regs[7], 0x000000FF,
            "CSRRS rd must hold OLD value (before set)"
        );
        assert_eq!(cpu.csrs.mscratch, 0xFFFFFFFF, "CSRRS must OR rs1 into CSR");
    }

    #[test]
    fn test_csrrs_does_not_clear_bits() {
        // CSRRS only sets bits, never clears them.
        // rs1 = 0x0F: only bits 3:0 should change (get set if not already).
        // All other bits must remain as they were.
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0xF0F0F0F0;
        cpu.regs[5] = 0x0F0F0F0F;

        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 5,
            rd: 0,
        });

        assert_eq!(
            cpu.csrs.mscratch, 0xFFFFFFFF,
            "CSRRS must set bits without clearing others"
        );
    }

    #[test]
    fn test_csrrs_rs1_zero_is_pure_read() {
        // When rs1 = x0, CSRRS reads without modifying.
        // This is the "csrr" pseudo-instruction.
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0x12345678;

        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 0, // x0 — no bits to set
            rd: 5,
        });

        assert_eq!(
            cpu.regs[5], 0x12345678,
            "CSRRS with rs1=x0 must read CSR unchanged"
        );
        assert_eq!(
            cpu.csrs.mscratch, 0x12345678,
            "CSRRS with rs1=x0 must not modify CSR"
        );
    }

    #[test]
    fn test_csrrs_returns_old_value_before_set() {
        // This is the most critical CSRRS invariant:
        // rd gets the value BEFORE the OR operation, not after.
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0x000000FF;
        cpu.regs[6] = 0xFFFFFF00;

        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 6,
            rd: 7,
        });

        // rd must be the PRE-modification value
        assert_eq!(
            cpu.regs[7], 0x000000FF,
            "CSRRS must return OLD value (0xFF), not new value (0xFFFFFFFF)"
        );
    }

    // =========================================================================
    // CSRRC tests
    // =========================================================================
    // CSRRC: rd = old CSR value, CSR = CSR & ~rs1
    // Bits that are 1 in rs1 get CLEARED in the CSR.
    // Bits that are 0 in rs1 are unchanged.
    // =========================================================================

    #[test]
    fn test_csrrc_clears_bits() {
        // Start with 0xFFFFFFFF, clear bits 0x0F0F0F0F
        // Result should be 0xF0F0F0F0
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0xFFFFFFFF;
        cpu.regs[6] = 0x0F0F0F0F; // bits to clear

        cpu.execute(Instruction::Csrrc {
            csr: 0x340,
            rs1: 6,
            rd: 7, // t2 = old value
        });

        assert_eq!(
            cpu.regs[7], 0xFFFFFFFF,
            "CSRRC rd must hold OLD value (before clear)"
        );
        assert_eq!(
            cpu.csrs.mscratch, 0xF0F0F0F0,
            "CSRRC must AND ~rs1 into CSR"
        );
    }

    #[test]
    fn test_csrrc_does_not_set_bits() {
        // CSRRC only clears bits, never sets them.
        // Bits that are 0 in rs1 must be unchanged in the CSR.
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0x00000000;
        cpu.regs[5] = 0xFFFFFFFF;

        cpu.execute(Instruction::Csrrc {
            csr: 0x340,
            rs1: 5,
            rd: 0,
        });

        assert_eq!(
            cpu.csrs.mscratch, 0x00000000,
            "CSRRC on zero CSR must stay zero"
        );
    }

    #[test]
    fn test_csrrc_returns_old_value_before_clear() {
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0xFFFFFFFF;
        cpu.regs[6] = 0x0F0F0F0F;

        cpu.execute(Instruction::Csrrc {
            csr: 0x340,
            rs1: 6,
            rd: 7,
        });

        assert_eq!(
            cpu.regs[7], 0xFFFFFFFF,
            "CSRRC must return OLD value (0xFFFFFFFF), not new value (0xF0F0F0F0)"
        );
    }

    #[test]
    fn test_csrrc_rs1_zero_is_pure_read() {
        // When rs1 = x0, CSRRC reads without modifying (no bits to clear).
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0xDEADBEEF;

        cpu.execute(Instruction::Csrrc {
            csr: 0x340,
            rs1: 0,
            rd: 5,
        });

        assert_eq!(
            cpu.regs[5], 0xDEADBEEF,
            "CSRRC with rs1=x0 must read CSR unchanged"
        );
        assert_eq!(
            cpu.csrs.mscratch, 0xDEADBEEF,
            "CSRRC with rs1=x0 must not modify CSR"
        );
    }

    // =========================================================================
    // CSRRWI tests
    // =========================================================================
    // CSRRWI: rd = old CSR value, CSR = zero_extend(uimm5)
    // The immediate is 5-bit ZERO-extended. Range: 0-31. Never negative.
    // =========================================================================

    #[test]
    fn test_csrrwi_writes_immediate() {
        // csrrwi with imm=0 → CSR must become 0
        // (This is the first CSRRWI test in section 8)
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0xF0F0F0F0; // pre-load so we can verify overwrite

        cpu.execute(Instruction::Csrrwi {
            csr: 0x340,
            uimm: 0,
            rd: 7, // capture old value
        });

        assert_eq!(
            cpu.csrs.mscratch, 0,
            "CSRRWI with imm=0 must write 0 to CSR"
        );
    }

    #[test]
    fn test_csrrwi_writes_max_immediate() {
        // Max 5-bit immediate = 31 = 0b11111
        let mut cpu = Cpu::new();

        cpu.execute(Instruction::Csrrwi {
            csr: 0x340,
            uimm: 31,
            rd: 0,
        });

        assert_eq!(
            cpu.csrs.mscratch, 31,
            "CSRRWI with imm=31 must write 31 to CSR"
        );
    }

    #[test]
    fn test_csrrwi_immediate_is_zero_extended() {
        // The immediate is ZERO-extended, not sign-extended.
        // Max value is 31 (5 bits). Bit 4 being set does NOT mean negative.
        // After CSRRWI with imm=31, CSR must be 0x0000001F, not 0xFFFFFFFF.
        let mut cpu = Cpu::new();

        cpu.execute(Instruction::Csrrwi {
            csr: 0x340,
            uimm: 31, // 0b11111 — if sign-extended = -1 = 0xFFFFFFFF (wrong)
            rd: 0,
        });

        assert_eq!(
            cpu.csrs.mscratch, 0x0000001F,
            "CSRRWI immediate must be ZERO-extended, not sign-extended. Expected 31, not 0xFFFFFFFF"
        );
    }

    #[test]
    fn test_csrrwi_returns_old_value() {
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0xABCDEF00;

        cpu.execute(Instruction::Csrrwi {
            csr: 0x340,
            uimm: 5,
            rd: 6,
        });

        assert_eq!(
            cpu.regs[6], 0xABCDEF00,
            "CSRRWI must return OLD CSR value in rd"
        );
        assert_eq!(cpu.csrs.mscratch, 5, "CSRRWI must write immediate to CSR");
    }

    // =========================================================================
    // CSRRSI tests
    // =========================================================================
    // CSRRSI: rd = old CSR value, CSR = CSR | zero_extend(uimm5)
    // Sets bits indicated by the 5-bit zero-extended immediate.
    // =========================================================================

    #[test]
    fn test_csrrsi_sets_bits() {
        // Section 8 sequence: CSR was just written to 0 by CSRRWI,
        // then CSRRSI with imm=31 sets bits 4:0 → CSR = 31
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0;

        cpu.execute(Instruction::Csrrsi {
            csr: 0x340,
            uimm: 31, // set bits 4:0
            rd: 7,
        });

        assert_eq!(
            cpu.csrs.mscratch, 31,
            "CSRRSI with imm=31 must set bits 4:0"
        );
    }

    #[test]
    fn test_csrrsi_does_not_clear_bits() {
        // CSRRSI only sets bits indicated by uimm, never clears others.
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0xFFFFFF00; // upper bits already set

        cpu.execute(Instruction::Csrrsi {
            csr: 0x340,
            uimm: 31, // set bits 4:0, upper bits must stay
            rd: 0,
        });

        assert_eq!(
            cpu.csrs.mscratch, 0xFFFFFF1F,
            "CSRRSI must not clear existing bits"
        );
    }

    #[test]
    fn test_csrrsi_returns_old_value() {
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0;

        cpu.execute(Instruction::Csrrsi {
            csr: 0x340,
            uimm: 31,
            rd: 7,
        });

        assert_eq!(
            cpu.regs[7], 0,
            "CSRRSI rd must hold OLD value (0), not new value (31)"
        );
    }

    #[test]
    fn test_csrrsi_uimm_zero_is_pure_read() {
        // CSRRSI with uimm=0 sets no bits → pure read
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0x12345678;

        cpu.execute(Instruction::Csrrsi {
            csr: 0x340,
            uimm: 0,
            rd: 5,
        });

        assert_eq!(
            cpu.regs[5], 0x12345678,
            "CSRRSI uimm=0 must read without modifying"
        );
        assert_eq!(
            cpu.csrs.mscratch, 0x12345678,
            "CSRRSI uimm=0 must not modify CSR"
        );
    }

    // =========================================================================
    // CSRRCI tests
    // =========================================================================
    // CSRRCI: rd = old CSR value, CSR = CSR & ~zero_extend(uimm5)
    // Clears bits indicated by the 5-bit zero-extended immediate.
    // =========================================================================

    #[test]
    fn test_csrrci_clears_bits() {
        // Section 8 sequence:
        // mscratch = 31 (0b11111) after CSRRSI
        // CSRRCI with imm=15 (0b01111) clears bits 3:0
        // result = 31 & ~15 = 0b11111 & 0b10000 = 16
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 31; // 0b11111

        cpu.execute(Instruction::Csrrci {
            csr: 0x340,
            uimm: 15, // 0b01111 — clear bits 3:0
            rd: 7,
        });

        assert_eq!(cpu.csrs.mscratch, 16, "CSRRCI: 31 & ~15 must equal 16");
    }

    #[test]
    fn test_csrrci_returns_old_value() {
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 31;

        cpu.execute(Instruction::Csrrci {
            csr: 0x340,
            uimm: 15,
            rd: 7,
        });

        assert_eq!(
            cpu.regs[7], 31,
            "CSRRCI rd must hold OLD value (31), not new value (16)"
        );
    }

    #[test]
    fn test_csrrci_does_not_set_bits() {
        // CSRRCI only clears bits, never sets.
        // Bits that are 0 in uimm must be unchanged.
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0x00000000;

        cpu.execute(Instruction::Csrrci {
            csr: 0x340,
            uimm: 31,
            rd: 0,
        });

        assert_eq!(cpu.csrs.mscratch, 0, "CSRRCI on zero CSR must stay zero");
    }

    #[test]
    fn test_csrrci_uimm_zero_is_pure_read() {
        let mut cpu = Cpu::new();
        cpu.csrs.mscratch = 0xABCD;

        cpu.execute(Instruction::Csrrci {
            csr: 0x340,
            uimm: 0,
            rd: 5,
        });

        assert_eq!(
            cpu.regs[5], 0xABCD,
            "CSRRCI uimm=0 must read without modifying"
        );
        assert_eq!(
            cpu.csrs.mscratch, 0xABCD,
            "CSRRCI uimm=0 must not modify CSR"
        );
    }

    // =========================================================================
    // Full section 8 sequence — mirrors the binary exactly
    // Run this after individual tests pass to confirm the whole flow works.
    // =========================================================================

    #[test]
    fn test_section8_full_sequence() {
        let mut cpu = Cpu::new();

        // --- csrw mscratch, t0  (t0 = 0xABCD1234) ---
        // csrw = csrrw x0, csr, rs1
        cpu.regs[5] = 0xABCD1234; // t0 = x5
        cpu.execute(Instruction::Csrrw {
            csr: 0x340,
            rs1: 5,
            rd: 0,
        });

        // --- csrr t1, mscratch ---
        // csrr = csrrs rd, csr, x0
        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 0,
            rd: 6,
        }); // t1 = x6
        assert_eq!(cpu.regs[6], 0xABCD1234, "step 1: csrr read-back failed");

        // --- csrw mscratch, t0  (t0 = 0x000000FF) ---
        cpu.regs[5] = 0x000000FF;
        cpu.execute(Instruction::Csrrw {
            csr: 0x340,
            rs1: 5,
            rd: 0,
        });

        // --- csrrs t2, mscratch, t1  (t1 = 0xFFFFFF00) ---
        // t2 must get OLD value; mscratch must become 0xFFFFFFFF
        cpu.regs[6] = 0xFFFFFF00; // t1 = x6
        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 6,
            rd: 7,
        }); // t2 = x7
        assert_eq!(cpu.regs[7], 0x000000FF, "step 2: CSRRS old value wrong");
        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 0,
            rd: 7,
        }); // read back
        assert_eq!(cpu.regs[7], 0xFFFFFFFF, "step 2: CSRRS new value wrong");

        // --- csrrc t2, mscratch, t1  (t1 = 0x0F0F0F0F) ---
        // t2 must get OLD value (0xFFFFFFFF); mscratch must become 0xF0F0F0F0
        cpu.regs[6] = 0x0F0F0F0F; // t1
        cpu.execute(Instruction::Csrrc {
            csr: 0x340,
            rs1: 6,
            rd: 7,
        }); // t2
        assert_eq!(cpu.regs[7], 0xFFFFFFFF, "step 3: CSRRC old value wrong");
        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 0,
            rd: 7,
        }); // read back
        assert_eq!(cpu.regs[7], 0xF0F0F0F0, "step 3: CSRRC new value wrong");

        // --- csrrwi t2, mscratch, 0 ---
        cpu.execute(Instruction::Csrrwi {
            csr: 0x340,
            uimm: 0,
            rd: 7,
        });
        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 0,
            rd: 7,
        }); // read back
        assert_eq!(cpu.regs[7], 0, "step 4: CSRRWI imm=0 failed");

        // --- csrrsi t2, mscratch, 31 ---
        cpu.execute(Instruction::Csrrsi {
            csr: 0x340,
            uimm: 31,
            rd: 7,
        });
        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 0,
            rd: 7,
        }); // read back
        assert_eq!(cpu.regs[7], 31, "step 5: CSRRSI imm=31 failed");

        // --- csrrci t2, mscratch, 15 ---
        // 31 & ~15 = 16
        cpu.execute(Instruction::Csrrci {
            csr: 0x340,
            uimm: 15,
            rd: 7,
        });
        cpu.execute(Instruction::Csrrs {
            csr: 0x340,
            rs1: 0,
            rd: 7,
        }); // read back
        assert_eq!(
            cpu.regs[7], 16,
            "step 6: CSRRCI imm=15 failed (expected 31 & ~15 = 16)"
        );
    }
}
