mod cpu;
mod decoder;
mod executor;
mod fetcher;
mod instructions;
mod memory;

use cpu::Cpu;
use std::{fs::File, io::Read};

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

        println!("inst_hex: 0X{:X}", &raw_inst.bits);
        let inst = cpu.decode(raw_inst);

        println!("{:#?}", inst);

        cpu.execute(inst);

        println!("PC: 0X{:X}", cpu.pc);
    }
}
