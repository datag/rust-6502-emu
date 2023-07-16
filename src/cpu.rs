
use std::fmt;
use crate::mem::Memory;

pub const VECTOR_RES: u16 = 0xfffc;

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

impl Cpu<'_> {
    pub fn create(mem: &mut Memory) -> Cpu {
        Cpu {
            // registers
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0,

            // memory
            mem,
        }
    }

    pub fn reset(&mut self) {
        // load address from reset vector $FFFC and store it into PC
        self.pc = self.mem.read_u16(VECTOR_RES);
    }

    pub fn exec(&mut self, max_cycles: u16) {
        let mut cycles = max_cycles;
        let mut ins: u8;

        while cycles > 0 {
            // load instruction from mem at PC
            ins = self.mem.read_u8(self.pc);

            // increment PC
            self.pc += 1;

            match ins {
                0x00 => {
                    println!("got 0x00");
                    self.pc += 1;
                    cycles -= 2;
                }
                0x01 => {
                    println!("got 0x01");
                    self.pc += 2;
                    cycles -= 1;
                },
                _ => panic!("Unimplemented or invalid instruction {:02X} @ {:04X}", ins, self.pc - 1),
            }
            // TODO: dec max_cycles
        }
    }
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
