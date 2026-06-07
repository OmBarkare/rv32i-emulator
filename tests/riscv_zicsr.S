# =============================================================================
# rv32i_zicsr.S
# Comprehensive test for: RV32I base ISA + Zicsr + ECALL + EBREAK
#
# HOW THIS WORKS:
#   - Binary is loaded at address 0x0000 and PC starts at 0x0000
#   - s0 (x8) is a section pass counter. After each section passes,
#     we do "addi s0, s0, 1". If something fails we jump to _fail.
#     On halt, s0 tells you exactly how many sections passed.
#   - On PASS: a0 = 0x900D900D, PC halts at _pass
#   - On FAIL: a0 = 0xDEADDEAD, PC halts at _fail
#     Check s0 to see which section failed.
#
# MEMORY LAYOUT:
#   0x0000 - 0x0EFF : test code
#   0x0E00 - 0x0EFF : scratch memory for load/store tests
#   0x0F00          : _trap_vec (trap handler)
#   0x0F80          : _fail
#   0x0FF0          : _pass
#
# TRAP HANDLER BEHAVIOR (matches your emulator):
#   - mepc = address of trapping instruction (PC - 4 in your impl)
#   - mret = jump to mepc + 4 (returns to instruction after trap)
#   - handler saves mcause into a1 so tests can inspect it
#
# SECTION MAP (s0 value = sections completed before failure):
#   s0 = 0  : failed in section 1  (LUI / AUIPC)
#   s0 = 1  : failed in section 2  (ADDI and immediate arithmetic)
#   s0 = 2  : failed in section 3  (shifts)
#   s0 = 3  : failed in section 4  (R-type register ops)
#   s0 = 4  : failed in section 5  (loads and stores)
#   s0 = 5  : failed in section 6  (JAL / JALR)
#   s0 = 6  : failed in section 7  (branches)
#   s0 = 7  : failed in section 8  (CSR instructions)
#   s0 = 8  : failed in section 9  (EBREAK)
#   s0 = 9  : failed in section 10 (ECALL)
#   s0 = 10 : failed in section 11 (x0 hardwired zero)
#   s0 = 11 : failed in section 12 (immediate edge cases)
# =============================================================================

    .section .text
    .globl _start
    .org 0x000

# =============================================================================
_start:
# =============================================================================

    # Zero out every register we will use.
    # In RISC-V, x0 is always 0 (hardwired), but x1-x31 are not
    # guaranteed to be 0 on reset in real hardware. We zero them
    # explicitly so tests start from a known state.
    li x1,  0
    li x2,  0
    li x3,  0
    li x4,  0
    li x5,  0       # t0
    li x6,  0       # t1
    li x7,  0       # t2
    li x8,  0       # s0 = section pass counter
    li x9,  0       # s1
    li x10, 0       # a0
    li x11, 0       # a1 = trap handler writes mcause here
    li x12, 0       # a2
    li x13, 0
    li x14, 0
    li x15, 0       # a5 = used as base addr in load/store tests

    # Install our trap handler.
    # la = Load Address. It's a pseudo-instruction that expands to
    # AUIPC + ADDI, loading the address of _trap_vec into t0.
    la   t0, _trap_vec

    # csrw mtvec, t0 = write t0 into the mtvec CSR.
    # mtvec holds the address the CPU jumps to when a trap occurs.
    # Without this, traps would jump to 0x0000 (mtvec default = 0)
    # which would restart _start — very bad.
    csrw mtvec, t0


# =============================================================================
# SECTION 1: LUI and AUIPC
# =============================================================================
# LUI (Load Upper Immediate):
#   Loads a 20-bit immediate into bits[31:12] of a register.
#   bits[11:0] are set to 0.
#   Example: lui t0, 1  →  t0 = 0x00001000
#
# AUIPC (Add Upper Immediate to PC):
#   Same as LUI but adds the current PC to the result.
#   Used to compute PC-relative addresses.
#   Example: if PC=0x100, auipc t0, 1  →  t0 = 0x100 + 0x1000 = 0x1100
# =============================================================================

    # --- LUI basic test ---
    lui  t0, 1              # t0 = 1 << 12 = 0x00001000
    li   t1, 0x1000         # t1 = expected value = 0x1000
    bne  t0, t1, _fail      # if t0 != t1, something is wrong with LUI

    # --- LUI with large immediate ---
    lui  t0, 0xFFFFF        # t0 = 0xFFFFF000 (all upper bits set)
    li   t1, 0xFFFFF000     # expected
    bne  t0, t1, _fail

    # --- AUIPC test ---
    # We use two consecutive AUIPC instructions to verify PC-relative behavior.
    # The trick: two AUIPCs 4 bytes apart, with upper immediates differing by 1.
    # Expected difference = 0x1000 (from imm) + 0x4 (from PC difference) = 0x1004
    #
    # Let's say first AUIPC is at address X:
    #   t0 = X + (0 << 12) = X
    #   t1 = (X+4) + (1 << 12) = X + 4 + 0x1000 = X + 0x1004
    #   t1 - t0 = 0x1004
    auipc t0, 0             # t0 = PC of this instruction (call it X)
    auipc t1, 1             # t1 = (X+4) + 0x1000 = X + 0x1004
    sub   t2, t1, t0        # t2 = t1 - t0 = 0x1004
    li    t3, 0x1004        # expected difference
    bne   t2, t3, _fail

    addi s0, s0, 1          # SECTION 1 PASSED. s0 = 1


# =============================================================================
# SECTION 2: ADDI and immediate arithmetic (I-type)
# =============================================================================
# I-type instructions take one register source and a 12-bit signed immediate.
# The immediate is sign-extended to 32 bits before use.
# Format: INSTR rd, rs1, imm  →  rd = rs1 OP sign_extend(imm)
# =============================================================================

    # --- ADDI basic ---
    li   t0, 0              # t0 = 0
    addi t0, t0, 42         # t0 = 0 + 42 = 42
    li   t1, 42
    bne  t0, t1, _fail

    # --- ADDI with negative immediate ---
    # Immediates in RISC-V are 12-bit signed, so range is -2048 to +2047
    li   t0, 100
    addi t0, t0, -1         # t0 = 100 + (-1) = 99
    li   t1, 99
    bne  t0, t1, _fail

    # --- SLTI (Set Less Than Immediate, signed) ---
    # rd = (rs1 < sign_extend(imm)) ? 1 : 0
    # Both operands treated as SIGNED
    li   t0, -1
    slti t1, t0, 0          # is -1 < 0 ? yes → t1 = 1
    li   t2, 1
    bne  t1, t2, _fail

    li   t0, 5
    slti t1, t0, 5          # is 5 < 5 ? no → t1 = 0
    bne  t1, x0, _fail      # x0 is always 0, using it as "expected 0"

    # --- SLTIU (Set Less Than Immediate, Unsigned) ---
    # Same but both operands treated as UNSIGNED.
    # The immediate is still sign-extended first, THEN treated as unsigned.
    # So sltiu t1, t0, -1 compares t0 against 0xFFFFFFFF unsigned.
    li   t0, -1             # t0 = 0xFFFFFFFF (unsigned: largest 32-bit value)
    sltiu t1, t0, 1         # is 0xFFFFFFFF < 1 unsigned? no → t1 = 0
    bne  t1, x0, _fail

    li   t0, 0
    sltiu t1, t0, 1         # is 0 < 1 unsigned? yes → t1 = 1
    li   t2, 1
    bne  t1, t2, _fail

    # --- XORI ---
    # Bitwise XOR with immediate. Bit-by-bit: 1 if bits differ, 0 if same.
    li   t0, 0xFF           # t0 = 0b11111111
    xori t0, t0, 0x0F       # XOR with 0b00001111 → 0b11110000 = 0xF0
    li   t1, 0xF0
    bne  t0, t1, _fail

    # --- ORI ---
    li   t0, 0x00
    ori  t0, t0, 0xAB       # OR with 0xAB → 0xAB
    li   t1, 0xAB
    bne  t0, t1, _fail

    # --- ANDI ---
    li   t0, 0xFF
    andi t0, t0, 0x55       # AND with 0x55 = 0b01010101 → 0x55
    li   t1, 0x55
    bne  t0, t1, _fail

    addi s0, s0, 1          # SECTION 2 PASSED. s0 = 2


# =============================================================================
# SECTION 3: Shift instructions
# =============================================================================
# Three shift types:
#   SLLI / SLL : Shift Left Logical  — fills right with 0s
#   SRLI / SRL : Shift Right Logical — fills left with 0s
#   SRAI / SRA : Shift Right Arithmetic — fills left with sign bit
#
# Immediate shifts use a 5-bit shamt (shift amount) in bits[24:20].
# Register shifts use only the low 5 bits of rs2 as the shift amount.
# This means you can only shift by 0-31, which makes sense for 32-bit regs.
# =============================================================================

    # --- SLLI (Shift Left Logical Immediate) ---
    li   t0, 1
    slli t0, t0, 4          # t0 = 1 << 4 = 16 = 0x10
    li   t1, 16
    bne  t0, t1, _fail

    # --- SRLI (Shift Right Logical Immediate) ---
    # LOGICAL shift: vacated bits on the LEFT are filled with 0.
    # So even if bit 31 was 1, after logical right shift it becomes 0.
    li   t0, 0x80000000     # t0 = 1000_0000_0000_0000_0000_0000_0000_0000
    srli t0, t0, 1          # t0 = 0100_0000_0000_0000_0000_0000_0000_0000 = 0x40000000
    li   t1, 0x40000000
    bne  t0, t1, _fail

    # --- SRAI (Shift Right Arithmetic Immediate) ---
    # ARITHMETIC shift: vacated bits on the LEFT are filled with the SIGN BIT.
    # If bit 31 was 1 (negative number), it stays 1 after shifting.
    # This is how signed division by powers of 2 works.
    li   t0, 0x80000000     # t0 = most negative 32-bit number (-2147483648)
    srai t0, t0, 1          # arithmetic right shift: sign bit propagates
                            # result = 1100_0000_... = 0xC0000000
    li   t1, 0xC0000000
    bne  t0, t1, _fail

    # --- SLL (register shift) ---
    # Same as SLLI but shift amount comes from a register (low 5 bits only)
    li   t0, 1
    li   t1, 10             # shift amount = 10
    sll  t2, t0, t1         # t2 = 1 << 10 = 1024
    li   t3, 1024
    bne  t2, t3, _fail

    # --- SRL (register shift, logical) ---
    li   t0, 0x80000000
    li   t1, 1
    srl  t2, t0, t1         # 0x80000000 >> 1 logical = 0x40000000
    li   t3, 0x40000000
    bne  t2, t3, _fail

    # --- SRA (register shift, arithmetic) ---
    li   t0, 0x80000000
    li   t1, 1
    sra  t2, t0, t1         # 0x80000000 >> 1 arithmetic = 0xC0000000
    li   t3, 0xC0000000
    bne  t2, t3, _fail

    # --- Shift amount masking ---
    # The shift amount is masked to 5 bits, so shifting by 32 = shifting by 0.
    # Shifting by 33 = shifting by 1, etc.
    li   t0, 1
    li   t1, 32             # 32 in binary = 100000, low 5 bits = 00000
    sll  t2, t0, t1         # effectively shift by 0 → t2 = 1
    li   t3, 1
    bne  t2, t3, _fail

    addi s0, s0, 1          # SECTION 3 PASSED. s0 = 3


# =============================================================================
# SECTION 4: R-type register-register instructions
# =============================================================================
# R-type instructions take two register sources and write to a destination.
# Format: INSTR rd, rs1, rs2  →  rd = rs1 OP rs2
# No immediate operand — both inputs come from registers.
# =============================================================================

    # --- ADD ---
    li   t0, 15
    li   t1, 9
    add  t2, t0, t1         # t2 = 15 + 9 = 24
    li   t3, 24
    bne  t2, t3, _fail

    # --- ADD overflow wrapping ---
    # RISC-V integer overflow is defined: it wraps silently (no trap, no flag).
    # 0x7FFFFFFF is the largest positive 32-bit signed integer.
    # Adding 1 wraps to 0x80000000 (which is the most negative signed int).
    li   t0, 0x7FFFFFFF
    li   t1, 1
    add  t2, t0, t1         # wraps: 0x7FFFFFFF + 1 = 0x80000000
    li   t3, 0x80000000
    bne  t2, t3, _fail

    # --- SUB ---
    li   t0, 15
    li   t1, 9
    sub  t2, t0, t1         # t2 = 15 - 9 = 6
    li   t3, 6
    bne  t2, t3, _fail

    # --- SUB underflow wrapping ---
    li   t0, 0              # 0 - 1 wraps to 0xFFFFFFFF
    li   t1, 1
    sub  t2, t0, t1
    li   t3, 0xFFFFFFFF
    bne  t2, t3, _fail

    # --- SLT (Set Less Than, signed) ---
    # rd = (rs1 < rs2 signed) ? 1 : 0
    li   t0, -5
    li   t1, 3
    slt  t2, t0, t1         # signed: -5 < 3 → 1
    li   t3, 1
    bne  t2, t3, _fail

    li   t0, 3
    li   t1, 3
    slt  t2, t0, t1         # 3 < 3 → 0 (equal, not less)
    bne  t2, x0, _fail

    # --- SLTU (Set Less Than, unsigned) ---
    # -5 as unsigned = 0xFFFFFFFB which is a huge number.
    # So 3 < 0xFFFFFFFB unsigned → 1
    li   t0, 3
    li   t1, -5             # 0xFFFFFFFB unsigned
    sltu t2, t0, t1         # unsigned: 3 < 0xFFFFFFFB → 1
    li   t3, 1
    bne  t2, t3, _fail

    # --- XOR ---
    li   t0, 0xAAAA         # 1010_1010_1010_1010
    li   t1, 0x5555         # 0101_0101_0101_0101
    xor  t2, t0, t1         # 1111_1111_1111_1111 = 0xFFFF
    li   t3, 0xFFFF
    bne  t2, t3, _fail

    # --- OR ---
    li   t0, 0xA0
    li   t1, 0x0B
    or   t2, t0, t1         # 0xA0 | 0x0B = 0xAB
    li   t3, 0xAB
    bne  t2, t3, _fail

    # --- AND ---
    li   t0, 0xFF
    li   t1, 0x0F
    and  t2, t0, t1         # 0xFF & 0x0F = 0x0F
    li   t3, 0x0F
    bne  t2, t3, _fail

    addi s0, s0, 1          # SECTION 4 PASSED. s0 = 4


# =============================================================================
# SECTION 5: Loads and Stores
# =============================================================================
# RISC-V has 5 load instructions and 3 store instructions.
# All addresses are computed as: base_register + sign_extended_offset
#
# Stores: SB (byte), SH (halfword=2bytes), SW (word=4bytes)
# Loads:  LB, LH (sign-extended), LBU, LHU (zero-extended), LW
#
# RISC-V is LITTLE ENDIAN: the least significant byte is at the lowest address.
# So if you store 0x04030201 at address X:
#   X+0 = 0x01 (least significant byte)
#   X+1 = 0x02
#   X+2 = 0x03
#   X+3 = 0x04 (most significant byte)
#
# We use address 0x0E00 as our scratch memory area.
# =============================================================================

    li   a5, 0x0E00         # a5 = base address for load/store scratch area

    # --- SW / LW (store/load full 32-bit word) ---
    li   t0, 0xDEADBEEF
    sw   t0, 0(a5)          # store 0xDEADBEEF at 0x0E00
    lw   t1, 0(a5)          # load it back
    bne  t0, t1, _fail      # must be identical

    # --- SB / LB (store byte, load with SIGN EXTENSION) ---
    # When you load a byte with LB, bit 7 of the byte is extended to fill bits[31:8].
    # 0xAB = 1010_1011. Bit 7 = 1, so it sign-extends to 0xFFFFFFAB.
    # As a signed 32-bit integer, 0xFFFFFFAB = -85.
    li   t0, 0xAB
    sb   t0, 4(a5)          # store byte 0xAB at 0x0E04
    lb   t1, 4(a5)          # load byte with sign extension → 0xFFFFFFAB
    li   t2, -85            # -85 = 0xFFFFFFAB (same bit pattern)
    bne  t1, t2, _fail

    # --- LBU (load byte with ZERO EXTENSION) ---
    # Same byte, but bits[31:8] are filled with 0 instead of the sign bit.
    lbu  t1, 4(a5)          # load byte zero-extended → 0x000000AB
    li   t2, 0xAB
    bne  t1, t2, _fail

    # --- SH / LH (store halfword=2bytes, load with SIGN EXTENSION) ---
    # 0x8000 = 1000_0000_0000_0000. Bit 15 = 1, sign-extends to 0xFFFF8000.
    li   t0, 0x8000
    sh   t0, 8(a5)          # store halfword 0x8000 at 0x0E08
    lh   t1, 8(a5)          # load with sign extension → 0xFFFF8000
    li   t2, 0xFFFF8000
    bne  t1, t2, _fail

    # --- LHU (load halfword with ZERO EXTENSION) ---
    lhu  t1, 8(a5)          # load zero-extended → 0x00008000
    li   t2, 0x8000
    bne  t1, t2, _fail

    # --- Little-endian byte layout verification ---
    # Store 4 individual bytes and read back as a word.
    # In little-endian: byte at lowest address = least significant byte of word.
    li   t0, 0x01
    sb   t0, 12(a5)         # addr 0x0E0C = 0x01 (least significant byte)
    li   t0, 0x02
    sb   t0, 13(a5)         # addr 0x0E0D = 0x02
    li   t0, 0x03
    sb   t0, 14(a5)         # addr 0x0E0E = 0x03
    li   t0, 0x04
    sb   t0, 15(a5)         # addr 0x0E0F = 0x04 (most significant byte)
    lw   t1, 12(a5)         # read as word → 0x04030201
    li   t2, 0x04030201
    bne  t1, t2, _fail

    addi s0, s0, 1          # SECTION 5 PASSED. s0 = 5


# =============================================================================
# SECTION 6: JAL and JALR
# =============================================================================
# JAL (Jump And Link):
#   rd = PC + 4             (save return address = address of NEXT instruction)
#   PC = PC + sign_extend(imm)   (jump to PC-relative target)
#
# JALR (Jump And Link Register):
#   rd = PC + 4             (same return address behavior)
#   PC = (rs1 + sign_extend(imm)) & ~1   (jump to register-relative target)
#   The & ~1 forces the LSB to 0, ensuring alignment.
#
# HOW WE TEST RETURN ADDRESS:
#   We need to know what value rd should hold after JAL/JALR.
#   The return address = address of the instruction immediately after JAL/JALR.
#   We use AUIPC to discover the current PC, then compute the expected
#   return address by counting bytes forward.
# =============================================================================

    # --- JAL return address test ---
    #
    # We need to know the address of the JAL instruction itself.
    # AUIPC with imm=0 gives us the PC of the AUIPC instruction.
    # Then we count 4 bytes forward to JAL, and 4 more for the return address.
    #
    # Layout (each instruction = 4 bytes):
    #   auipc t1, 0       ← t1 = address of THIS line (call it A)
    #   jal   t0, jal_land  ← at address A+4. rd=t0 gets A+8. jumps to jal_land.
    #   j     _fail       ← at address A+8. skipped if JAL works.
    #   jal_land:         ← at address A+12. execution resumes here.
    #     addi t2,t1,8    ← compute expected return addr = A + 8
    #     bne  t0,t2,_fail ← t0 must equal A+8

    auipc t1, 0             # t1 = address of this AUIPC instruction = A
    jal   t0, jal_land      # t0 = A+8 (return addr), jump to jal_land
    j     _fail             # this is at A+8, skipped if JAL taken correctly
jal_land:                   # this is at A+12, execution resumes here
    addi  t2, t1, 8         # expected return address = A + 8
    bne   t0, t2, _fail     # t0 must equal A+8

    # --- JALR return address and indirect jump test ---
    #
    # Layout:
    #   auipc t1, 0         ← t1 = address of THIS line (call it B)
    #   la    t3, jalr_land ← load address of jalr_land into t3
    #   jalr  t0, t3, 0    ← t0 = B+12 (return addr), jump to t3+0=jalr_land
    #   j     _fail        ← skipped
    #   jalr_land:
    #     addi t2, t1, 12  ← expected return addr = B+12
    #     bne  t0, t2, _fail

    auipc t1, 0             # t1 = address of this AUIPC = B
                            # NOTE: 'la' expands to 2 instructions (auipc+addi)
                            # so it takes 8 bytes, not 4!
    la    t3, jalr_land     # t3 = address of jalr_land (2 instructions = 8 bytes)
    jalr  t0, t3, 0         # t0 = B+16 (return addr), PC = t3 = jalr_land
                            # wait, la is 2 instrs (8 bytes), so jalr is at B+8
                            # return addr = B+8+4 = B+12
    j     _fail             # skipped if JALR works
jalr_land:
    # auipc was at B, la took 8 bytes (B+4, B+8), jalr at B+12
    # return address = B+12+4 = B+16... let me recount carefully:
    # B+0:  auipc t1,0
    # B+4:  (la part 1: auipc)
    # B+8:  (la part 2: addi)
    # B+12: jalr t0, t3, 0   → return addr stored = B+12+4 = B+16
    # B+16: j _fail           (skipped)
    # B+20: jalr_land:
    addi  t2, t1, 16        # expected return addr = B + 16
    bne   t0, t2, _fail

    addi s0, s0, 1          # SECTION 6 PASSED. s0 = 6


# =============================================================================
# SECTION 7: Branches
# =============================================================================
# All 6 branch instructions, tested both TAKEN and NOT TAKEN.
# Format: Bxx rs1, rs2, label
#   If condition is true: PC = PC + sign_extend(imm)  (taken)
#   If condition is false: PC = PC + 4                (not taken, normal flow)
#
# For each branch we test:
#   1. The case where it SHOULD jump (taken) → if it doesn't, we fall to _fail
#   2. The case where it should NOT jump (not taken) → if it does, we jump to _fail
# =============================================================================

    # --- BEQ (Branch if EQual) ---
    li   t0, 7
    li   t1, 7
    beq  t0, t1, beq_taken  # 7 == 7 → should jump
    j    _fail               # if we reach here, BEQ didn't take → wrong
beq_taken:
    li   t0, 7
    li   t1, 8
    beq  t0, t1, _fail      # 7 != 8 → should NOT jump, if it does → wrong

    # --- BNE (Branch if Not Equal) ---
    li   t0, 7
    li   t1, 8
    bne  t0, t1, bne_taken  # 7 != 8 → should jump
    j    _fail
bne_taken:
    li   t0, 5
    li   t1, 5
    bne  t0, t1, _fail      # 5 == 5 → should NOT jump

    # --- BLT (Branch if Less Than, SIGNED) ---
    li   t0, -1             # -1 in signed
    li   t1, 0
    blt  t0, t1, blt_taken  # signed: -1 < 0 → should jump
    j    _fail
blt_taken:
    li   t0, 1
    li   t1, 0
    blt  t0, t1, _fail      # signed: 1 > 0 → should NOT jump

    # --- BGE (Branch if Greater or Equal, SIGNED) ---
    li   t0, 0
    li   t1, 0
    bge  t0, t1, bge_taken_eq  # 0 >= 0 → equal case, should jump
    j    _fail
bge_taken_eq:
    li   t0, 1
    li   t1, -1
    bge  t0, t1, bge_taken_gt  # signed: 1 >= -1 → greater case, should jump
    j    _fail
bge_taken_gt:
    li   t0, -1
    li   t1, 1
    bge  t0, t1, _fail      # signed: -1 < 1 → should NOT jump

    # --- BLTU (Branch if Less Than, UNSIGNED) ---
    # Key insight: -1 as unsigned = 0xFFFFFFFF = largest 32-bit unsigned value
    li   t0, 1
    li   t1, -1             # t1 = 0xFFFFFFFF unsigned
    bltu t0, t1, bltu_taken # unsigned: 1 < 0xFFFFFFFF → should jump
    j    _fail
bltu_taken:
    bltu t1, t0, _fail      # unsigned: 0xFFFFFFFF < 1 → false, should NOT jump

    # --- BGEU (Branch if Greater or Equal, UNSIGNED) ---
    bgeu t1, t0, bgeu_taken # unsigned: 0xFFFFFFFF >= 1 → should jump
    j    _fail
bgeu_taken:
    bgeu t0, t1, _fail      # unsigned: 1 >= 0xFFFFFFFF → false, should NOT jump

    addi s0, s0, 1          # SECTION 7 PASSED. s0 = 7


# =============================================================================
# SECTION 8: CSR Instructions
# =============================================================================
# CSRs = Control and Status Registers. Special registers separate from x0-x31.
# They control CPU behavior and hold status information.
#
# We test using mscratch (CSR address 0x340).
# mscratch is a scratch register — no hardware behavior, safe to read/write freely.
#
# The 6 CSR instructions:
#   CSRRW  rd, csr, rs1  : rd=old CSR value, CSR=rs1
#   CSRRS  rd, csr, rs1  : rd=old CSR value, CSR=CSR|rs1  (set bits)
#   CSRRC  rd, csr, rs1  : rd=old CSR value, CSR=CSR&~rs1 (clear bits)
#   CSRRWI rd, csr, uimm : same as CSRRW but rs1 is a 5-bit zero-extended immediate
#   CSRRSI rd, csr, uimm : same as CSRRS with immediate
#   CSRRCI rd, csr, uimm : same as CSRRC with immediate
#
# IMPORTANT: The OLD value is always returned in rd BEFORE the CSR is modified.
# =============================================================================

    # --- CSRW / CSRR (write then read back) ---
    # csrw is a pseudo for csrrw x0, csr, rs1 (discard old value)
    # csrr is a pseudo for csrrs rd, csr, x0  (read without modifying)
    li   t0, 0xABCD1234
    csrw mscratch, t0       # mscratch = 0xABCD1234
    csrr t1, mscratch       # t1 = mscratch = 0xABCD1234
    bne  t0, t1, _fail

    # --- CSRRS (read old value, SET bits) ---
    # Step 1: set mscratch to a known value
    li   t0, 0x000000FF
    csrw mscratch, t0       # mscratch = 0x000000FF

    # Step 2: use CSRRS to set upper bits, and capture the OLD value
    li   t1, 0xFFFFFF00     # these are the bits we want to SET
    csrrs t2, mscratch, t1  # t2 = OLD value of mscratch (before modification)
                            # mscratch = 0xFF | 0xFFFFFF00 = 0xFFFFFFFF

    # Check that t2 holds the OLD value
    li   t3, 0x000000FF
    bne  t2, t3, _fail      # old value must be 0xFF

    # Check that mscratch was correctly updated
    csrr t2, mscratch
    li   t3, 0xFFFFFFFF
    bne  t2, t3, _fail      # new value must be 0xFFFFFFFF

    # --- CSRRC (read old value, CLEAR bits) ---
    # mscratch is currently 0xFFFFFFFF
    li   t1, 0x0F0F0F0F     # bits to CLEAR
    csrrc t2, mscratch, t1  # t2 = OLD value (0xFFFFFFFF)
                            # mscratch = 0xFFFFFFFF & ~0x0F0F0F0F = 0xF0F0F0F0

    li   t3, 0xFFFFFFFF
    bne  t2, t3, _fail      # old value check

    csrr t2, mscratch
    li   t3, 0xF0F0F0F0
    bne  t2, t3, _fail      # new value check

    # --- CSRRWI (write immediate) ---
    # The immediate is 5-bit ZERO-extended (not sign-extended!), range 0-31.
    csrrwi t2, mscratch, 0  # mscratch = 0 (write immediate 0, discard old in t2)
    csrr   t2, mscratch
    bne    t2, x0, _fail    # mscratch must be 0

    # --- CSRRSI (set bits from immediate) ---
    csrrsi t2, mscratch, 31 # mscratch = 0 | 31 = 31 (set bits 4:0)
    csrr   t2, mscratch
    li     t3, 31
    bne    t2, t3, _fail

    # --- CSRRCI (clear bits from immediate) ---
    # mscratch = 31 = 0b11111
    # clear bits from imm 15 = 0b01111
    # result = 31 & ~15 = 0b11111 & 0b10000 = 0b10000 = 16
    csrrci t2, mscratch, 15 # mscratch = 31 & ~15 = 16
    csrr   t2, mscratch
    li     t3, 16
    bne    t2, t3, _fail

    addi s0, s0, 1          # SECTION 8 PASSED. s0 = 8


# =============================================================================
# SECTION 9: EBREAK
# =============================================================================
# EBREAK causes a breakpoint exception.
# In your emulator: mcause = 3, mepc = address of EBREAK instruction.
# The trap handler (at _trap_vec) captures mcause into a1, then mrets.
# mret in your emulator returns to mepc+4 = instruction after EBREAK.
# So after EBREAK + mret, execution continues at the next instruction.
# =============================================================================

    li   a1, 0              # clear a1 so we can verify the handler wrote to it
    ebreak                  # triggers trap → jumps to _trap_vec
                            # handler sets a1 = mcause = 3
                            # handler mrets → comes back HERE (next instruction)
    li   t0, 3              # expected mcause for EBREAK = 3
    bne  a1, t0, _fail      # if a1 != 3, trap didn't fire or mcause is wrong

    addi s0, s0, 1          # SECTION 9 PASSED. s0 = 9


# =============================================================================
# SECTION 10: ECALL
# =============================================================================
# ECALL causes an environment call exception.
# In M-mode: mcause = 11.
# Same trap flow as EBREAK — handler captures mcause into a1, mrets back.
# =============================================================================

    li   a1, 0              # clear a1
    ecall                   # triggers trap → jumps to _trap_vec
                            # handler sets a1 = mcause = 11
                            # handler mrets → comes back HERE
    li   t0, 11             # expected mcause for ECALL in M-mode = 11
    bne  a1, t0, _fail

    addi s0, s0, 1          # SECTION 10 PASSED. s0 = 10


# =============================================================================
# SECTION 11: x0 is hardwired zero
# =============================================================================
# x0 always reads as 0. Writes to x0 are silently discarded.
# This is fundamental to RISC-V — many pseudo-instructions rely on it.
# (e.g. "li t0, 0" is actually "addi t0, x0, 0")
# =============================================================================

    li   t0, 42
    li   t1, 42
    add  x0, t0, t1         # attempt to write 84 into x0 — must be discarded
    bne  x0, x0, _fail      # x0 == x0 is always true (sanity check)
    li   t2, 0
    bne  x0, t2, _fail      # x0 must still be 0, not 84

    addi s0, s0, 1          # SECTION 11 PASSED. s0 = 11


# =============================================================================
# SECTION 12: Immediate edge cases
# =============================================================================
# RISC-V I-type immediates are 12-bit SIGNED values, range -2048 to +2047.
# They are sign-extended to 32 bits before use.
# The assembler handles this for you, but your emulator must implement it.
#
# The 'li' pseudo-instruction handles values outside 12-bit range by using
# LUI + ADDI. But single-instruction ADDI is what we're testing here.
# =============================================================================

    # -1 = 0xFFFFFFFF (all bits set)
    li   t0, -1
    li   t1, 0xFFFFFFFF
    bne  t0, t1, _fail

    # -2048 = 0xFFFFF800 (minimum 12-bit signed immediate)
    li   t0, -2048
    li   t1, 0xFFFFF800
    bne  t0, t1, _fail

    # 2047 = 0x000007FF (maximum 12-bit signed immediate)
    li   t0, 2047
    li   t1, 0x7FF
    bne  t0, t1, _fail

    # The LUI+ADDI correction case:
    # When a 32-bit value has bit 11 set, LUI must add 1 to compensate
    # because ADDI will sign-extend the lower 12 bits (treating them as negative).
    # The assembler handles this automatically. We just verify the result.
    li   t0, 0x12345678     # requires LUI+ADDI, bit 11 of lower 12 = 1 (0x678 has bit 11=0 actually)
    li   t1, 0x12345678
    bne  t0, t1, _fail

    li   t0, 0x80000800     # bit 11 of lower 12 IS set (0x800), needs correction
    li   t1, 0x80000800
    bne  t0, t1, _fail

    addi s0, s0, 1          # SECTION 12 PASSED. s0 = 12


# =============================================================================
# ALL SECTIONS DONE
# =============================================================================

    # Verify all 12 sections passed
    li   t0, 12
    bne  s0, t0, _fail      # if s0 != 12, something is wrong with the counter

    j    _pass              # explicit jump to _pass


# =============================================================================
# TRAP HANDLER
# =============================================================================
# This is installed into mtvec at the start of _start.
# It handles EBREAK (mcause=3) and ECALL M-mode (mcause=11).
# Any other trap cause jumps to _fail (unexpected trap).
#
# YOUR EMULATOR BEHAVIOR (as you told me):
#   - mepc = address of trapping instruction (you store PC-4)
#   - mret = jump to mepc + 4
# So mret naturally returns to the instruction after ecall/ebreak.
# The handler does NOT need to manually advance mepc.
#
# REGISTERS USED:
#   a1 = mcause (read by the test after returning from trap)
#   t4 = scratch (used only inside handler, restored by caller convention)
# =============================================================================

    .org 0x0F00
_trap_vec:
    csrr a1, mcause         # read mcause into a1 (tests inspect this after mret)

    li   t4, 3
    beq  a1, t4, _trap_ret  # mcause=3 means EBREAK → handle it

    li   t4, 11
    beq  a1, t4, _trap_ret  # mcause=11 means ECALL M-mode → handle it

    j    _fail              # any other cause = unexpected trap = test failure

_trap_ret:
    mret                    # return to mepc+4 (instruction after ecall/ebreak)


# =============================================================================
# FAIL: halts with a0 = 0xDEADDEAD
# Check s0 to see which section failed.
# =============================================================================
    .org 0x0F80
_fail:
    li   a0, 0xDEADDEAD     # failure sentinel
.Lhalt_fail:
    jal  x0, .Lhalt_fail    # infinite self-loop = halt


# =============================================================================
# PASS: halts with a0 = 0x900D900D
# s0 = 12 means all sections passed.
# =============================================================================
    .org 0x0FF0
_pass:
    li   a0, 0x900D900D     # success sentinel
.Lhalt_pass:
    jal  x0, .Lhalt_pass    # infinite self-loop = halt

