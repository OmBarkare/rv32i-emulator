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

    // writes only one page at a time, for now
    pub fn read_page(&self, addr: u32) -> Result<[&[u8; 4096]], ()>{
        // do I keep a num here also?
        // I cannot create a heap structure inside this function and then return it
        // If I ask for a buffer from the user to read the data into, then I will have to trust the user to give a sufficient sized buffer to read multiple pages if I plan to keep the num parameter
        // what should I do uuhhhh
    }

    // writes only one page at a time, for now.
    pub fn write_page(&mut self, addr: u32, buf: &[u8; 4096]) -> Result<(), ()> {
        let v_addr = addr >> 12;
        self.pages[v_addr as usize] = Some( Box::new(*buf) );

        Ok(())
    }
}