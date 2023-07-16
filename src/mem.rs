
use crate::cpu::VECTOR_RES;

const MEMORY_SIZE: usize = 0xffff;

const RESET_VECTOR_ADDR: u16 = 0xe000;


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
        self.write_u16(VECTOR_RES, RESET_VECTOR_ADDR);

        // demo data
        for i in 0..16 {
            self.write_u8(RESET_VECTOR_ADDR + (i as u16), i);
        }
    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        ((self.data[(addr + 1) as usize] as u16) << 8) | (self.data[addr as usize] as u16)
    }

    pub fn write_u8(&mut self, addr: u16, value: u8) {
        self.data[addr as usize] = value;
    }

    pub fn write_u16(&mut self, addr: u16, value: u16) {
        self.data[addr as usize] = (value & 0x00ff) as u8;          // LB
        self.data[(addr + 1) as usize] = ((value & 0xff00) >> 8) as u8;   // HB
    }
}
