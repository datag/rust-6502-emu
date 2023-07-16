
use std::fmt;
use crate::mem::Memory;

// #[derive(Debug)]
pub struct Cpu<'a> {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sr: u8,
    pub sp: u8,

    pub mem: &'a Memory,
}

impl fmt::Debug for Cpu<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Cpu")
            .field("PC", &format!("0x{:X}", self.pc))
            .field("AC", &format!("0x{:X}", self.ac))
            .finish()
    }
}

impl Cpu<'_> {
    pub fn create(memory: &Memory) -> Cpu {
        Cpu {
            pc: 0xfffc,
            ac: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0,

            mem: memory,
        }
    }
}


