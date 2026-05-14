pub struct Memory {
    pages: Vec<Option<Box<[u8; 4096]>>>
}

impl Memory {
    // new Memory struct with unallocated pages. 
    pub fn new() -> Self {
        Memory { pages: vec![None; 1 << 20] }
    }

    pub fn allocate_page(&mut self, addr: u32, num: u16) -> Result<(), ()> {
        // addr is the full address, including the lower 12 bits
        let v_addr = addr >> 12;
        for n in 0..num {
            self.pages[(v_addr + n as u32) as usize] = Some( Box::from([0u8; 4096]) );
        }

        Ok(())
    }

    // read page from memory to buffer provided by buffer
    pub fn read_page(&self, addr: u32, buf: &mut [u8; 4096]) -> Result<(), ()>{
        let v_addr = addr >> 12;

        if let Some(page) = &self.pages[v_addr as usize] {
            buf.copy_from_slice(&**page);
            Ok(())
        } else  {
            Err(())
        }
    }

    // write page from memory to buffer provided by buffer
    pub fn write_page(&mut self, addr: u32, buf: &[u8; 4096]) -> Result<(), ()> {
        let v_addr = addr >> 12;
        self.pages[v_addr as usize] = Some( Box::new(*buf) );

        Ok(())
    }
}