mod cpu;
mod csrs;
mod decoder;
mod executor;
mod fetcher;
mod instructions;
mod memory;

use cpu::Cpu;
use std::{fs::File, io::Read};

fn main() {
    let mut cpu = Cpu::new();
    let mut file =
        File::open("/home/om/omomo/projects/rv32i-emulator/dump/basic_exec_cssrs.elf").unwrap();
    let mut binary: Vec<u8> = Vec::new();

    file.read_to_end(&mut binary).unwrap();
    cpu.load_elf(&binary).unwrap();

    loop {
        let raw_inst = cpu.fetch();

        // Halt instruction is jal r0, 0
        if raw_inst.bits == 0x0000006F {
            println!("HALT ENCOUNTERED");
            cpu.dump_registers();
            println!("PRINTING MEMORY:\n");
            println!(
                "mem[0x44]: 0X{:X}\nmem[0x40]: 0X{:X}",
                cpu.mem.read_32(0x44).unwrap(),
                cpu.mem.read_32(0x40).unwrap()
            );
            break;
        }

        println!("inst_hex: 0X{:X}", &raw_inst.bits);
        println!("PC: 0X{:X}", cpu.curr_pc);
        let inst = cpu.decode(raw_inst);

        println!("{:#?}\n\n", inst);

        cpu.execute(inst);
    }
}
