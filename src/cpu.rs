
use std::fmt;
use crate::mem::Memory;

pub const VECTOR_RES: usize = 0xfffc;

// #[derive(Debug)]
pub struct Cpu<'a> {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sr: u8,
    pub sp: u8,

    pub mem: &'a mut Memory,
}

impl fmt::Debug for Cpu<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpu")
            .field("PC", &format!("0x{:04X}", self.pc))
            .field("AC", &format!("0x{:02X}", self.ac))
            // TODO 
            .finish()
    }
}

impl Cpu<'_> {
    pub fn create(memory: &mut Memory) -> Cpu {
        Cpu {
            // registers
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0,

            // memory
            mem: memory,
        }
    }

    pub fn reset(&mut self) {
        // load address from reset vector $FFFC and store it into PC
        self.pc = self.mem.read_u16(VECTOR_RES);
    }
}


