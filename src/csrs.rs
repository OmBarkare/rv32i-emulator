pub struct Csrs {
    pub mtvec: u32,   // trap vector base address
    pub mepc: u32,    // machine exception PC
    pub mcause: u32,  // machine exception cause
    pub mtval: u32,   // machine trap value
    pub mstatus: u32, // machine status register
    pub mstatush: u32,
    pub mie: u32,      // machine interrupt enable
    pub mip: u32,      // machine interrupt pending
    pub mscratch: u32, // temparary stratch word
}

impl Csrs {
    pub fn init() -> Self {
        Csrs {
            mtvec: 0,
            mepc: 0,
            mcause: 0,
            mtval: 0,
            mstatus: 0,
            mstatush: 0,
            mie: 0,
            mip: 0,
            mscratch: 0,
        }
    }

    pub fn write_mstatus_mpv(&mut self, set: bool) {
        self.mstatush &= !(1 << 7);
        self.mstatush |= (set as u32) << 7;
    }

    // writes only the lower 2 bits of val
    pub fn write_mstatus_mpp(&mut self, val: u8) {
        let val = ((val & 0x3) as u32) << 11;
        self.mstatus &= !(0x3 << 11);
        self.mstatus |= val;
    }

    // returns 0 or 1
    pub fn read_mstatus_mpv(&mut self) -> u8 {
        ((self.mstatush & 0x80) >> 7) as u8
    }

    // returns 0, 1 or 3
    pub fn read_mstatus_mpp(&mut self) -> u8 {
        let val = ((self.mstatus & 0xC00) >> 11) as u8;
        if val == 2 {
            panic!("MPP field in mstatus csr cannot be 2")
        }
        val
    }

    pub fn write_mstatus_mie(&mut self, set: bool) {
        self.mstatus &= !(1 << 3);
        self.mstatus |= (set as u32) << 3;
    }

    pub fn write_mstatus_mpie(&mut self, set: bool) {
        self.mstatus &= !(1 << 7);
        self.mstatus |= (set as u32) << 7;
    }

    pub fn read_mstatus_mpie(&mut self) -> bool {
        let val = (self.mstatus >> 7) & 1;
        if val == 1 {
            return true;
        } else {
            return false;
        }
    }

    pub fn read_mstatus_mie(&mut self) -> bool {
        let val = (self.mstatus >> 3) & 1;
        if val == 1 {
            return true;
        } else {
            return false;
        }
    }
}
