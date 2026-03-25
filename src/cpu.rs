pub struct Cpu {
    pub regs: [u32; 32],
    pub pc: u32,
    pub mem: Vec<u8>,
}
