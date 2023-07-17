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

// BCC - Branch on Carry Clear
pub const BCC_REL: u8 = 0x90;

// BCS - Branch on Carry Set
//pub const BCS_REL: u8 = 0xB0;

// BEQ - Branch on Result Zero
//pub const BEQ_REL: u8 = 0xF0;

// BIT - Test Bits in Memory with Accumulator
pub const BIT_ZPG: u8 = 0x24;
pub const BIT_ABS: u8 = 0x2C;

// JMP - Jump to New Location
pub const JMP_ABS: u8 = 0x4C;
pub const JMP_IND: u8 = 0x6C;



pub struct Instruction<'a> {
    pub opcode: u8,
    pub mnemonic: &'a str,
    pub bytes: u8,
    pub cycles: u8,
}

impl Instruction<'_> {
    pub fn from_opcode(opcode: u8) -> Result<Instruction<'static>, ()> {
        match opcode {
            ADC_IMM => Ok(Instruction { opcode, mnemonic: "ADC", bytes: 2, cycles: 2 }),
            ADC_ZPG => Ok(Instruction { opcode, mnemonic: "ADC", bytes: 2, cycles: 3 }),
            ADC_ZPX => Ok(Instruction { opcode, mnemonic: "ADC", bytes: 2, cycles: 4 }),
            ADC_ABS => Ok(Instruction { opcode, mnemonic: "ADC", bytes: 3, cycles: 4 }),
            ADC_ABX => Ok(Instruction { opcode, mnemonic: "ADC", bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ADC_ABY => Ok(Instruction { opcode, mnemonic: "ADC", bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ADC_IDX => Ok(Instruction { opcode, mnemonic: "ADC", bytes: 2, cycles: 6 }),
            ADC_IDY => Ok(Instruction { opcode, mnemonic: "ADC", bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            BCC_REL => Ok(Instruction { opcode, mnemonic: "BCC", bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),

            //BCS_REL => Ok(Instruction { opcode, mnemonic: "BCS", bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),

            //BEQ_REL => Ok(Instruction { opcode, mnemonic: "BEQ", bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),

            BIT_ZPG => Ok(Instruction { opcode, mnemonic: "BIT", bytes: 2, cycles: 3 }),
            BIT_ABS => Ok(Instruction { opcode, mnemonic: "BIT", bytes: 3, cycles: 4 }),

            JMP_ABS => Ok(Instruction { opcode, mnemonic: "JMP", bytes: 3, cycles: 3 }),
            JMP_IND => Ok(Instruction { opcode, mnemonic: "JMP", bytes: 3, cycles: 5 }),

            _ => Err(()),
        }
    }
}

impl fmt::Debug for Instruction<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("Opcode", &format!("0x{:02X}", self.opcode))
            .field("Mnemonic", &self.mnemonic)
            .field("Bytes", &self.bytes)
            .field("Cycles", &self.cycles)
            .finish()
    }
}
