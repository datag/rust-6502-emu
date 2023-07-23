
use std::fmt;
use bitflags::bitflags;
use crate::instruction::*;
use crate::mem::Memory;

pub const VECTOR_RES: u16 = 0xFFFC;                     // 0xFFFC LB, 0xFFFF HB for reading reset vector address
pub const STACK_BASE: u16 = 0x0100;                     // 0x0100 to 0x01FF
pub const INITIAL_STACK_POINTER: u8 = 0xFD;             // [0x0100 - 0x01FF] in memory; CPU starts with SP=0 and decrements 3x which is 0xFD
pub const CYCLES_AFTER_RESET: u64 = 7;                  // after reset 7 cycles already happend

bitflags! {
    #[derive(Clone, Copy, PartialEq, Debug)]
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

    #[allow(dead_code)]
    fn is_page_crossed(cur_addr: u16, rel: i8) -> bool {
        let target_addr = cur_addr.wrapping_add(rel as u16);
        Self::is_page_different(cur_addr, target_addr)
    }

    fn is_page_different(cur_addr: u16, target_addr: u16) -> bool {
        // divide current address by 256 (0x100) to get the current page
        let current_page = cur_addr >> 8;

        // calculate the target page
        let target_page = target_addr >> 8;

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
        self.cycles = CYCLES_AFTER_RESET;
    }

    pub fn exec(&mut self, mem: &mut Memory, max_cycles: u64) {
        println!("[before   ] {:?}", &self);

        let mut cycles_to_execute = max_cycles;
        let mut opcode: u8;
        let mut cur_addr: u16;

        while cycles_to_execute > 0 {
            // load instruction from mem at PC
            opcode = mem.read_u8(self.pc);

            // advance read address by 1 read opcode byte
            cur_addr = self.pc + 1;

            let result = Instruction::from_opcode(opcode);
            match result {
                Ok(ins) => {
                    println!(">> @{:04X} {:?}", self.pc, ins);
            
                    // advance PC by instruction bytes
                    self.pc += ins.bytes as u16;

                    // handle the opcode
                    let cycles_additional = self.handle_opcode(mem, &ins, cur_addr);
                    let cycles_consumed = ins.cycles + cycles_additional;
        
                    // decrease remaining cycle counter 
                    cycles_to_execute = cycles_to_execute.saturating_sub(cycles_consumed as u64);

                    // [debug] increase global cycles counter
                    self.cycles = self.cycles.saturating_add(cycles_consumed as u64);

                    println!("[after {:?}] {:?}\n", ins.mnemonic, self);
                },
                Err(()) => panic!("Unimplemented or invalid instruction {:02X} @ {:04X}", opcode, self.pc),
            }
        }
    }

    fn addr_zpg(&self, addr: u8) -> u16 {
        0x0000 | (addr as u16)
    }

    fn fetch_addr_zpg(&self, mem: &Memory, addr: u16) -> u16 {
        self.addr_zpg(mem.read_u8(addr))
    }

    fn addr_zpx(&self, addr: u8) -> u16 {
        addr.wrapping_add(self.x) as u16      // wrap around zero page  (= without carry)
    }

    fn fetch_addr_zpx(&self, mem: &Memory, addr: u16) -> u16 {
        self.addr_zpx(mem.read_u8(addr))
    }

    fn addr_zpy(&self, addr: u8) -> u16 {
        addr.wrapping_add(self.y) as u16      // wrap around zero page  (= without carry)
    }

    fn fetch_addr_zpy(&self, mem: &Memory, addr: u16) -> u16 {
        self.addr_zpy(mem.read_u8(addr))
    }

    fn addr_abs(&self, addr: u16) -> u16 {
        addr
    }

    fn fetch_addr_abs(&self, mem: &Memory, addr: u16) -> u16 {
        self.addr_abs(mem.read_u16(addr))
    }

    fn addr_abx(&self, addr: u16) -> u16 {
        addr.wrapping_add(self.x as u16)
    }

    fn fetch_addr_abx(&self, mem: &Memory, addr: u16) -> u16 {
        self.addr_abx(mem.read_u16(addr))
    }

    fn addr_aby(&self, addr: u16) -> u16 {
        addr.wrapping_add(self.y as u16)
    }

    fn fetch_addr_aby(&self, mem: &Memory, addr: u16) -> u16 {
        self.addr_aby(mem.read_u16(addr))
    }

    fn addr_ind(&self, mem: &Memory, addr: u16) -> u16 {
        mem.read_u16(addr)
    }

    fn fetch_addr_ind(&self, mem: &Memory, addr: u16) -> u16 {
        self.addr_ind(mem, mem.read_u16(addr))
    }

    fn addr_idx(&self, mem: &Memory, addr: u8) -> u16 {
        mem.read_u16(addr.wrapping_add(self.x) as u16)
    }

    fn fetch_addr_idx(&self, mem: &Memory, addr: u16) -> u16 {
        self.addr_idx(mem, mem.read_u8(addr))
    }

    fn addr_idy(&self, mem: &Memory, addr: u8) -> u16 {
        mem.read_u16(addr as u16).wrapping_add(self.y as u16)
    }

    fn fetch_addr_idy(&self, mem: &Memory, addr: u16) -> u16 {
        self.addr_idy(mem, mem.read_u8(addr))
    }

    fn addr_rel(&self, rel: i8) -> u16 {
        self.pc.wrapping_add(rel as u16)     // add/sub relative address
    }

    fn fetch_addr_rel(&self, mem: &Memory, addr: u16) -> u16 {
        self.addr_rel(mem.read_i8(addr))
    }

    fn fetch_addr(&self, mem: &Memory, ins: &Instruction, addr: u16) -> u16 {
        match ins.addr_mode {
            AddressingMode::IMM => addr,
            AddressingMode::ZPG => self.fetch_addr_zpg(mem, addr),
            AddressingMode::ZPX => self.fetch_addr_zpx(mem, addr),
            AddressingMode::ZPY => self.fetch_addr_zpy(mem, addr),
            AddressingMode::ABS => self.fetch_addr_abs(mem, addr),
            AddressingMode::ABX => self.fetch_addr_abx(mem, addr),
            AddressingMode::ABY => self.fetch_addr_aby(mem, addr),
            AddressingMode::IND => self.fetch_addr_ind(mem, addr),
            AddressingMode::IDX => self.fetch_addr_idx(mem, addr),
            AddressingMode::IDY => self.fetch_addr_idy(mem, addr),
            _ => panic!("Unhandled address mode {}", ins.addr_mode),
        }
    }
    
    fn handle_opcode(&mut self, mem: &mut Memory, ins: &Instruction, cur_addr: u16) -> u8 {
        let opcode = ins.opcode;
        let mut cycles_additional = 0;

        match opcode {
            NOP => println!("NOP"),

            ADC_IMM | ADC_ZPG | ADC_ZPX | ADC_ABS | ADC_ABX | ADC_ABY | ADC_IDX | ADC_IDY
            | SBC_IMM | SBC_ZPG | SBC_ZPX | SBC_ABS | SBC_ABX | SBC_ABY | SBC_IDX | SBC_IDY => {
                // TODO: possible page crossing additional cycle for ZPX, ABX and ABY?

                // TODO: BCD mode
                if self.sr.contains(StatusFlags::D) {
                    panic!("BCD mode not yet implemented");
                }

                let addr = self.fetch_addr(mem, ins, cur_addr);
                let value: u8 = mem.read_u8(addr);
                println!("oper: 0x{:02X}", value);

                let result: u8;
                if ins.mnemonic == Mnemonic::ADC {
                    let sum = (self.ac as u16) + value as u16 + if self.sr.contains(StatusFlags::C) { 1 } else { 0 } as u16;
                    result = (sum & 0xFF) as u8;
                    
                    self.sr.set(StatusFlags::C, sum > 255);
                    self.sr.set(StatusFlags::V, (!(self.ac ^ value) & (self.ac ^ result) & 0x80) != 0);
                } else {
                    let difference = (self.ac as u16) - value as u16 - if self.sr.contains(StatusFlags::C) { 0 } else { 1 };
                    result = (difference & 0xFF) as u8;

                    self.sr.set(StatusFlags::C, difference < 256);      // acts as borrow flag
                    self.sr.set(StatusFlags::V, ((self.ac ^ value) & (self.ac ^ result) & 0x80) != 0);
                }
                println!("AC is now: 0x{:02X}", result);

                self.sr.set(StatusFlags::N, result & 0b10000000 != 0);
                self.sr.set(StatusFlags::Z, result == 0);
                self.ac = result;
            }

            JMP_ABS | JMP_IND => self.pc = self.fetch_addr(mem, ins, cur_addr),

            BIT_ZPG | BIT_ABS => {
                let addr = self.fetch_addr(mem, ins, cur_addr);
                let value = mem.read_u8(addr);
                // println!("addr: {:04X} value: {:02X} result: {:02X}", addr, value, value & self.ac);
                self.sr.set(StatusFlags::N, value & StatusFlags::N.bits() != 0);    // transfer bit 7 of operand to N
                self.sr.set(StatusFlags::V, value & StatusFlags::V.bits() != 0);    // transfer bit 6 of operand to V
                self.sr.set(StatusFlags::Z, value & self.ac == 0);                  // result of operand and AC
            },

            AND_IMM | AND_ZPG | AND_ZPX | AND_ABS | AND_ABX | AND_ABY | AND_IDX | AND_IDY
            | EOR_IMM | EOR_ZPG | EOR_ZPX | EOR_ABS | EOR_ABX | EOR_ABY | EOR_IDX | EOR_IDY
            | ORA_IMM | ORA_ZPG | ORA_ZPX | ORA_ABS | ORA_ABX | ORA_ABY | ORA_IDX | ORA_IDY => {
                let addr = self.fetch_addr(mem, ins, cur_addr);
            
                let value: u8 = mem.read_u8(addr);
                println!("oper: 0x{:02X} @{:04X}", value, addr);

                self.ac = match ins.mnemonic {
                    Mnemonic::AND => self.ac & value,
                    Mnemonic::EOR => self.ac ^ value,
                    Mnemonic::ORA => self.ac | value,
                    _ => panic!("Unhandled mnemonic {:?}", ins.mnemonic),
                };

                self.sr.set(StatusFlags::N, self.ac & 0b10000000 != 0);
                self.sr.set(StatusFlags::Z, self.ac == 0);
            },

            CLC => self.sr.remove(StatusFlags::C),
            CLD => self.sr.remove(StatusFlags::D),
            CLI => self.sr.remove(StatusFlags::I),
            CLV => self.sr.remove(StatusFlags::V),
            SEC => self.sr.insert(StatusFlags::C),
            SED => self.sr.insert(StatusFlags::D),
            SEI => self.sr.insert(StatusFlags::I),

            BCC_REL | BCS_REL | BEQ_REL | BNE_REL | BPL_REL | BMI_REL | BVC_REL | BVS_REL => {
                let jmp = match opcode {
                    BCC_REL => !self.sr.contains(StatusFlags::C),
                    BCS_REL => self.sr.contains(StatusFlags::C),
                    BEQ_REL => self.sr.contains(StatusFlags::Z),
                    BNE_REL => !self.sr.contains(StatusFlags::Z),
                    BPL_REL => !self.sr.contains(StatusFlags::N),
                    BMI_REL => self.sr.contains(StatusFlags::N),
                    BVC_REL => !self.sr.contains(StatusFlags::V),
                    BVS_REL => self.sr.contains(StatusFlags::V),
                    _ => panic!("Unhandled branch opcode {:02X}", opcode),
                };
                // println!("jmp: {}", jmp);
                if jmp {
                    let addr = self.fetch_addr_rel(mem, cur_addr);

                    // +1 if branch occurs on same page, +2 if on different page
                    cycles_additional += if Self::is_page_different(self.pc, addr) { 2 } else { 1 };
                    self.pc = addr;
                }
            }

            INC_ZPG | INC_ZPX | INC_ABS | INC_ABX | DEC_ZPG | DEC_ZPX | DEC_ABS | DEC_ABX => {
                // TODO: possible page crossing additional cycle for ZPX and ABX?
                let addr = self.fetch_addr(mem, ins, cur_addr);
                let mut value: u8 = mem.read_u8(addr);

                if ins.mnemonic == Mnemonic::INC { value = value.wrapping_add(1) } else { value = value.wrapping_sub(1) }
                mem.write_u8(addr, value);
                self.sr.set(StatusFlags::Z, value == 0);
                self.sr.set(StatusFlags::N, value & 0b10000000 != 0);
            },

            INX | INY | DEX | DEY => {
                let mut value: u8 = match opcode {
                    INX | DEX => self.x,
                    INY | DEY => self.y,
                    _ => panic!("Undefined INC/DEC opcode {:02X}", opcode),
                };

                if matches!(opcode, INX | INY) { value = value.wrapping_add(1) } else { value = value.wrapping_sub(1) }
                if matches!(opcode, INX | DEX) { self.x = value } else { self.y = value }

                self.sr.set(StatusFlags::Z, value == 0);
                self.sr.set(StatusFlags::N, value & 0b10000000 != 0);
            },

            LDA_IMM | LDA_ZPG | LDA_ZPX | LDA_ABS | LDA_ABX | LDA_ABY | LDA_IDX | LDA_IDY
            | LDX_IMM | LDX_ZPG | LDX_ZPY | LDX_ABS | LDX_ABY
            | LDY_IMM | LDY_ZPG | LDY_ZPY | LDY_ABS | LDY_ABY => {
                // TODO: possible page crossing additional cycle for LDA: ABX, ABY and IDX  and LDX/LDY: ABX?
                let addr = self.fetch_addr(mem, ins, cur_addr);
                let value: u8 = mem.read_u8(addr);
                println!("oper: 0x{:02X}", value);

                match ins.mnemonic {
                    Mnemonic::LDA => self.ac = value,
                    Mnemonic::LDX => self.x = value,
                    Mnemonic::LDY => self.y = value,
                    _ => panic!("Unhandled LD* opcode {:02X}", opcode),
                }

                self.sr.set(StatusFlags::Z, value == 0);
                self.sr.set(StatusFlags::N, value & 0b10000000 != 0);
            },

            STA_ZPG | STA_ZPX | STA_ABS | STA_ABX | STA_ABY | STA_IDX | STA_IDY
             | STX_ZPG | STX_ZPY | STX_ABS
             | STY_ZPG | STY_ZPX | STY_ABS => {
                let addr = self.fetch_addr(mem, ins, cur_addr);
                let value = match ins.mnemonic {
                    Mnemonic::STA => self.ac,
                    Mnemonic::STX => self.x,
                    Mnemonic::STY => self.y,
                    _ => panic!("Unhandled ST* opcpde {:02X}", opcode),
                };
                mem.write_u8(addr, value);
            },

            TAX | TAY | TSX | TXA | TXS | TYA => {
                let value = match ins.opcode {
                    TAY | TAX => self.ac,
                    TXA | TXS => self.x,
                    TYA       => self.y,
                    TSX       => self.sp,
                    _ => panic!("Unhandled T** opcpde {:02X}", opcode),
                };

                match ins.opcode {
                    TXA | TYA => self.ac = value,
                    TAX | TSX => self.x = value,
                    TAY       => self.y = value,
                    TXS       => self.sp = value,
                    _ => panic!("Unhandled T** opcpde {:02X}", opcode),
                };

                if opcode != TXS {      // no setting SR N/Z flags for TXS
                    self.sr.set(StatusFlags::Z, value == 0);
                    self.sr.set(StatusFlags::N, value & 0b10000000 != 0);
                }
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

        assert_eq!(cpu.cycles, CYCLES_AFTER_RESET);
    }

    #[test]
    fn fetch_addr_zpx() {
        let (cpu, mut mem) = setup();

        let addr: u8 = 0xF0;
        let addr_expected: u16 = addr as u16;
        let data: u8 = 0xAA;
        mem.write_u8(addr_expected, data);
        mem.write_u8(ADDR_RESET_VECTOR + 0, NOP /* opcode does not matter */);
        mem.write_u8(ADDR_RESET_VECTOR + 1, addr);

        let addr_effective = cpu.fetch_addr_zpg(&mem, ADDR_RESET_VECTOR + 1);
        println!("addr: {:02X}  expected_addr: {:04X}  effective addr: {:04X}", addr, addr_expected, addr_effective);
        assert_eq!(addr_effective, addr_expected);
        assert_eq!(mem.read_u8(addr_effective), data);
    }

    #[test]
    fn fetch_addr_pgxy() {
        let (mut cpu, mut mem) = setup();

        let addr: u8 = 0x80;
        let addr_expected: u16 = 0x8F;
        let data: u8 = 0xAA;

        cpu.reset(&mut mem);
        cpu.x = 0x0F;
        mem.write_u8(addr_expected, data);
        mem.write_u8(ADDR_RESET_VECTOR + 0, NOP /* opcode does not matter */);
        mem.write_u8(ADDR_RESET_VECTOR + 1, addr);
        let addr_effective = cpu.fetch_addr_zpx(&mem, ADDR_RESET_VECTOR + 1);
        println!("addr: {:02X}  expected_addr: {:04X}  effective addr: {:04X}", addr, addr_expected, addr_effective);
        assert_eq!(addr_effective, addr_expected);
        assert_eq!(mem.read_u8(addr_effective), data);

        cpu.reset(&mut mem);
        cpu.y = 0x0F;
        mem.write_u8(addr_expected, data);
        mem.write_u8(ADDR_RESET_VECTOR + 0, NOP /* opcode does not matter */);
        mem.write_u8(ADDR_RESET_VECTOR + 1, addr);
        let addr_effective = cpu.fetch_addr_zpy(&mem, ADDR_RESET_VECTOR + 1);
        println!("addr: {:02X}  expected_addr: {:04X}  effective addr: {:04X}", addr, addr_expected, addr_effective);
        assert_eq!(addr_effective, addr_expected);
        assert_eq!(mem.read_u8(addr_effective), data);
    }

    #[test]
    fn fetch_addr_abs() {
        let (cpu, mut mem) = setup();

        let addr: u16 = 0xA000;
        let addr_expected: u16 = addr;
        let data: u8 = 0xAA;
        mem.write_u8(addr_expected, data);
        mem.write_u8(ADDR_RESET_VECTOR + 0, NOP /* opcode does not matter */);
        mem.write_u16(ADDR_RESET_VECTOR + 1, addr);

        let addr_effective = cpu.fetch_addr_abs(&mem, ADDR_RESET_VECTOR + 1);
        println!("addr: {:02X}  expected_addr: {:04X}  effective addr: {:04X}", addr, addr_expected, addr_effective);
        assert_eq!(addr_effective, addr_expected);
        assert_eq!(mem.read_u8(addr_effective), data);
    }

    #[test]
    fn fetch_addr_abxy() {
        let (mut cpu, mut mem) = setup();

        let addr: u16 = 0xA000;
        let data: u8 = 0xAA;

        cpu.reset(&mut mem);
        cpu.x = 0x0F;
        let addr_expected: u16 = addr.wrapping_add(cpu.x as u16);
        mem.write_u8(addr_expected, data);
        mem.write_u8(ADDR_RESET_VECTOR + 0, NOP /* opcode does not matter */);
        mem.write_u16(ADDR_RESET_VECTOR + 1, addr);

        let addr_effective = cpu.fetch_addr_abx(&mem, ADDR_RESET_VECTOR + 1);
        println!("addr: {:02X}  expected_addr: {:04X}  effective addr: {:04X}", addr, addr_expected, addr_effective);
        assert_eq!(addr_effective, addr_expected);
        assert_eq!(mem.read_u8(addr_effective), data);

        cpu.reset(&mut mem);
        cpu.y = 0x0F;
        let addr_expected: u16 = addr.wrapping_add(cpu.y as u16);
        mem.write_u8(addr_expected, data);
        mem.write_u8(ADDR_RESET_VECTOR + 0, NOP /* opcode does not matter */);
        mem.write_u16(ADDR_RESET_VECTOR + 1, addr);

        let addr_effective = cpu.fetch_addr_aby(&mem, ADDR_RESET_VECTOR + 1);
        println!("addr: {:02X}  expected_addr: {:04X}  effective addr: {:04X}", addr, addr_expected, addr_effective);
        assert_eq!(addr_effective, addr_expected);
        assert_eq!(mem.read_u8(addr_effective), data);
    }

    #[test]
    fn fetch_addr_ind() {
        let (cpu, mut mem) = setup();

        let addr: u16 = 0xA000;
        let addr_expected: u16 = 0x0B00;
        let data: u8 = 0xAA;
        mem.write_u16(addr, addr_expected);     // address holds indirect address
        mem.write_u8(addr_expected, data);      // indirect address holds data
        mem.write_u8(ADDR_RESET_VECTOR + 0, NOP /* opcode does not matter */);
        mem.write_u16(ADDR_RESET_VECTOR + 1, addr);

        let addr_effective = cpu.fetch_addr_ind(&mem, ADDR_RESET_VECTOR + 1);
        println!("addr: {:02X}  expected_addr: {:04X}  effective addr: {:04X}", addr, addr_expected, addr_effective);
        assert_eq!(addr_effective, addr_expected);
        assert_eq!(mem.read_u8(addr_effective), data);
    }

    #[test]
    fn fetch_addr_idxy() {
        let (mut cpu, mut mem) = setup();

        let addr: u8 = 0xF0;
        let data: u8 = 0xAA;

        cpu.reset(&mut mem);
        let addr_expected: u16 = 0x0B00;
        cpu.x = 3;
        mem.write_u16(addr.wrapping_add(cpu.x) as u16, addr_expected);     // address holds indirect address
        mem.write_u8(addr_expected, data);      // indirect address holds data
        mem.write_u8(ADDR_RESET_VECTOR + 0, NOP /* opcode does not matter */);
        mem.write_u8(ADDR_RESET_VECTOR + 1, addr);

        let addr_effective = cpu.fetch_addr_idx(&mem, ADDR_RESET_VECTOR + 1);
        println!("addr: {:02X}  expected_addr: {:04X}  effective addr: {:04X}", addr, addr_expected, addr_effective);
        assert_eq!(addr_effective, addr_expected);
        assert_eq!(mem.read_u8(addr_effective), data);

        cpu.reset(&mut mem);
        let addr_expected: u16 = 0x0B03;
        cpu.y = 3;
        mem.write_u16(addr as u16, addr_expected.wrapping_sub(cpu.y as u16));     // address holds indirect address
        mem.write_u8(addr_expected, data);      // indirect address holds data
        mem.write_u8(ADDR_RESET_VECTOR + 0, NOP /* opcode does not matter */);
        mem.write_u8(ADDR_RESET_VECTOR + 1, addr);

        let addr_effective = cpu.fetch_addr_idy(&mem, ADDR_RESET_VECTOR + 1);
        println!("addr: {:02X}  expected_addr: {:04X}  effective addr: {:04X}", addr, addr_expected, addr_effective);
        assert_eq!(addr_effective, addr_expected);
        assert_eq!(mem.read_u8(addr_effective), data);
    }

    #[test]
    fn fetch_addr_rel() {
        let (cpu, mut mem) = setup();

        let addr: i8 = -10;
        let addr_expected: u16 = cpu.pc.wrapping_add(addr as u16);
        let data: u8 = 0xAA;
        mem.write_u8(addr_expected, data);
        mem.write_u8(ADDR_RESET_VECTOR + 0, NOP /* opcode does not matter */);
        mem.write_i8(ADDR_RESET_VECTOR + 1, addr);

        let addr_effective = cpu.fetch_addr_rel(&mem, ADDR_RESET_VECTOR + 1);
        println!("addr: {:02X}  expected_addr: {:04X}  effective addr: {:04X}", addr, addr_expected, addr_effective);
        assert_eq!(addr_effective, addr_expected);
        assert_eq!(mem.read_u8(addr_effective), data);
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
        assert_eq!(cpu.cycles, CYCLES_AFTER_RESET + Instruction::from_opcode(NOP).unwrap().cycles as u64);
    }

    #[test]
    fn ins_adcsbc() {
        let (mut cpu, mut mem) = setup();

        for (opcode, ac, value, carry, value_expect, sr_expect) in [
            // ADC
            (ADC_IMM, 0x01, 0x01, false, 0x02, StatusFlags::RESERVED),
            (ADC_IMM, 0x7F, 0x01, false, 0x80, StatusFlags::RESERVED | StatusFlags::N | StatusFlags::V),
            (ADC_IMM, 0x7F, 0x00, true,  0x80, StatusFlags::RESERVED | StatusFlags::N | StatusFlags::V),      // test if carry is taken into account
            (ADC_IMM, 0xfF, 0xFF, false, 0xFE, StatusFlags::RESERVED | StatusFlags::N | StatusFlags::C),

            // SBC
            (SBC_IMM, 0x02, 0x01, false, 0x00, StatusFlags::RESERVED | StatusFlags::C | StatusFlags::Z),
            (SBC_IMM, 0x03, 0x01, false, 0x01, StatusFlags::RESERVED | StatusFlags::C),
            (SBC_IMM, 0x03, 0x00, true,  0x03, StatusFlags::RESERVED | StatusFlags::C),                      // test if carry is taken into account
            (SBC_IMM, 0xFF, 0x01, false, 0xFD, StatusFlags::RESERVED | StatusFlags::C | StatusFlags::N),
        ] {
            let addr: u16 = 0x000A;
            cpu.reset(&mut mem);
            cpu.ac = ac;
            cpu.sr.set(StatusFlags::C, carry);
            mem.write_u8(addr, value);
            mem.write_u8(ADDR_RESET_VECTOR, opcode);
            mem.write_u8(None, value);
            cpu.exec(&mut mem, 1);
            assert_eq!(cpu.ac, value_expect);
            assert_eq!(cpu.sr, sr_expect);
        }
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

        for opcode in [BIT_ZPG, BIT_ABS] {
            for (ac, value, sr_expect) in [
                (0x01, 0x01, StatusFlags::RESERVED),
                (0x01, 0x00, StatusFlags::RESERVED | StatusFlags::Z),
                (0x00, 0x01, StatusFlags::RESERVED | StatusFlags::Z),
                (0x01, StatusFlags::N.bits(), StatusFlags::RESERVED | StatusFlags::Z | StatusFlags::N),
                (0x01, StatusFlags::V.bits(), StatusFlags::RESERVED | StatusFlags::Z | StatusFlags::V),
            ] {
                let addr: u16 = 0x000A;
                cpu.reset(&mut mem);
                cpu.ac = ac;
                mem.write_u8(addr, value);
                mem.write_u8(ADDR_RESET_VECTOR, opcode);
                if opcode == BIT_ZPG {
                    mem.write_u8(None, (addr & 0xFF) as u8);
                } else {
                    mem.write_u16(None, addr);
                }
                cpu.exec(&mut mem, 1);
                assert_eq!(cpu.sr, sr_expect);
            }
        }
    }

    #[test]
    fn ins_and() {
        let (mut cpu, mut mem) = setup();

        for opcode in [AND_IMM, AND_ZPG, AND_ZPX, AND_ABS, AND_ABX, AND_ABY, AND_IDX, AND_IDY] {
            for (ac, value, ac_expect, sr_expect) in [
                (0x00, 0x00, 0x00, StatusFlags::RESERVED | StatusFlags::Z),
                (0x01, 0x00, 0x00, StatusFlags::RESERVED | StatusFlags::Z),
                (0x00, 0x01, 0x00, StatusFlags::RESERVED | StatusFlags::Z),
                (0xA1, 0x0F, 0x01, StatusFlags::RESERVED),
                (0xFF, 0xF0, 0xF0, StatusFlags::RESERVED | StatusFlags::N),
            ] {
                cpu.reset(&mut mem);
                cpu.ac = ac;

                let addr: u16 = 0x000A;
                println!("ac:{:02X} value:{:02X} ac_expect:{:?} sf_expect:{:?}", ac, value, ac_expect, sr_expect);
                cpu.x = 0;
                cpu.y = 0;
                if matches!(opcode, AND_ZPG | AND_ZPX | AND_ABS | AND_ABX | AND_ABY) {
                    mem.write_u8(addr, value);
                } else if matches!(opcode, AND_IDX | AND_IDY) {
                    mem.write_u16(addr, addr + 2);
                    mem.write_u8(addr + 2, value);
                }
                mem.write_u8(ADDR_RESET_VECTOR, opcode);
                if opcode == AND_IMM {
                    mem.write_u8(None, value);
                } else if matches!(opcode, AND_ZPG | AND_ZPX | AND_IDX | AND_IDY) {
                    mem.write_u8(None, (addr & 0xFF) as u8);
                } else {
                    mem.write_u16(None, addr);
                }

                cpu.exec(&mut mem, 1);
                assert_eq!(cpu.ac, ac_expect);
                assert_eq!(cpu.sr, sr_expect);
            }
        }
    }

    #[test]
    fn ins_ora() {
        let (mut cpu, mut mem) = setup();

        for opcode in [ORA_IMM, ORA_ZPG, ORA_ZPX, ORA_ABS, ORA_ABX, ORA_ABY, ORA_IDX, ORA_IDY] {
            for (ac, value, ac_expect, sr_expect) in [
                (0x00, 0x00, 0x00, StatusFlags::RESERVED | StatusFlags::Z),
                (0x01, 0x00, 0x01, StatusFlags::RESERVED),
                (0x00, 0x01, 0x01, StatusFlags::RESERVED),
                (0x01, 0x01, 0x01, StatusFlags::RESERVED),
                (0xF0, 0x0F, 0xFF, StatusFlags::RESERVED | StatusFlags::N),
            ] {
                cpu.reset(&mut mem);
                cpu.ac = ac;

                let addr: u16 = 0x000A;
                println!("ac:{:02X} value:{:02X} ac_expect:{:?} sf_expect:{:?}", ac, value, ac_expect, sr_expect);
                cpu.x = 0;
                cpu.y = 0;
                if matches!(opcode, ORA_ZPG | ORA_ZPX | ORA_ABS | ORA_ABX | ORA_ABY) {
                    mem.write_u8(addr, value);
                } else if matches!(opcode, ORA_IDX | ORA_IDY) {
                    mem.write_u16(addr, addr + 2);
                    mem.write_u8(addr + 2, value);
                }
                mem.write_u8(ADDR_RESET_VECTOR, opcode);
                if opcode == ORA_IMM {
                    mem.write_u8(None, value);
                } else if matches!(opcode, ORA_ZPG | ORA_ZPX | ORA_IDX | ORA_IDY) {
                    mem.write_u8(None, (addr & 0xFF) as u8);
                } else {
                    mem.write_u16(None, addr);
                }

                cpu.exec(&mut mem, 1);
                assert_eq!(cpu.ac, ac_expect);
                assert_eq!(cpu.sr, sr_expect);
            }
        }
    }

    #[test]
    fn ins_eor() {
        let (mut cpu, mut mem) = setup();

        for opcode in [EOR_IMM, EOR_ZPG, EOR_ZPX, EOR_ABS, EOR_ABX, EOR_ABY, EOR_IDX, EOR_IDY] {
            for (ac, value, ac_expect, sr_expect) in [
                (0x00, 0x00, 0x00, StatusFlags::RESERVED | StatusFlags::Z),
                (0x01, 0x00, 0x01, StatusFlags::RESERVED),
                (0x00, 0x01, 0x01, StatusFlags::RESERVED),
                (0x01, 0x01, 0x00, StatusFlags::RESERVED | StatusFlags::Z),
                (0xF0, 0x0F, 0xFF, StatusFlags::RESERVED | StatusFlags::N),
            ] {
                cpu.reset(&mut mem);
                cpu.ac = ac;

                let addr: u16 = 0x000A;
                println!("ac:{:02X} value:{:02X} ac_expect:{:?} sf_expect:{:?}", ac, value, ac_expect, sr_expect);
                cpu.x = 0;
                cpu.y = 0;
                if matches!(opcode, EOR_ZPG | EOR_ZPX | EOR_ABS | EOR_ABX | EOR_ABY) {
                    mem.write_u8(addr, value);
                } else if matches!(opcode, EOR_IDX | EOR_IDY) {
                    mem.write_u16(addr, addr + 2);
                    mem.write_u8(addr + 2, value);
                }
                mem.write_u8(ADDR_RESET_VECTOR, opcode);
                if opcode == EOR_IMM {
                    mem.write_u8(None, value);
                } else if matches!(opcode, EOR_ZPG | EOR_ZPX | EOR_IDX | EOR_IDY) {
                    mem.write_u8(None, (addr & 0xFF) as u8);
                } else {
                    mem.write_u16(None, addr);
                }

                cpu.exec(&mut mem, 1);
                assert_eq!(cpu.ac, ac_expect);
                assert_eq!(cpu.sr, sr_expect);
            }
        }
    }

    #[test]
    fn ins_cxxsxx() {
        let (mut cpu, mut mem) = setup();

        for (opcode, sr_before, sr_expect) in [
            (CLC, StatusFlags::RESERVED | StatusFlags::C, StatusFlags::RESERVED),
            (CLD, StatusFlags::RESERVED | StatusFlags::D, StatusFlags::RESERVED),
            (CLI, StatusFlags::RESERVED | StatusFlags::I, StatusFlags::RESERVED),
            (CLV, StatusFlags::RESERVED | StatusFlags::V, StatusFlags::RESERVED),

            (SEC, StatusFlags::RESERVED, StatusFlags::RESERVED | StatusFlags::C),
            (SED, StatusFlags::RESERVED, StatusFlags::RESERVED | StatusFlags::D),
            (SEI, StatusFlags::RESERVED, StatusFlags::RESERVED | StatusFlags::I),
        ] {
            cpu.reset(&mut mem);
            cpu.sr = sr_before;
            mem.write_u8(ADDR_RESET_VECTOR, opcode);
            cpu.exec(&mut mem, 1);
            assert_eq!(cpu.sr, sr_expect);
        }
    }

    #[test]
    fn ins_bxx() {
        let (mut cpu, mut mem) = setup();

        // test with both positive and negative relative address
        for rel in [-128, 16, 0, -16, 127] {
            for (opcode, srf, jmp) in [
                (BCC_REL, StatusFlags::C, false),
                (BCC_REL, StatusFlags::empty(), true),

                (BCS_REL, StatusFlags::C, true),
                (BCS_REL, StatusFlags::empty(), false),

                (BEQ_REL, StatusFlags::Z, true),
                (BEQ_REL, StatusFlags::empty(), false),

                (BNE_REL, StatusFlags::Z, false),
                (BNE_REL, StatusFlags::empty(), true),

                (BPL_REL, StatusFlags::N, false),
                (BPL_REL, StatusFlags::empty(), true),

                (BMI_REL, StatusFlags::N, true),
                (BMI_REL, StatusFlags::empty(), false),

                (BVC_REL, StatusFlags::V, false),
                (BVC_REL, StatusFlags::empty(), true),

                (BVS_REL, StatusFlags::V, true),
                (BVS_REL, StatusFlags::empty(), false),
            ] {
                let addr_nobranch = ADDR_RESET_VECTOR + 2;
                let addr_branch = (ADDR_RESET_VECTOR + 2 as u16).wrapping_add(rel as u16);

                cpu.reset(&mut mem);
                cpu.sr.insert(srf);
                mem.write_u8(ADDR_RESET_VECTOR, opcode);
                mem.write_i8(None, rel);

                let cycles_orig = cpu.cycles;
                cpu.exec(&mut mem, 1);

                assert_eq!(cpu.pc, if jmp { addr_branch } else { addr_nobranch });
        
                let mut expected_cycles = Instruction::from_opcode(opcode).unwrap().cycles as u64;
                if jmp {
                    // jump occured: same page -> +1, page crossed -> +2
                    expected_cycles += if Cpu::is_page_crossed(ADDR_RESET_VECTOR + 2, rel) { 2 } else { 1 };
                }
                assert_eq!(cpu.cycles - cycles_orig, expected_cycles);
            }
        }
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

        for opcode in [INX, INY, DEX, DEY] {
            for value in [0xFE, 0xFF] {
                cpu.reset(&mut mem);
                mem.write_u8(ADDR_RESET_VECTOR, opcode);
                
                match opcode {
                    INX | DEX => {
                        cpu.x = value
                    },
                    INY | DEY => {
                        cpu.y = value
                    },
                    _ => panic!("Unhandled test case INC/DEC {:02X}", opcode)
                }
                
                cpu.exec(&mut mem, 1);

                let result = match opcode {
                    INX | DEX => {
                        cpu.x
                    },
                    INY | DEY => {
                        cpu.y
                    },
                    _ => panic!("Unhandled test case INC/DEC {:02X}", opcode)
                };
                assert_eq!(result, if matches!(opcode, INX | INY) { value.wrapping_add(1) } else { value.wrapping_sub(1) });
                if result == 0 { assert!(cpu.sr.contains(StatusFlags::Z),) }
                if result & 0b10000000 != 0 { assert!(cpu.sr.contains(StatusFlags::N)) }
            }
        }
    }

    #[test]
    fn ins_ldaldxldy() {
        let (mut cpu, mut mem) = setup();

        for opcode in [
                LDA_IMM, LDA_ZPG, LDA_ZPX, LDA_ABS, LDA_ABX, LDA_ABY, LDA_IDY, LDA_IDY,
                LDX_IMM, LDX_ZPG, LDX_ZPY, LDX_ABS, LDX_ABY,
                LDY_IMM, LDY_ZPG, LDY_ZPY, LDY_ABS, LDY_ABY,
            ] {
            for (value, sr_expect) in [
                (0x00, StatusFlags::RESERVED | StatusFlags::Z),
                (0x01, StatusFlags::RESERVED),
                (0xF0, StatusFlags::RESERVED | StatusFlags::N),
            ] {
                cpu.reset(&mut mem);

                let ins = Instruction::from_opcode(opcode).unwrap();
                let addr: u16 = 0x000A;
                cpu.x = 0;
                cpu.y = 0;
                if matches!(ins.addr_mode, AddressingMode::ZPG | AddressingMode::ZPX | AddressingMode::ZPY | AddressingMode::ABS | AddressingMode::ABX | AddressingMode::ABY) {
                    mem.write_u8(addr, value);
                } else if matches!(ins.addr_mode, AddressingMode::IDX | AddressingMode::IDY) {
                    mem.write_u16(addr, addr + 2);
                    mem.write_u8(addr + 2, value);
                }
                mem.write_u8(ADDR_RESET_VECTOR, opcode);
                if ins.addr_mode == AddressingMode::IMM {
                    mem.write_u8(None, value);
                } else if matches!(ins.addr_mode, AddressingMode::ZPG | AddressingMode::ZPX | AddressingMode::ZPY | AddressingMode::IDX | AddressingMode::IDY) {
                    mem.write_u8(None, (addr & 0xFF) as u8);
                } else {
                    mem.write_u16(None, addr);
                }

                cpu.exec(&mut mem, 1);

                let value_reg = match ins.mnemonic {
                    Mnemonic::LDA => cpu.ac,
                    Mnemonic::LDX => cpu.x,
                    Mnemonic::LDY => cpu.y,
                    _ => panic!("Unhandled test case LD* {:02X}", opcode),
                };
                assert_eq!(value_reg, value);
                assert_eq!(cpu.sr, sr_expect);
            }
        }
    }

    #[test]
    fn ins_stastxsty() {
        let (mut cpu, mut mem) = setup();

        for opcode in [
                STA_ZPG, STA_ZPX, STA_ABS, STA_ABX, STA_ABY, STA_IDY, STA_IDY,
                STX_ZPG, STX_ZPY, STX_ABS,
                STY_ZPG, STY_ZPX, STY_ABS,
            ] {
                cpu.reset(&mut mem);

                let ins = Instruction::from_opcode(opcode).unwrap();
                let addr: u16 = 0x000A;
                let value: u8 = 0xBB;

                match ins.mnemonic {
                    Mnemonic::STA => cpu.ac = value,
                    Mnemonic::STX => cpu.x = value,
                    Mnemonic::STY => cpu.y = value,
                    _ => panic!("Unhandled test case ST* {:02X}", opcode),
                };

                mem.write_u8(ADDR_RESET_VECTOR, opcode);
                
                if ins.addr_mode == AddressingMode::IMM {
                    mem.write_u8(None, value);
                } else if matches!(ins.addr_mode, AddressingMode::ZPG | AddressingMode::ZPX | AddressingMode::ZPY | AddressingMode::IDX | AddressingMode::IDY) {
                    mem.write_u8(None, (addr & 0xFF) as u8);

                    if matches!(ins.addr_mode, AddressingMode::IDX | AddressingMode::IDY) {
                        mem.write_u16(addr, addr + 2);  // write indirect address
                    }
                } else {
                    mem.write_u16(None, addr);
                }

                cpu.exec(&mut mem, 1);

                let value_read = match ins.addr_mode {
                    AddressingMode::ZPG | AddressingMode::ZPX | AddressingMode::ZPY | AddressingMode::ABS | AddressingMode::ABX | AddressingMode::ABY => mem.read_u8(addr),
                    AddressingMode::IDX | AddressingMode::IDY => mem.read_u8(addr + 2),
                    _ => panic!("Unhandled addressing mode {}", ins.addr_mode),
                };

                assert_eq!(value, value_read);
        }
    }

    #[test]
    fn ins_txx() {
        let (mut cpu, mut mem) = setup();

        for opcode in [TAX, TAY, TSX, TXA, TXS, TYA] {
            for (value, sr_expect) in [
                (0x00, StatusFlags::RESERVED | StatusFlags::Z),
                (0x01, StatusFlags::RESERVED),
                (0xF0, StatusFlags::RESERVED | StatusFlags::N),
            ] {
                cpu.reset(&mut mem);

                match opcode {
                    TAX | TAY => cpu.ac = value,
                    TXA | TXS => cpu.x = value,
                    TYA       => cpu.y = value,
                    TSX       => cpu.sp = value,
                    _ => panic!("Unhandled T** opcode {:02X}", opcode),
                };

                mem.write_u8(ADDR_RESET_VECTOR, opcode);

                cpu.exec(&mut mem, 1);

                let value_read = match opcode {
                    TXA | TYA => cpu.ac,
                    TAX | TSX => cpu.x,
                    TAY       => cpu.y,
                    TXS       => cpu.sp,
                    _ => panic!("Unhandled T** opcode {:02X}", opcode),
                };
                
                assert_eq!(value, value_read);
                if opcode != TXS {
                    assert_eq!(cpu.sr, sr_expect);
                }
            }
        }
    }
}
