mod cpu;
mod decoder;
mod executor;
mod instructions;
mod fetcher;

use std::{fs::File, io::Read};
use cpu::Cpu;

fn main() {
    
    let mut cpu = Cpu::new();
    let mut file = File::open("binary").unwrap();
    let mut binary: Vec<u8> = Vec::new();

    file.read_to_end(&mut binary).unwrap();
    cpu.load_binary(&binary);

    loop {
        let raw_inst = cpu.fetch();
        let inst = cpu.decode(raw_inst);
        cpu.execute(inst);
    }
}
