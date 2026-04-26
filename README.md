# RV32I emulator
implementation of the rv32i instruction set in rust

## Why I am making it
because I want to learn how computers work at the lowest level of abstraction

## How to run tests
clone using
```bash
git clone https://github.com/OmBarkare/dsandbox.git
```

go to the project directory and run
```bash
cargo test
```

## Immediate goals
- make an elf loader to load a elf compiled for rv32i into memory, so that I can execute at least bare metal binary

## Final goal
- Implement the privilaged ISA-
- make implementation capable enough to run an operating system

## What is already working
- currently implements the full rv32i base integer instruction set