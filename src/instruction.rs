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

pub struct Instruction<'a> {
    pub opcode: u8,
    pub mnemonic: &'a str,
    pub bytes: u8,
    pub cycles: u8,
}

impl Instruction<'_> {
    pub fn from_opcode(opcode: u8) -> Instruction<'static> {

        match opcode {
            // ADC
            ADC_IMM => Instruction { opcode: ADC_IMM, mnemonic: "ADC", bytes: 2, cycles: 2 },
            ADC_ZPG => Instruction { opcode: ADC_ZPG, mnemonic: "ADC", bytes: 2, cycles: 3 },
            ADC_ZPX => Instruction { opcode: ADC_ZPX, mnemonic: "ADC", bytes: 2, cycles: 4 },
            ADC_ABS => Instruction { opcode: ADC_ABS, mnemonic: "ADC", bytes: 3, cycles: 4 },
            ADC_ABX => Instruction { opcode: ADC_ABX, mnemonic: "ADC", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            ADC_ABY => Instruction { opcode: ADC_ABY, mnemonic: "ADC", bytes: 3, cycles: 4 /* +1 if page crossed */ },
            ADC_IDX => Instruction { opcode: ADC_IDX, mnemonic: "ADC", bytes: 2, cycles: 6 },
            ADC_IDY => Instruction { opcode: ADC_IDY, mnemonic: "ADC", bytes: 2, cycles: 5 /* +1 if page crossed */ },

            _ => panic!("Unimplemented or invalid opcode {:2X}", opcode),
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
