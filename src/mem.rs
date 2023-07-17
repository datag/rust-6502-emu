
use crate::cpu::VECTOR_RES;

const MEMORY_SIZE: usize = 0xFFFF;

pub const ADDR_RESET_VECTOR: u16 = 0xE000;


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

        // // ADC #1
        // self.write_u8(ADDR_RESET_VECTOR + 0, 0x69);
        // self.write_u8(ADDR_RESET_VECTOR + 1, 0x01);

        // // ADC #FF
        // self.write_u8(ADDR_RESET_VECTOR + 2, 0x69);
        // self.write_u8(ADDR_RESET_VECTOR + 3, 0xFF);

        // // JMP (ABS)
        // self.write_u8(ADDR_RESET_VECTOR + 4, 0x4C);
        // self.write_u16(ADDR_RESET_VECTOR + 5, ADDR_RESET_VECTOR);

        // // JMP (IND)
        // self.write_u16(0x00F0, ADDR_RESET_VECTOR);
        // self.write_u8(ADDR_RESET_VECTOR + 4, 0x6C);
        // self.write_u16(ADDR_RESET_VECTOR + 5, 0x00F0);

        // BIT (ABS)
        self.write_u16(0x00F0, 0x81);
        self.write_u8(ADDR_RESET_VECTOR + 0, 0x2C);
        self.write_u16(ADDR_RESET_VECTOR + 1, 0x00F0);

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
        self.data[addr as usize] = (value & 0x00FF) as u8;                // LB
        self.data[(addr + 1) as usize] = ((value & 0xFF00) >> 8) as u8;   // HB
    }

    pub fn dump(&self, addr: u16, bytes: u16) {
        print!("mem @ 0x{:04X}:", addr);
        for i in 0..bytes {
            print!(" {:02X}", self.read_u8(addr + i));
        }
        println!()
    }
}
