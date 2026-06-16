.global _start

_start:
  li sp, 0x7ffffff0
  jal ra, main

_halt:
  j _halt
