const MEMORY_SIZE: usize = 0xffff;

pub struct Memory {
    pub data: [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn create() -> Memory {
        Memory {
            data: [0; MEMORY_SIZE]
        }
    }

    pub fn init(&mut self) {
        // init reset vector at $FFFC to point to $E000 for initial PC
        self.data[0xfffc] = 0x00;
        self.data[0xfffd] = 0xE0;
    }
}
