
use std::fmt;
use bitflags::bitflags;
use crate::instruction::*;
use crate::mem::Memory;

pub const VECTOR_RES: u16 = 0xfffc;

bitflags! {
    pub struct StatusFlags: u8 {
        const C = 0b00000001;          // [0] Carry Flag
        const Z = 0b00000010;          // [1] Zero Flag
        const I = 0b00000100;          // [2] Interrupt Disable
        const D = 0b00001000;          // [3] Decimal Mode
        const B = 0b00010000;          // [4] Break Command
        const RESERVED = 0b00100000;   // [5] (reserved, always 1)
        const V = 0b01000000;          // [6] Overflow Flag
        const N = 0b10000000;          // [7] Negative Flag

        const ALL = Self::C.bits() | Self::Z.bits() | Self::I.bits() | Self::D.bits() | Self::B.bits() | Self::V.bits() | Self::N.bits();
    }
}

pub struct Cpu<'a> {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sr: StatusFlags,
    pub sp: u8,

    pub mem: &'a mut Memory,

    // for debugging
    pub cycles: u64,
}

impl Cpu<'_> {
    pub fn create(mem: &mut Memory) -> Cpu {
        Cpu {
            // registers
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,

            sr: StatusFlags::RESERVED,

            sp: 0xff,   // [0x0100 - 0x01FF] in memory

            // memory
            mem,

            // debug
            cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        // load address from reset vector $FFFC and store it into PC
        self.pc = self.mem.read_u16(VECTOR_RES);
    }

    pub fn exec(&mut self, max_cycles: u16) {
        let mut cycles_to_execute = max_cycles;
        let mut opcode: u8;
        let mut addr: u16;

        while cycles_to_execute > 0 {
            addr = self.pc;

            // load instruction from mem at PC
            opcode = self.mem.read_u8(addr);

            let ins = Instruction::from_opcode(opcode);
            println!("@{:04X} {:#?}", self.pc, ins);

            // advance current address
            addr += 1;

            match opcode {
                ADC_IMM | ADC_ZPG | ADC_ZPX | ADC_ABS | ADC_ABX | ADC_ABY | ADC_IDX | ADC_IDY => {
                    println!("[[ADC]] ${:02X}", opcode);
                    let value: u8 = self.mem.read_u8(addr);
                    println!("value: ${:02X}", value);

                    self.sr.set(StatusFlags::V, (self.ac as u16 + value as u16) > 0xff);        // FIXME
                    self.ac = self.ac.wrapping_add(value);
                    println!("AC is now: 0x{:02X}", self.ac);
                    
                    self.sr.set(StatusFlags::Z, self.ac == 0);
                    // TODO: SR flags
                }

                _ => panic!("Unimplemented or invalid instruction {:02X} @ {:04X}", opcode, self.pc),
            }
            
            self.pc += ins.bytes as u16;
            cycles_to_execute = cycles_to_execute.saturating_sub(ins.cycles as u16);

            // increase global cycles counter
            self.cycles = self.cycles.saturating_add(ins.cycles as u64);
        }
    }
}

impl fmt::Debug for Cpu<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let f_n = if self.sr.contains(StatusFlags::N) {"N"} else {"-"};
        let f_z = if self.sr.contains(StatusFlags::Z) {"Z"} else {"-"};
        let f_c = if self.sr.contains(StatusFlags::C) {"C"} else {"-"};
        let f_i = if self.sr.contains(StatusFlags::I) {"I"} else {"-"};
        let f_d = if self.sr.contains(StatusFlags::D) {"D"} else {"-"};
        let f_v = if self.sr.contains(StatusFlags::V) {"V"} else {"-"};

        f.debug_struct("Cpu")
            .field("PC", &format!("0x{:04X}", self.pc))
            .field("AC", &format!("0x{:02X}", self.ac))
            .field("X", &format!("0x{:02X}", self.x))
            .field("Y", &format!("0x{:02X}", self.y))
            .field("SR", &format!("0x{:02X}  [{}{}{}{}{}{}]", self.sr, f_n, f_z, f_c, f_i, f_d, f_v))
            .field("SP", &format!("0x{:02X}", self.sp))
            .field("[cycles]", &self.cycles)
            .finish()
    }
}
