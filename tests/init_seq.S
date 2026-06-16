.global _start

_start:
  li sp, 0x7ffffff0
  la t0, trap_handler
  csrw mtvec, t0
  jal ra, main

_halt:
  j _halt

trap_handler:
  csrr t0, mcause
  li t1, 11;
  beq t0, 11, sys_handler
  j return_from_trap

sys_handler:
  li t0, 64
  beq a7, t0, sys_write
  j return_from_trap

sys_write:
  li t1, 0x80000000 # anything written to this address will be printed
  mv t2, a2 # a2 -> len
  mv t4, a1 # buf pointer
write_loop:
  beqz t2, return_from_trap
  lb t3, 0(t4)
  sb t3, 0(t1)
  addi t4, t4, 1
  addi t2, t2, -1
  j write_loop

return_from_trap:
  csrr t0, mepc
  addi t0, t0, 4
  csrw mepc, t0
  mret
