#[derive(Debug)]
pub enum InstructionEnum {
    // ADC - Add Memory to Accumulator with Carry
    ADC_IMM,
    ADC_ZPG,
    ADC_ZPX,
    ADC_ABS,
    ADC_ABX,
    ADC_ABY,
    ADC_IDX,
    ADC_IDY,
}


#[derive(Debug)]
pub struct Instruction {
    pub ins_enum: InstructionEnum,
    pub opcode: u8,
    pub bytes: u8,
    pub cycles: u8,
}


impl Instruction {
    pub fn from_opcode(opcode: u8) -> Instruction {
        match opcode {
            0x69 => Instruction { ins_enum: InstructionEnum::ADC_IMM, opcode: 0x69, bytes: 2, cycles: 2 },
            _ => panic!("Unimplemented or invalid instruction {:2X}", opcode),
        }
    }
}
