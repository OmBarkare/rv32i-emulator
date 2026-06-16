mod cpu;
mod csrs;
mod decoder;
mod executor;
mod fetcher;
mod instructions;
mod memory;

use cpu::Cpu;
use env_logger;
use log;
use std::{fs::File, io::Read};

fn main() {
    env_logger::Builder::new()
        .format_timestamp(None)
        .format_target(false)
        .parse_default_env()
        .format_level(false)
        .init();
    let mut cpu = Cpu::new();
    let mut file =
        File::open("/home/om/omomo/projects/rv32i-emulator/tests/print_example.elf").unwrap();
    let mut binary: Vec<u8> = Vec::new();

    file.read_to_end(&mut binary).unwrap();
    cpu.load_elf(&binary).unwrap();

    loop {
        let raw_inst = cpu.fetch();

        // Halt instruction is jal r0, 0
        if raw_inst.bits == 0x0000006F {
            log::debug!("HALT ENCOUNTERED");
            cpu.dump_registers();
            break;
        }

        log::debug!("inst_hex: 0X{:X}", &raw_inst.bits);
        log::debug!("PC: 0X{:X}", cpu.curr_pc);
        let inst = cpu.decode(raw_inst);

        log::debug!("{:#?}\n\n", inst);

        cpu.execute(inst);
    }
}
