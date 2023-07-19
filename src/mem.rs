
use crate::cpu::VECTOR_RES;

const MEMORY_SIZE: usize = 0xFFFF;

pub const ADDR_RESET_VECTOR: u16 = 0xE000;


pub struct Memory {
    data: [u8; MEMORY_SIZE]
}

impl Memory {
    pub fn create() -> Self {
        Self {
            data: [0; MEMORY_SIZE]
        }
    }

    pub fn reset(&mut self) {
        // initialize with zero
        self.data = [0; MEMORY_SIZE];

        self.write_u16(VECTOR_RES, ADDR_RESET_VECTOR);
    }

    pub fn demo(&mut self) {
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

        // // BIT (ABS)
        // self.write_u16(0x00F0, 0x81);
        // self.write_u8(ADDR_RESET_VECTOR + 0, 0x2C);
        // self.write_u16(ADDR_RESET_VECTOR + 1, 0x00F0);

        // B**
        self.write_u8(ADDR_RESET_VECTOR + 0, crate::instruction::BVS_REL);
        self.write_u8(ADDR_RESET_VECTOR + 1, (-2 as i8) as u8);

        // some instruction
        self.write_u8(ADDR_RESET_VECTOR + 2, crate::instruction::ADC_IMM);
        self.write_u8(ADDR_RESET_VECTOR + 3, 0x01);

        // some other instruction
        self.write_u8(ADDR_RESET_VECTOR + 4, crate::instruction::ADC_IMM);
        self.write_u8(ADDR_RESET_VECTOR + 5, 0x02);

    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn read_i8(&self, addr: u16) -> i8 {
        self.data[addr as usize] as i8
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

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> Memory {
        let mut mem = Memory::create();
        mem.reset();
        mem
    }

    fn has_nonzero_value(array: &[u8]) -> bool {
        array.iter().any(|&value| value != 0)
    }

    #[test]
    fn initialized_with_zero() {
        let mem = setup();

        // all zero excluding VECTOR_RES (2 bytes)
        assert_eq!(has_nonzero_value(&mem.data[1..VECTOR_RES as usize]), false);
        assert_eq!(has_nonzero_value(&mem.data[(VECTOR_RES as usize)+2..]), false);
    }

    #[test]
    fn addr_reset_vector_correct() {
        let mem = setup();
        assert_eq!(mem.read_u16(VECTOR_RES), ADDR_RESET_VECTOR);
    }

    #[test]
    fn read_u8() {
        let mut mem = setup();
        let addr: u16 = 0x0F00;
        let value: u8 = 0xFE;
        mem.data[addr as usize] = value;
        assert_eq!(mem.read_u8(addr), value);
    }

    #[test]
    fn read_i8() {
        let mut mem = setup();
        let addr: u16 = 0x0F00;
        let value: i8 = -120;
        mem.data[addr as usize] = value as u8;
        assert_eq!(mem.read_i8(addr), value);
    }

    #[test]
    fn read_u16() {
        let mut mem = setup();
        let addr: u16 = 0x0F00;
        let value: u16 = 0xBEEF;
        let lb: u8 = 0xEF;  // (value & 0x00FF) as u8;
        let hb: u8 = 0xBE;  // ((value & 0xFF00) >> 8) as u8;
        mem.data[(addr + 0) as usize] = lb;
        mem.data[(addr + 1) as usize] = hb;
        assert_eq!(mem.read_u16(addr), value);
    }

    #[test]
    fn write_u8() {
        let mut mem = setup();
        let addr: u16 = 0x0F00;
        let value: u8 = 0xFE;
        mem.write_u8(addr, value);
        assert_eq!(mem.data[addr as usize], value);
    }

    #[test]
    fn write_u16() {
        let mut mem = setup();
        let addr: u16 = 0x0F00;
        let value: u16 = 0xBEEF;
        let lb: u8 = 0xEF;  // (value & 0x00FF) as u8;
        let hb: u8 = 0xBE;  // ((value & 0xFF00) >> 8) as u8;
        mem.write_u16(addr, value);
        assert_eq!(mem.data[(addr + 0) as usize], lb);
        assert_eq!(mem.data[(addr + 1) as usize], hb);
    }
}
