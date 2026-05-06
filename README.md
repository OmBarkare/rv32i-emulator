# RV32I emulator
implementation of the rv32i instruction set in rust

## Why I am making it
because I want to learn how computers work at the lowest level of abstraction

## What it does
This RV32I emulator can execute flat binaries containing RV32I unprivileged instructions

## How to run test binary
clone using
```bash
git clone https://github.com/OmBarkare/rv32i-emulator.git
```

go to the project directory and run
```bash
cargo run
```

## Immediate goals
- make an elf loader to load a elf compiled for rv32i into memory, so that I can execute at least bare metal binary

## Final goal
- Implement the privilaged ISA
- make implementation capable enough to run an operating system
- implement ISA extensions

## What is already working
- Can run flat binaries for unprivileged instructions