# RV32I Emulator
A RISCV emulator written in Rust, implementing RV32I and Zicsr extensions with full M-mode privilege support including trap handling (for exceptions)

## Extensions currently implemented
Extensions:-
- I
- Zicsr

## Other capabilities
- Currently, it is a full M mode implementation. It does not have other privilege modes.
- It can load statically linked ELF files
- Can trap for exceptions, no interrupts implemented yet
- CSR enforce register level read only semantics. Per-bit WARL masking is not yet implemented
- Ecall, Ebreak and Illegal instructions trap
- Can run C programs, and can print strings

NOTE
> trap handler has to be setup by the user program themselves, and then change mtvec to point to that address.

## To compile C programs
`init_seq.S` and `syscall.c` files are present in `tests/` directory
```
riscv64-unknown-elf-gcc -march=rv32i_zicsr -mabi=ilp32 -nostdlib init_seq.S syscall.s <filename>
```
you can use the `-o` flag to specify output filename

## To run the statically linked elf file
- go to `src/main.rs` and update the file path to the path where your elf file is located
go to the project directory and run
```bash
cargo run
```
to also print all debug messages, which include register dump upon halt and each instruction executed,<br>
```
RUST_LOG=DEBUG cargo run
```

## Other interesting technical details
### Sparsely allocated memory
The whole 32 bit address space is usable, but is not immediately allocated. The memory has been split into 2^20 pages of 4KB size, and a page is only allocated when the process attempt to write to an address that falls inside that page.
If you read from an unallocated memory (memory which was never written to), then the interface will return a `None`<br>
This is how the memory looks:<br>
```
pub struct Memory {
    pages: Vec<Option<Box<[u8; 4096]>>>,
}
```
And this is how it is initialised:<br>
```
    pub fn new() -> Self {
        Memory {
            pages: vec![None; 1 << 20],
        }
    }
```

### CSRs access interface
More than the interface itself, what is interesting is the return value that you get on accessing a csr. The return value looks like this:<br>
```
pub struct CsrAccessResult {
    pub read_val: Option<u32>, // value read is put here
    pub write_performed: bool, // true if write was performed
    pub try_read: bool,        // true if tried to read when not allowed
    pub try_write: bool,       // true if tried to write when not allowed
}
```
Such a return value allows to easily implement side-effects or read and write, and also makes policy violation visible and easily accessible.

## Why am I doing this?
I want to understand how these wonderful machines work.<br>Why didnt you just read a book to learn it then?<br>It is more fun to learn this way, and you can tweak the machine and make it behave as you want. It is just fun to test what the books say, and make it my own.
