use std::fmt;

pub const ADC_IMM: u8 = 0x69;

pub struct Instruction {
    pub opcode: u8,
    pub bytes: u8,
    pub cycles: u8,
}


impl Instruction {
    pub fn from_opcode(opcode: u8) -> Instruction {
        let mut ins: Instruction;
        
        match opcode {
            ADC_IMM => ins = Instruction { opcode: 0x69, bytes: 2, cycles: 2 },
            _ => panic!("Unimplemented or invalid instruction {:2X}", opcode),
        }

        // for all
        ins.opcode = opcode;
        return ins;
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("Opcode", &format!("{:02X}", self.opcode))
            .field("Bytes", &self.bytes)
            .field("Cycles", &self.cycles)
            .finish()
    }
}
