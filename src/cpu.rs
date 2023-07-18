
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
        const V = 0b01000000;          // [6] Overflow Flag
        const N = 0b10000000;          // [7] Negative Flag

        const RESERVED = 0b00100000;   // [5] (reserved, always 1)

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

            sp: 0xFF,   // [0x0100 - 0x01FF] in memory

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
        let mut cur_addr: u16;

        while cycles_to_execute > 0 {
            // load instruction from mem at PC
            opcode = self.mem.read_u8(self.pc);

            // advance PC by 1 read opcode byte
            cur_addr = self.pc + 1;

            let result = Instruction::from_opcode(opcode);
            match result {
                Ok(ins) => {
                    println!("@{:04X} {:?}", self.pc, ins);
            
                    // advance PC
                    self.pc += ins.bytes as u16;

                    // handle the opcode
                    let cycles_additional = self.handle_opcode(&ins, cur_addr);
                    let cycles_consumed = ins.cycles + cycles_additional;
        
                    // decrease remaining cycle counter 
                    cycles_to_execute = cycles_to_execute.saturating_sub(cycles_consumed as u16);

                    // [debug] increase global cycles counter
                    self.cycles = self.cycles.saturating_add(cycles_consumed as u64);
                },
                Err(()) => panic!("Unimplemented or invalid instruction {:02X} @ {:04X}", opcode, self.pc),
            }
        }
    }

    fn handle_opcode(&mut self, ins: &Instruction, cur_addr: u16) -> u8 {
        let opcode = ins.opcode;
        let mut cycles_additional = 0;

        match opcode {
            ADC_IMM | ADC_ZPG | ADC_ZPX | ADC_ABS | ADC_ABX | ADC_ABY | ADC_IDX | ADC_IDY => {
                let value = self.mem.read_u8(cur_addr);
                println!("value: ${:02X}", value);

                self.sr.set(StatusFlags::V, (self.ac as u16 + value as u16) > 0xFF);        // FIXME
                self.ac = self.ac.wrapping_add(value);
                println!("AC is now: 0x{:02X}", self.ac);
                
                self.sr.set(StatusFlags::Z, self.ac == 0);
                // TODO: SR flags
            }

            JMP_ABS | JMP_IND => {
                let mut addr = self.mem.read_u16(cur_addr);
                if opcode == JMP_IND {
                    addr = self.mem.read_u16(addr); // indirection: real target is at read addr
                }
                println!("addr: ${:04X}", addr);
                self.pc = addr;
            }

            BIT_ZPG | BIT_ABS => {
                let addr: u16;
                if opcode == BIT_ABS {
                    addr = self.mem.read_u16(cur_addr);
                } else {
                    addr = self.mem.read_u8(cur_addr) as u16;
                }
                let value = self.mem.read_u8(addr);
                println!("addr: {:04X} value: {:02X} result: {:02X}", addr, value, value & self.ac);
                self.sr.set(StatusFlags::N, (value & StatusFlags::N.bits()) != 0);    // transfer bit 7 of operand to N
                self.sr.set(StatusFlags::V, (value & StatusFlags::V.bits()) != 0);    // transfer bit 6 of operand to V
                self.sr.set(StatusFlags::Z, (value & self.ac) == 0);                  // result of operand and AC
            },

            BCC_REL | BCS_REL | BEQ_REL | BNE_REL | BPL_REL | BMI_REL | BVC_REL | BVS_REL => {
                let rel = self.mem.read_i8(cur_addr);
                let jmp = match opcode {
                    BCC_REL => !self.sr.contains(StatusFlags::C),
                    BCS_REL => self.sr.contains(StatusFlags::C),
                    BEQ_REL => self.sr.contains(StatusFlags::Z),
                    BNE_REL => !self.sr.contains(StatusFlags::Z),
                    BPL_REL => !self.sr.contains(StatusFlags::N),
                    BMI_REL => self.sr.contains(StatusFlags::N),
                    BVC_REL => !self.sr.contains(StatusFlags::V),
                    BVS_REL => self.sr.contains(StatusFlags::V),
                    _ => panic!("Undefined branch opcode {:02X}", opcode),
                };
                println!("rel: ${:02X} {}  jmp: {}", rel, rel, jmp);
                if jmp {
                    self.pc = self.pc.wrapping_add(rel as u16);     // add/sub relative address
                    cycles_additional += 1;   // TODO: +2 if on different page
                }
            }

            _ => panic!("Unimplemented or invalid instruction {:02X} @ {:04X}", opcode, cur_addr - 1),
        }

        cycles_additional
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
