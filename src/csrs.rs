#[derive(Debug)]
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
    pub misa: u32,     // to convey the ISA capabilities of machine
    pub mhartid: u32,  // 0 for single threaded
}

#[derive(Debug)]
pub struct CsrAccessResult {
    pub read_val: Option<u32>, // value read is put here
    pub write_performed: bool, // true if write was performed
    pub try_read: bool,        // true if tried to read when not allowed
    pub try_write: bool,       // true if tried to write when not allowed
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
            misa: (1 << 30) | 1 << 8,
            mhartid: 0,
        }
    }

    // TODO - figure out a way to make some csr like
    // mhartid read only
    fn get_csr(&mut self, csr: u16) -> Result<&mut u32, ()> {
        match csr {
            0x300 => Ok(&mut self.mstatus),
            0x301 => Ok(&mut self.misa),
            0x304 => Ok(&mut self.mie),
            0x305 => Ok(&mut self.mtvec),
            0x340 => Ok(&mut self.mscratch),
            0x341 => Ok(&mut self.mepc),
            0x342 => Ok(&mut self.mcause),
            0x343 => Ok(&mut self.mtval),
            0x344 => Ok(&mut self.mip),
            0xF14 => Ok(&mut self.mhartid),
            _ => Err(()),
        }
    }

    pub fn access_write(
        &mut self,
        csr: u16,
        do_read: bool,
        do_write: bool,
        val: u32,
    ) -> Result<CsrAccessResult, ()> {
        let mut res = CsrAccessResult {
            read_val: None,
            write_performed: false,
            try_read: false,
            try_write: false,
        };

        let read_only = (csr >> 10) == 3;

        // if csr is a valid csr address then proceed with operations
        if let Ok(reg) = self.get_csr(csr) {
            if do_read {
                res.read_val = Some(*reg);
            }

            if do_write {
                if read_only {
                    res.try_write = true;
                } else {
                    *reg = val;
                    res.write_performed = true;
                }
            }
        } else {
            return Err(());
        }
        Ok(res)
    }

    pub fn access_set(
        &mut self,
        csr: u16,
        do_read: bool,
        do_write: bool,
        val: u32,
    ) -> Result<CsrAccessResult, ()> {
        let mut res = CsrAccessResult {
            read_val: None,
            write_performed: false,
            try_read: false,
            try_write: false,
        };

        let read_only = (csr >> 10) == 3;

        // if csr is a valid csr address then proceed with operations
        if let Ok(reg) = self.get_csr(csr) {
            if do_read {
                res.read_val = Some(*reg);
            }

            if do_write {
                if read_only {
                    res.try_write = true;
                } else {
                    *reg |= val;
                    res.write_performed = true;
                }
            }
        } else {
            return Err(());
        }
        Ok(res)
    }

    pub fn access_clear(
        &mut self,
        csr: u16,
        do_read: bool,
        do_write: bool,
        val: u32,
    ) -> Result<CsrAccessResult, ()> {
        let mut res = CsrAccessResult {
            read_val: None,
            write_performed: false,
            try_read: false,
            try_write: false,
        };

        let read_only = (csr >> 10) == 3;

        // if csr is a valid csr address then proceed with operations
        if let Ok(reg) = self.get_csr(csr) {
            if do_read {
                res.read_val = Some(*reg);
            }

            if do_write {
                if read_only {
                    res.try_write = true;
                } else {
                    *reg &= !val;
                    res.write_performed = true;
                }
            }
        } else {
            return Err(());
        }
        Ok(res)
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
