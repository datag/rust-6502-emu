
use crate::cpu::VECTOR_RES;

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

    pub fn init(&mut self) {
        // init reset vector at $FFFC to point to $E000 for initial PC
        self.write_u16(VECTOR_RES, 0xE000);
    }

    pub fn read_u8(&self, addr: usize) -> u8 {
        self.data[addr]
    }

    pub fn read_u16(&self, addr: usize) -> u16 {
        ((self.data[addr + 1] as u16) << 8) | (self.data[addr + 0] as u16)
    }

    pub fn write_u8(&mut self, addr: usize, value: u8) {
        self.data[addr] = value;
    }

    pub fn write_u16(&mut self, addr: usize, value: u16) {
        self.data[addr + 0] = (value & 0x00ff) as u8;          // LB
        self.data[addr + 1] = ((value & 0xff00) >> 8) as u8;   // HB
    }
}
