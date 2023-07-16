
const MEMORY_SIZE: usize = 0xffff;


pub struct Memory {
    data: [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn create() -> Memory {
        Memory {
            data: [0; MEMORY_SIZE]
        }
    }

    pub fn read_u8(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn read_u16(&self, addr: usize) -> u16 {
        ((self.data[addr] as u16) << 8) | (self.data[addr + 1] as u16)
    }

    pub fn write_u8(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }

    pub fn write_u16(&mut self, addr: usize, value: u16) {
        self.data[addr + 0] = ((value & 0xff00) >> 8) as u8;  // HB
        self.data[addr + 1] = (value & 0x00ff) as u8;         // LB
    }
}
