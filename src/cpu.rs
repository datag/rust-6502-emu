
use std::fmt;
use bitflags::bitflags;
use crate::instruction::*;
use crate::mem::Memory;

pub const VECTOR_RES: u16 = 0xFFFC;
pub const INITIAL_STACK_POINTER: u8 = 0xFF;        // [0x0100 - 0x01FF] in memory

bitflags! {
    #[derive(PartialEq, Debug)]
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

impl Default for StatusFlags {
    fn default() -> StatusFlags {
        StatusFlags::RESERVED          // the reserved bit reads always as 1
    }
}

pub struct Cpu {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sr: StatusFlags,
    pub sp: u8,

    // for debugging
    pub cycles: u64,
}

impl Cpu {
    pub fn create() -> Cpu {
        Cpu {
            // registers
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            sr: StatusFlags::empty(),
            sp: 0,

            // debug
            cycles: 0,
        }
    }

    fn is_page_crossed(cur_addr: u16, rel: i8) -> bool {
        // divide current address by 256 (0x100) to get the current page
        let current_page = cur_addr >> 8;

        // calculate the target page
        let target_page = (cur_addr.wrapping_add(rel as u16)) >> 8;

        current_page != target_page
    }

    pub fn reset(&mut self, mem: &mut Memory) {
        mem.reset();

        // AC, X and Y
        self.ac = 0;
        self.x = 0;
        self.y = 0;

        // only the reserved bit 5 is set; the flag B is 0 and the others may be uninitialized (?)
        self.sr = StatusFlags::default();

        // load address from reset vector $FFFC and store it into PC
        self.pc = mem.read_u16(VECTOR_RES);

        // stack pointer
        self.sp = INITIAL_STACK_POINTER;

        // [debug]
        self.cycles = 0;
    }

    pub fn exec(&mut self, mem: &mut Memory, max_cycles: u16) {
        let mut cycles_to_execute = max_cycles;
        let mut opcode: u8;
        let mut cur_addr: u16;

        while cycles_to_execute > 0 {
            // load instruction from mem at PC
            opcode = mem.read_u8(self.pc);

            // advance PC by 1 read opcode byte
            cur_addr = self.pc + 1;

            let result = Instruction::from_opcode(opcode);
            match result {
                Ok(ins) => {
                    println!("@{:04X} {:?}", self.pc, ins);
            
                    // advance PC
                    self.pc += ins.bytes as u16;

                    // handle the opcode
                    let cycles_additional = self.handle_opcode(mem, &ins, cur_addr);
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

    fn handle_opcode(&mut self, mem: &mut Memory, ins: &Instruction, cur_addr: u16) -> u8 {
        let opcode = ins.opcode;
        let mut cycles_additional = 0;

        match opcode {
            NOP => println!("NOP"),

            ADC_IMM | ADC_ZPG | ADC_ZPX | ADC_ABS | ADC_ABX | ADC_ABY | ADC_IDX | ADC_IDY => {
                // FIXME: incomplete
                // TODO: possible page crossing additional cycle for ZPX, ABX and ABY?
                let value = mem.read_u8(cur_addr);
                println!("value: ${:02X}", value);

                self.sr.set(StatusFlags::V, (self.ac as u16 + value as u16) > 0xFF);        // FIXME
                self.ac = self.ac.wrapping_add(value);
                println!("AC is now: 0x{:02X}", self.ac);
                
                self.sr.set(StatusFlags::Z, self.ac == 0);
                // TODO: SR flags
            }

            JMP_ABS | JMP_IND => {
                let mut addr = mem.read_u16(cur_addr);
                if opcode == JMP_IND {
                    addr = mem.read_u16(addr); // indirection: real target is at read addr
                }
                println!("addr: ${:04X}", addr);
                self.pc = addr;
            }

            BIT_ZPG | BIT_ABS => {
                let addr: u16;
                match opcode {
                    BIT_ABS => addr = mem.read_u16(cur_addr),
                    BIT_ZPG => addr = mem.read_u8(cur_addr) as u16,
                    _ => panic!("Unhandled BIT opcode {:02X}", opcode),
                }
                let value = mem.read_u8(addr);
                println!("addr: {:04X} value: {:02X} result: {:02X}", addr, value, value & self.ac);
                self.sr.set(StatusFlags::N, value & StatusFlags::N.bits() != 0);    // transfer bit 7 of operand to N
                self.sr.set(StatusFlags::V, value & StatusFlags::V.bits() != 0);    // transfer bit 6 of operand to V
                self.sr.set(StatusFlags::Z, value & self.ac == 0);                  // result of operand and AC
            },

            CLC => self.sr.remove(StatusFlags::C),
            CLD => self.sr.remove(StatusFlags::D),
            CLI => self.sr.remove(StatusFlags::I),
            CLV => self.sr.remove(StatusFlags::V),
            SEC => self.sr.insert(StatusFlags::C),
            SED => self.sr.insert(StatusFlags::D),
            SEI => self.sr.insert(StatusFlags::I),

            BCC_REL | BCS_REL | BEQ_REL | BNE_REL | BPL_REL | BMI_REL | BVC_REL | BVS_REL => {
                let rel = mem.read_i8(cur_addr);
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
                    // +1 if branch occurs on same page, +2 if on different page
                    cycles_additional += if Self::is_page_crossed(self.pc, rel) { 2 } else { 1 };

                    self.pc = self.pc.wrapping_add(rel as u16);     // add/sub relative address
                }
            }

            INC_ZPG | INC_ZPX | INC_ABS | INC_ABX | DEC_ZPG | DEC_ZPX | DEC_ABS | DEC_ABX => {
                // TODO: possible page crossing additional cycle for ZPX and ABX?
                let addr: u16;
                match opcode {
                    INC_ZPG | INC_ZPX | DEC_ZPG | DEC_ZPX => addr = mem.read_u8(cur_addr) as u16 + if opcode == INC_ZPX { self.x } else { 0 } as u16,
                    INC_ABS | INC_ABX | DEC_ABS | DEC_ABX => addr = mem.read_u16(cur_addr) + if opcode == INC_ABX { self.x } else { 0 } as u16,
                    _ => panic!("Undefined INC/DEC opcode {:02X}", opcode),
                }
                let mut value: u8 = mem.read_u8(addr);
                if matches!(opcode, INC_ZPG | INC_ZPX | INC_ABS | INC_ABX) { value = value.wrapping_add(1) } else { value = value.wrapping_sub(1) }
                mem.write_u8(addr, value);
                self.sr.set(StatusFlags::Z, value == 0);
                self.sr.set(StatusFlags::N, value & 0b10000000 != 0);
            },

            _ => panic!("Unimplemented or invalid instruction {:02X} @ {:04X}", opcode, cur_addr - 1),
        }

        cycles_additional
    }
}

impl fmt::Debug for Cpu {
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

#[cfg(test)]
mod tests {
    use crate::mem::ADDR_RESET_VECTOR;

    use super::*;

    fn setup() -> (Cpu, Memory) {
        let mut mem = Memory::create();
        let mut cpu = Cpu::create();
        cpu.reset(&mut mem);
        (cpu, mem)
    }

    #[test]
    fn is_page_crossed() {
        assert!(!Cpu::is_page_crossed(0x01FF, -128));   // Target: 0x017F    C-Page: 1   T-Page: 1
        assert!(Cpu::is_page_crossed(0x0200, -128));    // Target: 0x0180    C-Page: 2   T-Page: 1   -> crossed

        assert!(!Cpu::is_page_crossed(0x01FF, -1));     // Target: 0x01FE    C-Page: 1   T-Page: 1
        assert!(Cpu::is_page_crossed(0x0200, -1));      // Target: 0x01FF    C-Page: 2   T-Page: 1   -> crossed

        assert!(Cpu::is_page_crossed(0x01FF, 1));       // Target: 0x0200    C-Page: 1   T-Page: 2   -> crossed
        assert!(!Cpu::is_page_crossed(0x0200, 1));      // Target: 0x0201    C-Page: 2   T-Page: 2

        assert!(Cpu::is_page_crossed(0x01FF, 127));     // Target: 0x027E    C-Page: 1   T-Page: 2   -> crossed
        assert!(!Cpu::is_page_crossed(0x0200, 127));    // Target: 0x027F    C-Page: 2   T-Page: 2
    }

    #[test]
    fn initial_state() {
        let (cpu, _) = setup();

        assert_eq!(cpu.ac, 0);
        assert_eq!(cpu.x, 0);
        assert_eq!(cpu.y, 0);
        assert_eq!(cpu.sr, StatusFlags::RESERVED);
        assert_eq!(cpu.sp, INITIAL_STACK_POINTER);
        assert_eq!(cpu.pc, ADDR_RESET_VECTOR);      // ensures working memory as well

        assert_eq!(cpu.cycles, 0);
    }

    #[test]
    fn ins_nop() {
        let (mut cpu, mut mem) = setup();

        mem.write_u8(ADDR_RESET_VECTOR + 0, NOP);
        let pc_orig = cpu.pc;
        cpu.exec(&mut mem, 1);

        // verify we're at next instruction
        assert_eq!(cpu.pc, pc_orig + 1);

        // verify 2 cycles happened
        assert_eq!(cpu.cycles, Instruction::from_opcode(NOP).unwrap().cycles as u64);
    }

    #[test]
    fn ins_jmp() {
        let (mut cpu, mut mem) = setup();
        let target_addr: u16 = ADDR_RESET_VECTOR + 0x10;
        let target_addr_ind: u16 = 0xAA00;

        // JMP ABS
        cpu.reset(&mut mem);
        mem.write_u8(ADDR_RESET_VECTOR, JMP_ABS);
        mem.write_u16(None, target_addr);
        cpu.exec(&mut mem, 1);
        assert_eq!(cpu.pc, target_addr);

        // JMP IND
        cpu.reset(&mut mem);
        mem.write_u16(target_addr, target_addr_ind);
        mem.write_u8(ADDR_RESET_VECTOR, JMP_IND);
        mem.write_u16(None, target_addr);
        cpu.exec(&mut mem, 1);
        assert_eq!(cpu.pc, target_addr_ind);
    }

    #[test]
    fn ins_bit() {
        let (mut cpu, mut mem) = setup();
        let cpu_ref = &mut cpu;
        let mem_ref = &mut mem;

        for opcode in [BIT_ZPG, BIT_ABS] {
            ins_bit_(cpu_ref, mem_ref, opcode, 0x01, 0x01, StatusFlags::RESERVED);
            ins_bit_(cpu_ref, mem_ref, opcode, 0x01, 0x00, StatusFlags::RESERVED | StatusFlags::Z);
            ins_bit_(cpu_ref, mem_ref, opcode, 0x00, 0x01, StatusFlags::RESERVED | StatusFlags::Z);
            ins_bit_(cpu_ref, mem_ref, opcode, 0x01, StatusFlags::N.bits(), StatusFlags::RESERVED | StatusFlags::Z | StatusFlags::N);
            ins_bit_(cpu_ref, mem_ref, opcode, 0x01, StatusFlags::V.bits(), StatusFlags::RESERVED | StatusFlags::Z | StatusFlags::V);
        }
    }

    fn ins_bit_(cpu: &mut Cpu, mem: &mut Memory, opcode: u8, ac: u8, value: u8, sr_expect: StatusFlags) {
        let addr: u16 = 0x000A;
        cpu.reset(mem);
        cpu.ac = ac;
        mem.write_u8(addr, value);
        mem.write_u8(ADDR_RESET_VECTOR, opcode);
        if opcode == BIT_ZPG {
            mem.write_u8(None, (addr & 0xFF) as u8);
        } else {
            mem.write_u16(None, addr);
        }
        cpu.exec(mem, 1);
        assert_eq!(cpu.sr, sr_expect);
    }

    #[test]
    fn ins_cxxsxx() {
        let (mut cpu, mut mem) = setup();
        let cpu_ref = &mut cpu;
        let mem_ref = &mut mem;

        ins_cxxsxx_(cpu_ref, mem_ref, CLC, StatusFlags::RESERVED | StatusFlags::C, StatusFlags::RESERVED);
        ins_cxxsxx_(cpu_ref, mem_ref, CLD, StatusFlags::RESERVED | StatusFlags::D, StatusFlags::RESERVED);
        ins_cxxsxx_(cpu_ref, mem_ref, CLI, StatusFlags::RESERVED | StatusFlags::I, StatusFlags::RESERVED);
        ins_cxxsxx_(cpu_ref, mem_ref, CLV, StatusFlags::RESERVED | StatusFlags::V, StatusFlags::RESERVED);

        ins_cxxsxx_(cpu_ref, mem_ref, SEC, StatusFlags::RESERVED, StatusFlags::RESERVED | StatusFlags::C);
        ins_cxxsxx_(cpu_ref, mem_ref, SED, StatusFlags::RESERVED, StatusFlags::RESERVED | StatusFlags::D);
        ins_cxxsxx_(cpu_ref, mem_ref, SEI, StatusFlags::RESERVED, StatusFlags::RESERVED | StatusFlags::I);
    }

    fn ins_cxxsxx_(cpu: &mut Cpu, mem: &mut Memory, opcode: u8, sr_before: StatusFlags, sr_expect: StatusFlags) {
        cpu.reset(mem);
        cpu.sr = sr_before;
        mem.write_u8(ADDR_RESET_VECTOR, opcode);
        cpu.exec(mem, 1);
        assert_eq!(cpu.sr, sr_expect);
    }

    #[test]
    fn ins_bxx() {
        let (mut cpu, mut mem) = setup();
        let cpu_ref = &mut cpu;
        let mem_ref = &mut mem;

        // test with both positive and negative relative address
        for rel in [-128, 16, 0, -16, 127] {
            ins_bxx_(cpu_ref, mem_ref, BCC_REL, rel, StatusFlags::C, false);
            ins_bxx_(cpu_ref, mem_ref, BCC_REL, rel, StatusFlags::empty(), true);

            ins_bxx_(cpu_ref, mem_ref, BCS_REL, rel, StatusFlags::C, true);
            ins_bxx_(cpu_ref, mem_ref, BCS_REL, rel, StatusFlags::empty(), false);

            ins_bxx_(cpu_ref, mem_ref, BEQ_REL, rel, StatusFlags::Z, true);
            ins_bxx_(cpu_ref, mem_ref, BEQ_REL, rel, StatusFlags::empty(), false);

            ins_bxx_(cpu_ref, mem_ref, BNE_REL, rel, StatusFlags::Z, false);
            ins_bxx_(cpu_ref, mem_ref, BNE_REL, rel, StatusFlags::empty(), true);

            ins_bxx_(cpu_ref, mem_ref, BPL_REL, rel, StatusFlags::N, false);
            ins_bxx_(cpu_ref, mem_ref, BPL_REL, rel, StatusFlags::empty(), true);

            ins_bxx_(cpu_ref, mem_ref, BMI_REL, rel, StatusFlags::N, true);
            ins_bxx_(cpu_ref, mem_ref, BMI_REL, rel, StatusFlags::empty(), false);

            ins_bxx_(cpu_ref, mem_ref, BVC_REL, rel, StatusFlags::V, false);
            ins_bxx_(cpu_ref, mem_ref, BVC_REL, rel, StatusFlags::empty(), true);

            ins_bxx_(cpu_ref, mem_ref, BVS_REL, rel, StatusFlags::V, true);
            ins_bxx_(cpu_ref, mem_ref, BVS_REL, rel, StatusFlags::empty(), false);
        }
    }

    fn ins_bxx_(cpu: &mut Cpu, mem: &mut Memory, opcode: u8, rel: i8, srf: StatusFlags, jmp: bool) {
        let addr_nobranch = ADDR_RESET_VECTOR + 2;
        let addr_branch = (ADDR_RESET_VECTOR + 2 as u16).wrapping_add(rel as u16);

        cpu.reset(mem);
        cpu.sr.insert(srf);
        mem.write_u8(ADDR_RESET_VECTOR, opcode);
        mem.write_i8(None, rel);
        cpu.exec(mem, 1);
        assert_eq!(cpu.pc, if jmp { addr_branch } else {addr_nobranch});

        let mut expected_cycles = 2;
        if jmp {
            // jump occured: same page -> +1, page crossed -> +2
            expected_cycles += if Cpu::is_page_crossed(ADDR_RESET_VECTOR + 2, rel) { 2 } else { 1 };
        }
        assert_eq!(cpu.cycles, expected_cycles);
    }

    #[test]
    fn ins_incdec() {
        let (mut cpu, mut mem) = setup();

        for opcode in [INC_ZPG, INC_ZPX | INC_ABS | INC_ABX | DEC_ZPG, DEC_ZPX | DEC_ABS | DEC_ABX] {
            for value in [0xFE, 0xFF] {
                let rel_addr: u8 = 0xAA;
                let abs_addr: u16 = 0xCAFE;

                cpu.reset(&mut mem);
                mem.write_u8(ADDR_RESET_VECTOR, opcode);

                let mut addr: u16;
                match opcode {
                    INC_ZPG | INC_ZPX | DEC_ZPG | DEC_ZPX => {
                        addr = rel_addr as u16;
                        mem.write_u8(None, rel_addr);
                    },
                    INC_ABS | INC_ABX | DEC_ABS | DEC_ABX => {
                        addr = abs_addr;
                        mem.write_u16(None, abs_addr);
                    },
                    _ => panic!("Unhandled test case INC/DEC {:02X}", opcode)
                }
                
                if matches!(opcode, INC_ZPX | INC_ABX) {
                    cpu.x = 1;
                    addr = addr.wrapping_add(cpu.x as u16);
                }
                mem.write_u8(addr, value);      // memory location that gets incremented
                cpu.exec(&mut mem, 1);

                let result = mem.read_u8(addr);
                assert_eq!(result, if matches!(opcode, INC_ZPG | INC_ZPX | INC_ABS | INC_ABX) { value.wrapping_add(1) } else { value.wrapping_sub(1) });
                if result == 0 { assert!(cpu.sr.contains(StatusFlags::Z),) }
                if result & 0b10000000 != 0 { assert!(cpu.sr.contains(StatusFlags::N)) }
            }
        }
    }
}
