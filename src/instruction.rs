use std::fmt;
use crate::cpu::AdressingMode;
use crate::cpu::AdressingMode::*;

// ADC - Add with Carry
pub const ADC_IMM: u8 = 0x69;
pub const ADC_ZPG: u8 = 0x65;
pub const ADC_ZPX: u8 = 0x75;
pub const ADC_ABS: u8 = 0x6D;
pub const ADC_ABX: u8 = 0x7D;
pub const ADC_ABY: u8 = 0x79;
pub const ADC_IDX: u8 = 0x61;
pub const ADC_IDY: u8 = 0x71;

// Branches
pub const BCC_REL: u8 = 0x90;   // BCC - Branch on Carry Clear
pub const BCS_REL: u8 = 0xB0;   // BCS - Branch on Carry Set
pub const BEQ_REL: u8 = 0xF0;   // BEQ - Branch on Result Zero
pub const BNE_REL: u8 = 0xD0;   // BNE - Branch on Result not Zero
pub const BPL_REL: u8 = 0x10;   // BPL - Branch on Result Plus
pub const BMI_REL: u8 = 0x30;   // BMI - Branch on Result Minus
pub const BVC_REL: u8 = 0x50;   // BVC - Branch on Overflow Clear
pub const BVS_REL: u8 = 0x70;   // BVS - Branch on Overflow Set

// BIT - Test Bits in Memory with Accumulator
pub const BIT_ZPG: u8 = 0x24;
pub const BIT_ABS: u8 = 0x2C;

// AND - AND Memory with Accumulator
pub const AND_IMM: u8 = 0x29;
pub const AND_ZPG: u8 = 0x25;
pub const AND_ZPX: u8 = 0x35;
pub const AND_ABS: u8 = 0x2D;
pub const AND_ABX: u8 = 0x3D;
pub const AND_ABY: u8 = 0x39;
pub const AND_IDX: u8 = 0x21;
pub const AND_IDY: u8 = 0x31;

// EOR - Exclusive-OR Memory with Accumulator
pub const EOR_IMM: u8 = 0x49;
pub const EOR_ZPG: u8 = 0x45;
pub const EOR_ZPX: u8 = 0x55;
pub const EOR_ABS: u8 = 0x4D;
pub const EOR_ABX: u8 = 0x5D;
pub const EOR_ABY: u8 = 0x59;
pub const EOR_IDX: u8 = 0x41;
pub const EOR_IDY: u8 = 0x51;

// ORA - OR Memory with Accumulator
pub const ORA_IMM: u8 = 0x09;
pub const ORA_ZPG: u8 = 0x05;
pub const ORA_ZPX: u8 = 0x15;
pub const ORA_ABS: u8 = 0x0D;
pub const ORA_ABX: u8 = 0x1D;
pub const ORA_ABY: u8 = 0x19;
pub const ORA_IDX: u8 = 0x01;
pub const ORA_IDY: u8 = 0x11;

// Flag Instructions
pub const CLC: u8 = 0x18;
pub const CLD: u8 = 0xD8;
pub const CLI: u8 = 0x58;
pub const CLV: u8 = 0xB8;
pub const SEC: u8 = 0x38;
pub const SED: u8 = 0xF8;
pub const SEI: u8 = 0x78;

// INC - Increment Memory by One
pub const INC_ZPG: u8 = 0xE6;
pub const INC_ZPX: u8 = 0xF6;
pub const INC_ABS: u8 = 0xEE;
pub const INC_ABX: u8 = 0xFE;

// Increment Index by One
pub const INX: u8 = 0xE8;   // INX - Increment Index X by One
pub const INY: u8 = 0xC8;   // INY - Increment Index Y by One

// DEC - Decrement Memory by One
pub const DEC_ZPG: u8 = 0xC6;
pub const DEC_ZPX: u8 = 0xD6;
pub const DEC_ABS: u8 = 0xCE;
pub const DEC_ABX: u8 = 0xDE;

// Decrement Index by One
pub const DEX: u8 = 0xCA;   // DEX - Decrement Index X by One
pub const DEY: u8 = 0x88;   // DEY - Decrement Index Y by One

// JMP - Jump to New Location
pub const JMP_ABS: u8 = 0x4C;
pub const JMP_IND: u8 = 0x6C;

// NOP - No Operation
pub const NOP: u8 = 0xEA;

pub struct Instruction {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub addr_mode: AdressingMode,
    pub bytes: u8,
    pub cycles: u8,
}

impl Instruction {
    pub fn from_opcode(opcode: u8) -> Result<Self, ()> {
        match opcode {
            ADC_IMM => Ok(Self { opcode, mnemonic: "ADC", addr_mode: Imm, bytes: 2, cycles: 2 }),
            ADC_ZPG => Ok(Self { opcode, mnemonic: "ADC", addr_mode: Zpg, bytes: 2, cycles: 3 }),
            ADC_ZPX => Ok(Self { opcode, mnemonic: "ADC", addr_mode: Zpx, bytes: 2, cycles: 4 }),
            ADC_ABS => Ok(Self { opcode, mnemonic: "ADC", addr_mode: Abs, bytes: 3, cycles: 4 }),
            ADC_ABX => Ok(Self { opcode, mnemonic: "ADC", addr_mode: Abx, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ADC_ABY => Ok(Self { opcode, mnemonic: "ADC", addr_mode: Aby, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ADC_IDX => Ok(Self { opcode, mnemonic: "ADC", addr_mode: Idx, bytes: 2, cycles: 6 }),
            ADC_IDY => Ok(Self { opcode, mnemonic: "ADC", addr_mode: Idy, bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            BCC_REL => Ok(Self { opcode, mnemonic: "BCC", addr_mode: Rel, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BCS_REL => Ok(Self { opcode, mnemonic: "BCS", addr_mode: Rel, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BEQ_REL => Ok(Self { opcode, mnemonic: "BEQ", addr_mode: Rel, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BNE_REL => Ok(Self { opcode, mnemonic: "BNE", addr_mode: Rel, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BPL_REL => Ok(Self { opcode, mnemonic: "BPL", addr_mode: Rel, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BMI_REL => Ok(Self { opcode, mnemonic: "BMI", addr_mode: Rel, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BVC_REL => Ok(Self { opcode, mnemonic: "BVC", addr_mode: Rel, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BVS_REL => Ok(Self { opcode, mnemonic: "BVS", addr_mode: Rel, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),

            BIT_ZPG => Ok(Self { opcode, mnemonic: "BIT", addr_mode: Zpg, bytes: 2, cycles: 3 }),
            BIT_ABS => Ok(Self { opcode, mnemonic: "BIT", addr_mode: Abs, bytes: 3, cycles: 4 }),

            AND_IMM => Ok(Self { opcode, mnemonic: "AND", addr_mode: Imm, bytes: 2, cycles: 2 }),
            AND_ZPG => Ok(Self { opcode, mnemonic: "AND", addr_mode: Zpg, bytes: 2, cycles: 3 }),
            AND_ZPX => Ok(Self { opcode, mnemonic: "AND", addr_mode: Zpx, bytes: 2, cycles: 4 }),
            AND_ABS => Ok(Self { opcode, mnemonic: "AND", addr_mode: Abs, bytes: 3, cycles: 4 }),
            AND_ABX => Ok(Self { opcode, mnemonic: "AND", addr_mode: Abx, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            AND_ABY => Ok(Self { opcode, mnemonic: "AND", addr_mode: Aby, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            AND_IDX => Ok(Self { opcode, mnemonic: "AND", addr_mode: Idx, bytes: 2, cycles: 6 }),
            AND_IDY => Ok(Self { opcode, mnemonic: "AND", addr_mode: Idy, bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            EOR_IMM => Ok(Self { opcode, mnemonic: "EOR", addr_mode: Imm, bytes: 2, cycles: 2 }),
            EOR_ZPG => Ok(Self { opcode, mnemonic: "EOR", addr_mode: Zpg, bytes: 2, cycles: 3 }),
            EOR_ZPX => Ok(Self { opcode, mnemonic: "EOR", addr_mode: Zpx, bytes: 2, cycles: 4 }),
            EOR_ABS => Ok(Self { opcode, mnemonic: "EOR", addr_mode: Abs, bytes: 3, cycles: 4 }),
            EOR_ABX => Ok(Self { opcode, mnemonic: "EOR", addr_mode: Abx, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            EOR_ABY => Ok(Self { opcode, mnemonic: "EOR", addr_mode: Aby, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            EOR_IDX => Ok(Self { opcode, mnemonic: "EOR", addr_mode: Idx, bytes: 2, cycles: 6 }),
            EOR_IDY => Ok(Self { opcode, mnemonic: "EOR", addr_mode: Idy, bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            ORA_IMM => Ok(Self { opcode, mnemonic: "ORA", addr_mode: Imm, bytes: 2, cycles: 2 }),
            ORA_ZPG => Ok(Self { opcode, mnemonic: "ORA", addr_mode: Zpg, bytes: 2, cycles: 3 }),
            ORA_ZPX => Ok(Self { opcode, mnemonic: "ORA", addr_mode: Zpx, bytes: 2, cycles: 4 }),
            ORA_ABS => Ok(Self { opcode, mnemonic: "ORA", addr_mode: Abs, bytes: 3, cycles: 4 }),
            ORA_ABX => Ok(Self { opcode, mnemonic: "ORA", addr_mode: Abx, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ORA_ABY => Ok(Self { opcode, mnemonic: "ORA", addr_mode: Aby, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ORA_IDX => Ok(Self { opcode, mnemonic: "ORA", addr_mode: Idx, bytes: 2, cycles: 6 }),
            ORA_IDY => Ok(Self { opcode, mnemonic: "ORA", addr_mode: Idy, bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            CLC     => Ok(Self { opcode, mnemonic: "CLC", addr_mode: Imp, bytes: 1, cycles: 2 }),
            CLD     => Ok(Self { opcode, mnemonic: "CLS", addr_mode: Imp, bytes: 1, cycles: 2 }),
            CLI     => Ok(Self { opcode, mnemonic: "CLI", addr_mode: Imp, bytes: 1, cycles: 2 }),
            CLV     => Ok(Self { opcode, mnemonic: "CLV", addr_mode: Imp, bytes: 1, cycles: 2 }),
            SEC     => Ok(Self { opcode, mnemonic: "SEC", addr_mode: Imp, bytes: 1, cycles: 2 }),
            SED     => Ok(Self { opcode, mnemonic: "SED", addr_mode: Imp, bytes: 1, cycles: 2 }),
            SEI     => Ok(Self { opcode, mnemonic: "SEI", addr_mode: Imp, bytes: 1, cycles: 2 }),

            INC_ZPG => Ok(Self { opcode, mnemonic: "INC", addr_mode: Zpg, bytes: 2, cycles: 5 }),
            INC_ZPX => Ok(Self { opcode, mnemonic: "INC", addr_mode: Zpx, bytes: 2, cycles: 6 }),
            INC_ABS => Ok(Self { opcode, mnemonic: "INC", addr_mode: Abs, bytes: 3, cycles: 6 }),
            INC_ABX => Ok(Self { opcode, mnemonic: "INC", addr_mode: Abx, bytes: 3, cycles: 7 }),

            INX     => Ok(Self { opcode, mnemonic: "INX", addr_mode: Imp, bytes: 1, cycles: 2 }),
            INY     => Ok(Self { opcode, mnemonic: "INY", addr_mode: Imp, bytes: 1, cycles: 2 }),

            DEC_ZPG => Ok(Self { opcode, mnemonic: "DEC", addr_mode: Zpg, bytes: 2, cycles: 5 }),
            DEC_ZPX => Ok(Self { opcode, mnemonic: "DEC", addr_mode: Zpx, bytes: 2, cycles: 6 }),
            DEC_ABS => Ok(Self { opcode, mnemonic: "DEC", addr_mode: Abs, bytes: 3, cycles: 6 }),
            DEC_ABX => Ok(Self { opcode, mnemonic: "DEC", addr_mode: Abx, bytes: 3, cycles: 7 }),

            DEX     => Ok(Self { opcode, mnemonic: "DEX", addr_mode: Imp, bytes: 1, cycles: 2 }),
            DEY     => Ok(Self { opcode, mnemonic: "DEY", addr_mode: Imp, bytes: 1, cycles: 2 }),

            JMP_ABS => Ok(Self { opcode, mnemonic: "JMP", addr_mode: Abs, bytes: 3, cycles: 3 }),
            JMP_IND => Ok(Self { opcode, mnemonic: "JMP", addr_mode: Ind, bytes: 3, cycles: 5 }),

            NOP     => Ok(Self { opcode, mnemonic: "NOP", addr_mode: Imp, bytes: 1, cycles: 2 }),

            _ => Err(()),
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("Opcode", &format!("0x{:02X}", self.opcode))
            .field("Mnemonic", &self.mnemonic)
            .field("AddrMode", &self.addr_mode)
            .field("Bytes", &self.bytes)
            .field("Cycles", &self.cycles)
            .finish()
    }
}
