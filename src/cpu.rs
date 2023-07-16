
use std::fmt;
use crate::instruction::*;
use crate::mem::Memory;

pub const VECTOR_RES: u16 = 0xfffc;

pub const SRF_C: u8 = 0x1;      // [0] Carry Flag
pub const SRF_Z: u8 = 0x2;      // [1] Zero Flag
pub const SRF_I: u8 = 0x4;      // [2] Interrupt Disable
pub const SRF_D: u8 = 0x8;      // [3] Decimal Mode
pub const SRF_B: u8 = 0x10;     // [4] Break Command
// Bit 5 (0x20) is ignored      // [5] (ignored)
pub const SRF_V: u8 = 0x40;     // [6] Overflow Flag
pub const SRF_N: u8 = 0x80;     // [7] Negative Flag

pub struct Cpu<'a> {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sr: u8,
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
            sr: 0,
            sp: 0,

            // memory
            mem,

            // debug
            cycles: 0,
        }
    }

    pub fn reset(&mut self) {
        // load address from reset vector $FFFC and store it into PC
        self.pc = self.mem.read_u16(VECTOR_RES);

        // initialize SP; [0x0100 - 0x01FF] in memory
        self.sp = 0xFF;
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

                    self.sr = (self.sr & !SRF_V) | if (self.ac as u16 + value as u16) > 0xff {SRF_V} else {0};      // FIXME!
                    self.ac = self.ac.wrapping_add(value);
                    println!("AC is now: 0x{:02X}", self.ac);
                    self.sr = (self.sr & !SRF_Z) | if self.ac == 0 {SRF_Z} else {0};
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
        let f_n = if self.sr & SRF_N > 0 {"N"} else {"-"};
        let f_z = if self.sr & SRF_Z > 0 {"Z"} else {"-"};
        let f_c = if self.sr & SRF_C > 0 {"C"} else {"-"};
        let f_i = if self.sr & SRF_I > 0 {"I"} else {"-"};
        let f_d = if self.sr & SRF_D > 0 {"D"} else {"-"};
        let f_v = if self.sr & SRF_V > 0 {"V"} else {"-"};

        f.debug_struct("Cpu")
            .field("PC", &format!("0x{:04X}", self.pc))
            .field("AC", &format!("0x{:02X}", self.ac))
            .field("X", &format!("0x{:02X}", self.x))
            .field("Y", &format!("0x{:02X}", self.y))
            .field("SR", &format!("0x{:02X}", self.sr))
            .field("SR flags", &format!("{}{}{}{}{}{}", f_n, f_z, f_c, f_i, f_d, f_v))
            .field("SP", &format!("0x{:02X}", self.sp))
            .field("[cycles]", &self.cycles)
            .finish()
    }
}
