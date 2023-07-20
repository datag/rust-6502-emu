use std::fmt;

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

// INC - Increment Memory by One
pub const INC_ZPG: u8 = 0xE6;
pub const INC_ZPX: u8 = 0xF6;
pub const INC_ABS: u8 = 0xEE;
pub const INC_ABX: u8 = 0xFE;

// DEC - Decrement Memory by One
pub const DEC_ZPG: u8 = 0xC6;
pub const DEC_ZPX: u8 = 0xD6;
pub const DEC_ABS: u8 = 0xCE;
pub const DEC_ABX: u8 = 0xDE;

// JMP - Jump to New Location
pub const JMP_ABS: u8 = 0x4C;
pub const JMP_IND: u8 = 0x6C;

// NOP - No Operation
pub const NOP: u8 = 0xEA;

pub struct Instruction {
    pub opcode: u8,
    pub mnemonic: &'static str,
    pub bytes: u8,
    pub cycles: u8,
}

impl Instruction {
    pub fn from_opcode(opcode: u8) -> Result<Self, ()> {
        match opcode {
            ADC_IMM => Ok(Self { opcode, mnemonic: "ADC", bytes: 2, cycles: 2 }),
            ADC_ZPG => Ok(Self { opcode, mnemonic: "ADC", bytes: 2, cycles: 3 }),
            ADC_ZPX => Ok(Self { opcode, mnemonic: "ADC", bytes: 2, cycles: 4 }),
            ADC_ABS => Ok(Self { opcode, mnemonic: "ADC", bytes: 3, cycles: 4 }),
            ADC_ABX => Ok(Self { opcode, mnemonic: "ADC", bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ADC_ABY => Ok(Self { opcode, mnemonic: "ADC", bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ADC_IDX => Ok(Self { opcode, mnemonic: "ADC", bytes: 2, cycles: 6 }),
            ADC_IDY => Ok(Self { opcode, mnemonic: "ADC", bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            BCC_REL => Ok(Self { opcode, mnemonic: "BCC", bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BCS_REL => Ok(Self { opcode, mnemonic: "BCS", bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BEQ_REL => Ok(Self { opcode, mnemonic: "BEQ", bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BNE_REL => Ok(Self { opcode, mnemonic: "BNE", bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BPL_REL => Ok(Self { opcode, mnemonic: "BPL", bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BMI_REL => Ok(Self { opcode, mnemonic: "BMI", bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BVC_REL => Ok(Self { opcode, mnemonic: "BVC", bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BVS_REL => Ok(Self { opcode, mnemonic: "BVS", bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),

            BIT_ZPG => Ok(Self { opcode, mnemonic: "BIT", bytes: 2, cycles: 3 }),
            BIT_ABS => Ok(Self { opcode, mnemonic: "BIT", bytes: 3, cycles: 4 }),

            INC_ZPG => Ok(Self { opcode, mnemonic: "INC", bytes: 2, cycles: 5 }),
            INC_ZPX => Ok(Self { opcode, mnemonic: "INC", bytes: 2, cycles: 6 }),
            INC_ABS => Ok(Self { opcode, mnemonic: "INC", bytes: 3, cycles: 6 }),
            INC_ABX => Ok(Self { opcode, mnemonic: "INC", bytes: 3, cycles: 7 }),

            DEC_ZPG => Ok(Self { opcode, mnemonic: "DEC", bytes: 2, cycles: 5 }),
            DEC_ZPX => Ok(Self { opcode, mnemonic: "DEC", bytes: 2, cycles: 6 }),
            DEC_ABS => Ok(Self { opcode, mnemonic: "DEC", bytes: 3, cycles: 6 }),
            DEC_ABX => Ok(Self { opcode, mnemonic: "DEC", bytes: 3, cycles: 7 }),

            JMP_ABS => Ok(Self { opcode, mnemonic: "JMP", bytes: 3, cycles: 3 }),
            JMP_IND => Ok(Self { opcode, mnemonic: "JMP", bytes: 3, cycles: 5 }),

            NOP =>     Ok(Self { opcode, mnemonic: "NOP", bytes: 1, cycles: 2 }),

            _ => Err(()),
        }
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("Opcode", &format!("0x{:02X}", self.opcode))
            .field("Mnemonic", &self.mnemonic)
            .field("Bytes", &self.bytes)
            .field("Cycles", &self.cycles)
            .finish()
    }
}
