
use crate::cpu::VECTOR_RES;

const MEMORY_SIZE: usize = 0xffff;

pub const ADDR_RESET_VECTOR: u16 = 0xe000;


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
        self.write_u16(VECTOR_RES, ADDR_RESET_VECTOR);

        // demo data
        for i in 0..16 {
            self.write_u8(ADDR_RESET_VECTOR + (i as u16), i);
        }

        // ADC #1
        self.write_u8(ADDR_RESET_VECTOR + 0, 0x69);
        self.write_u8(ADDR_RESET_VECTOR + 1, 0x01);

        // ADC #FF
        self.write_u8(ADDR_RESET_VECTOR + 2, 0x69);
        self.write_u8(ADDR_RESET_VECTOR + 3, 0xff);
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
        self.data[addr as usize] = (value & 0x00ff) as u8;                // LB
        self.data[(addr + 1) as usize] = ((value & 0xff00) >> 8) as u8;   // HB
    }

    pub fn dump(&self, addr: u16, bytes: u16) {
        print!("mem @ 0x{:04X}:", addr);
        for i in 0..bytes {
            print!(" {:02X}", self.read_u8(addr + i));
        }
        println!()
    }
}
