.section .text
.global _start

_start:

# ── 1. Immediate arithmetic ───────────────────────────────────────────
    addi  x1,  x0,  100     # x1  = 100
    addi  x2,  x0,  -1      # x2  = 0xFFFFFFFF
    slti  x3,  x1,  200     # x3  = 1  (100 < 200)
    slti  x4,  x1,  50      # x4  = 0  (100 not < 50)
    sltiu x5,  x2,  1       # x5  = 0  (0xFFFFFFFF not <u 1)
    xori  x6,  x1,  0x0F    # x6  = 107 (0x64 ^ 0x0F)
    ori   x7,  x1,  0x0F    # x7  = 111 (0x64 | 0x0F)
    andi  x8,  x1,  0x0F    # x8  = 4   (0x64 & 0x0F)
    slli  x9,  x1,  2       # x9  = 400
    srli  x10, x1,  2       # x10 = 25
    srai  x11, x2,  4       # x11 = -1  (arithmetic shift keeps sign)

# ── 2. LUI / AUIPC ────────────────────────────────────────────────────
    lui   x12, 1            # x12 = 0x00001000
    auipc x13, 0            # x13 = PC of this instruction (0x30 = 48)

# ── 3. R-type ─────────────────────────────────────────────────────────
    addi  x1,  x0, 12       # x1 = 12 = 0b1100
    addi  x2,  x0, 10       # x2 = 10 = 0b1010
    add   x14, x1, x2       # x14 = 22
    sub   x15, x1, x2       # x15 = 2
    and   x16, x1, x2       # x16 = 8  (0b1100 & 0b1010)
    or    x17, x1, x2       # x17 = 14 (0b1100 | 0b1010)
    xor   x18, x1, x2       # x18 = 6  (0b1100 ^ 0b1010)
    slt   x19, x2, x1       # x19 = 1  (10 < 12)
    sltu  x20, x2, x1       # x20 = 1  (10 <u 12)
    addi  x3,  x0, 2
    sll   x21, x1, x3       # x21 = 48  (12 << 2)
    srl   x22, x1, x3       # x22 = 3   (12 >> 2)
    addi  x4,  x0, -12
    sra   x23, x4, x3       # x23 = -3  (-12 >> 2 arithmetic)

# ── 4. Memory ─────────────────────────────────────────────────────────
    addi  x24, x0, 512      # base address for data (well past code)

    addi  x1, x0, 0x42
    sw    x1, 0(x24)
    lw    x2, 0(x24)        # x2 = 0x42

    sh    x1, 4(x24)
    lh    x3, 4(x24)        # x3 = 0x42
    lhu   x4, 4(x24)        # x4 = 0x42

    sb    x1, 8(x24)
    lb    x5, 8(x24)        # x5 = 0x42
    lbu   x6, 8(x24)        # x6 = 0x42

    # sign extension: store 0xFF as byte, load signed vs unsigned
    addi  x7, x0, -1
    sb    x7, 12(x24)
    lb    x8, 12(x24)       # x8 = -1 = 0xFFFFFFFF (sign extended)
    lbu   x9, 12(x24)       # x9 = 255 = 0x000000FF (zero extended)

    # sign extension: store 0xFFFF as halfword
    sh    x7, 16(x24)
    lh    x10, 16(x24)      # x10 = -1 = 0xFFFFFFFF (sign extended)
    lhu   x11, 16(x24)      # x11 = 65535 = 0x0000FFFF (zero extended)

# ── 5. Branches ───────────────────────────────────────────────────────
    addi  x1, x0, 5
    addi  x2, x0, 5
    addi  x3, x0, -1        # 0xFFFFFFFF

    beq   x1, x2, beq_ok   # 5 == 5
    jal   x0, fail
beq_ok:
    bne   x1, x3, bne_ok   # 5 != -1
    jal   x0, fail
bne_ok:
    blt   x3, x1, blt_ok   # -1 < 5  (signed)
    jal   x0, fail
blt_ok:
    bge   x1, x2, bge_ok   # 5 >= 5
    jal   x0, fail
bge_ok:
    bltu  x1, x3, bltu_ok  # 5 <u 0xFFFFFFFF
    jal   x0, fail
bltu_ok:
    bgeu  x3, x1, bgeu_ok  # 0xFFFFFFFF >=u 5
    jal   x0, fail
bgeu_ok:

# ── 6. JAL / JALR ─────────────────────────────────────────────────────
    jal   x1, jal_target    # x1 = return addr, jump to jal_target
    jal   x0, after_jal     # (jumped over by jalr returning here)
jal_target:
    jalr  x0, x1, 0         # jump back to return addr (the jal x0 above)
    jal   x0, fail          # never reached
after_jal:

# ── PASS ──────────────────────────────────────────────────────────────
    addi  x31, x0, 0xAA     # x31 = 0xAA means all tests passed
halt:
    jal   x0, halt

# ── FAIL ──────────────────────────────────────────────────────────────
fail:
    addi  x31, x0, 0xFF     # x31 = 0xFF means a branch test failed
    jal   x0, 0