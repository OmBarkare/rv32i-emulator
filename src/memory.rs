pub struct Memory {
    pages: Vec<Option<Box<[u8; 4096]>>>,
}

impl Memory {
    // new Memory struct with unallocated pages.
    pub fn new() -> Self {
        Memory {
            pages: vec![None; 1 << 20],
        }
    }

    pub fn allocate_page(&mut self, v_addr: u32, num: u16) -> Result<(), ()> {
        // addr is the full address, including the lower 12 bits
        let p_addr = v_addr >> 12;
        for n in 0..num {
            self.pages[(p_addr + n as u32) as usize] = Some(Box::from([0u8; 4096]));
        }

        Ok(())
    }

    // write page from memory to buffer provided by buffer
    pub fn write_page(&mut self, v_addr: u32, buf: &[u8; 4096]) -> Result<(), ()> {
        let p_addr = v_addr >> 12;
        self.pages[p_addr as usize] = Some(Box::new(*buf));

        Ok(())
    }

    pub fn write_8(&mut self, v_addr: u32, byte: u8) -> Result<(), ()> {
        let p_addr = v_addr >> 12;
        let p_offset = v_addr & 0xFFF;

        match &mut self.pages[p_addr as usize] {
            Some(page) => {
                page[p_offset as usize] = byte;
                Ok(())
            }

            None => {
                self.allocate_page(v_addr, 1).unwrap();
                if let Some(page) = &mut self.pages[p_addr as usize] {
                    page[p_offset as usize] = byte;
                } else {
                    return Err(());
                }

                Ok(())
            }
        }
    }

    // write half word as little endian
    pub fn write_16(&mut self, v_addr: u32, hword: u16) -> Result<(), ()> {
        let p_addr = v_addr >> 12;
        let p_offset = v_addr & 0xFFF;

        match &mut self.pages[p_addr as usize] {
            Some(page) => {
                page[p_offset as usize] = hword as u8;
                page[p_offset as usize + 1] = (hword >> 8) as u8;
                return Ok(());
            }

            None => {
                self.allocate_page(v_addr, 1).unwrap();
                if let Some(page) = &mut self.pages[p_addr as usize] {
                    page[p_offset as usize] = hword as u8;
                    page[(p_offset + 1) as usize] = (hword >> 8) as u8;
                    return Ok(());
                } else {
                    return Err(());
                }
            }
        }
    }

    // write word as little endian
    pub fn write_32(&mut self, v_addr: u32, word: u32) -> Result<(), ()> {
        let p_addr = v_addr >> 12;
        let p_offset = v_addr & 0xFFF;

        if let Some(page) = &mut self.pages[p_addr as usize] {
            page[p_offset as usize] = word as u8;
            page[p_offset as usize + 1] = (word >> 8) as u8;
            page[p_offset as usize + 2] = (word >> 16) as u8;
            page[p_offset as usize + 3] = (word >> 24) as u8;
            Ok(())
        } else {
            Err(())
        }
    }

    // read page from memory to buffer provided by buffer
    pub fn read_page(&self, v_addr: u32, buf: &mut [u8; 4096]) -> Result<(), ()> {
        let p_addr = v_addr >> 12;

        if let Some(page) = &self.pages[p_addr as usize] {
            buf.copy_from_slice(&**page);
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn read_8(&self, v_addr: u32) -> Result<u8, ()> {
        let p_addr = v_addr >> 12;
        let p_offset = v_addr & 0xFFF;

        if let Some(page) = &self.pages[p_addr as usize] {
            Ok(page[p_offset as usize])
        } else {
            Err(())
        }
    }

    // read half word from little endian
    pub fn read_16(&self, v_addr: u32) -> Result<u16, ()> {
        let p_addr = v_addr >> 12;
        let p_offset = v_addr & 0xFFF;

        if let Some(page) = &self.pages[p_addr as usize] {
            Ok((page[p_offset as usize] as u16) | ((page[p_offset as usize + 1] as u16) << 8))
        } else {
            Err(())
        }
    }

    // read word from little endian
    pub fn read_32(&self, v_addr: u32) -> Result<u32, ()> {
        let p_addr = v_addr >> 12;
        let p_offset = v_addr & 0xFFF;

        if let Some(page) = &self.pages[p_addr as usize] {
            Ok((page[p_offset as usize] as u32)
                | ((page[p_offset as usize + 1] as u32) << 8)
                | ((page[p_offset as usize + 2] as u32) << 16)
                | ((page[p_offset as usize + 3] as u32) << 24))
        } else {
            Err(())
        }
    }
}

