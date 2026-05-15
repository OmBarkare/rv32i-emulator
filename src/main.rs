mod cpu;
mod decoder;
mod executor;
mod instructions;
mod fetcher;
mod memory;

use std::{fs::File, io::Read};
use cpu::Cpu;

fn main() {
    
    let mut cpu = Cpu::new();
    let mut file = File::open("/home/om/omomo/projects/rv32i-emulator/tests/binary").unwrap();
    let mut binary: Vec<u8> = Vec::new();

    file.read_to_end(&mut binary).unwrap();
    cpu.load_binary(&binary);

    loop {
        let raw_inst = cpu.fetch();

        if raw_inst.bits == 0x0000006F {
            println!("HALT");
            cpu.dump_registers();
            break;
        }

        let inst = cpu.decode(raw_inst);

        println!("{:#?}", inst);

        cpu.execute(inst);

        println!("PC: {:X}", cpu.pc);
    }
}