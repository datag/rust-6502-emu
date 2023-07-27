
use std::fs::File;
use std::io::{BufReader, Read, Error};

use crate::cpu;
use crate::instruction;

const MEMORY_SIZE: usize = 0x10000;

pub const ADDR_RESET_VECTOR: u16 = 0xE000;


pub struct Memory {
    data: [u8; MEMORY_SIZE],
    current_write_addr: Option<u16>,
}

impl Memory {
    pub fn create() -> Self {
        Self {
            data: [0; MEMORY_SIZE],
            current_write_addr: None,       // comfort feature for consecutive writes
        }
    }

    pub fn reset(&mut self) {
        // initialize with zero
        self.data = [0; MEMORY_SIZE];

        self.write_u16(cpu::VECTOR_RES, ADDR_RESET_VECTOR);

        self.current_write_addr = None;
    }

    pub fn load_from_file(&mut self, addr: u16, filename: &str) -> Result<(), Error>{
        let file = File::open(filename)?;
        let mut reader = BufReader::new(file);
        
        let mut buffer = [0u8; 1024];
        let mut pos = 0;

        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(bytes_read) => {
                    for item in buffer.iter().take(bytes_read) {
                        self.write_u8(addr + pos, *item);
                        pos += 1;
                    }
                }
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }

    pub fn demo(&mut self) {
        // demo data
        for i in 0..16 {
            self.write_u8(ADDR_RESET_VECTOR + (i as u16), i);
        }

        self.write_u8(ADDR_RESET_VECTOR, instruction::NOP);

        self.write_u8(None, instruction::ADC_IMM);
        self.write_u8(None, 0x01);

        self.write_u8(None, instruction::ADC_ZPG);
        self.write_u8(None, 0x01);

        self.write_u8(None, instruction::ADC_ZPX);
        self.write_u8(None, 0x01);

        self.write_u8(None, instruction::ADC_ABS);
        self.write_u16(None, 0xF001);


    }

    pub fn read_u8(&self, addr: u16) -> u8 {
        self.data[addr as usize]
    }

    pub fn read_i8(&self, addr: u16) -> i8 {
        self.data[addr as usize] as i8
    }

    pub fn read_u16(&self, addr: u16) -> u16 {
        (self.data[addr as usize] as u16) /* LB */ | ((self.data[(addr + 1) as usize] as u16) << 8) /* HB */
    }

    pub fn write_u8<T: Into<Option<u16>>>(&mut self, addr: T, value: u8) {
        let write_addr: u16;
        match addr.into() {
            Some(addr) => write_addr = addr,
            None => {
                match self.current_write_addr {
                    Some(addr) => write_addr = addr,
                    None => panic!("No address provided and no previous write has occurred."),
                }
            }
        }
        self.data[write_addr as usize] = value;
        self.current_write_addr = Some(write_addr.wrapping_add(1));
    }

    pub fn write_i8<T: Into<Option<u16>>>(&mut self, addr: T, value: i8) {
        self.write_u8(addr, value as u8);
    }

    pub fn write_u16<T: Into<Option<u16>>>(&mut self, addr: T, value: u16) {
        let write_addr: u16;
        match addr.into() {
            Some(addr) => write_addr = addr,
            None => {
                match self.current_write_addr {
                    Some(addr) => write_addr = addr,
                    None => panic!("No address provided and no previous write has occurred."),
                }
            }
        }
        self.data[write_addr as usize] = (value & 0x00FF) as u8;                // LB
        self.data[write_addr.wrapping_add(1) as usize] = ((value & 0xFF00) >> 8) as u8;   // HB
        self.current_write_addr = Some(write_addr.wrapping_add(2));
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
        assert!(!has_nonzero_value(&mem.data[0..cpu::VECTOR_RES as usize]));
        assert!(!has_nonzero_value(&mem.data[(cpu::VECTOR_RES as usize)+2..]));
    }

    #[test]
    fn addr_reset_vector_correct() {
        let mem = setup();
        assert_eq!(mem.read_u16(cpu::VECTOR_RES), ADDR_RESET_VECTOR);
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
        mem.data[addr as usize] = lb;
        mem.data[(addr + 1) as usize] = hb;
        assert_eq!(mem.read_u16(addr), value);
    }

    #[test]
    fn write_u8() {
        let mut mem = setup();
        let addr: u16 = 0x0F00;
        let value1: u8 = 0xFE;
        let value2: u8 = 0xAA;
        mem.write_u8(addr, value1);
        assert_eq!(mem.data[addr as usize], value1);

        // consecutive test
        mem.write_u8(None, value2);
        assert_eq!(mem.read_u8(addr + 1), value2);
    }

    #[test]
    fn write_i8() {
        let mut mem = setup();
        let addr: u16 = 0x0F00;
        let value1: i8 = -120;
        let value2: i8 = 33;
        mem.write_i8(addr, value1);
        assert_eq!(mem.data[addr as usize], value1 as u8);
        assert_eq!(mem.read_i8(addr), value1);      // also via read method

        // consecutive test
        mem.write_i8(None, value2);
        assert_eq!(mem.read_i8(addr + 1), value2);
    }

    #[test]
    fn write_u16() {
        let mut mem = setup();
        let addr: u16 = 0x0F00;
        let value1: u16 = 0xBEEF;
        let lb: u8 = 0xEF;  // (value & 0x00FF) as u8;
        let hb: u8 = 0xBE;  // ((value & 0xFF00) >> 8) as u8;
        let value2: u16 = 0xCAFE;
        mem.write_u16(addr, value1);
        assert_eq!(mem.data[addr as usize], lb);
        assert_eq!(mem.data[(addr + 1) as usize], hb);

        // consecutive test
        mem.write_u16(None, value2);
        assert_eq!(mem.read_u16(addr + 2), value2);
    }
}
