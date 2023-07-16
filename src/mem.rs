
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
}
