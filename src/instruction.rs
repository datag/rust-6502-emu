use std::fmt;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

use AddressingMode::*;
use Opcode::*;

#[allow(non_camel_case_types)]
#[derive(Debug, FromPrimitive, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum Opcode {
    // ADC - Add with Carry
    ADC_IMM = 0x69,
    ADC_ZPG = 0x65,
    ADC_ZPX = 0x75,
    ADC_ABS = 0x6D,
    ADC_ABX = 0x7D,
    ADC_ABY = 0x79,
    ADC_IDX = 0x61,
    ADC_IDY = 0x71,

    // SBC - Subtract Memory from Accumulator with Borrow
    SBC_IMM = 0xE9,
    SBC_ZPG = 0xE5,
    SBC_ZPX = 0xF5,
    SBC_ABS = 0xED,
    SBC_ABX = 0xFD,
    SBC_ABY = 0xF9,
    SBC_IDX = 0xE1,
    SBC_IDY = 0xF1,

    // CMP - Compare Memory with Accumulator
    CMP_IMM = 0xC9,
    CMP_ZPG = 0xC5,
    CMP_ZPX = 0xD5,
    CMP_ABS = 0xCD,
    CMP_ABX = 0xDD,
    CMP_ABY = 0xD9,
    CMP_IDX = 0xC1,
    CMP_IDY = 0xD1,

    // CPX Compare Memory and Index X
    CPX_IMM = 0xE0,
    CPX_ZPG = 0xE4,
    CPX_ABS = 0xEC,

    // CPY Compare Memory and Index Y
    CPY_IMM = 0xC0,
    CPY_ZPG = 0xC4,
    CPY_ABS = 0xCC,

    // Branches
    BCC_REL = 0x90,   // BCC - Branch on Carry Clear
    BCS_REL = 0xB0,   // BCS - Branch on Carry Set
    BEQ_REL = 0xF0,   // BEQ - Branch on Result Zero
    BNE_REL = 0xD0,   // BNE - Branch on Result not Zero
    BPL_REL = 0x10,   // BPL - Branch on Result Plus
    BMI_REL = 0x30,   // BMI - Branch on Result Minus
    BVC_REL = 0x50,   // BVC - Branch on Overflow Clear
    BVS_REL = 0x70,   // BVS - Branch on Overflow Set

    // BIT - Test Bits in Memory with Accumulator
    BIT_ZPG = 0x24,
    BIT_ABS = 0x2C,

    // ASL - Shift Left One Bit (Memory or Accumulator)
    ASL_ACC = 0x0A,
    ASL_ZPG = 0x06,
    ASL_ZPX = 0x16,
    ASL_ABS = 0x0E,
    ASL_ABX = 0x1E,

    // LSR -  Shift One Bit Right (Memory or Accumulator)
    LSR_ACC = 0x4A,
    LSR_ZPG = 0x46,
    LSR_ZPX = 0x56,
    LSR_ABS = 0x4E,
    LSR_ABX = 0x5E,

    // ROL - Rotate One Bit Left (Memory or Accumulator)
    ROL_ACC = 0x2A,
    ROL_ZPG = 0x26,
    ROL_ZPX = 0x36,
    ROL_ABS = 0x2E,
    ROL_ABX = 0x3E,

    // ROR - Rotate One Bit Right (Memory or Accumulator)
    ROR_ACC = 0x6A,
    ROR_ZPG = 0x66,
    ROR_ZPX = 0x76,
    ROR_ABS = 0x6E,
    ROR_ABX = 0x7E,

    // AND - AND Memory with Accumulator
    AND_IMM = 0x29,
    AND_ZPG = 0x25,
    AND_ZPX = 0x35,
    AND_ABS = 0x2D,
    AND_ABX = 0x3D,
    AND_ABY = 0x39,
    AND_IDX = 0x21,
    AND_IDY = 0x31,

    // EOR - Exclusive-OR Memory with Accumulator
    EOR_IMM = 0x49,
    EOR_ZPG = 0x45,
    EOR_ZPX = 0x55,
    EOR_ABS = 0x4D,
    EOR_ABX = 0x5D,
    EOR_ABY = 0x59,
    EOR_IDX = 0x41,
    EOR_IDY = 0x51,

    // ORA - OR Memory with Accumulator
    ORA_IMM = 0x09,
    ORA_ZPG = 0x05,
    ORA_ZPX = 0x15,
    ORA_ABS = 0x0D,
    ORA_ABX = 0x1D,
    ORA_ABY = 0x19,
    ORA_IDX = 0x01,
    ORA_IDY = 0x11,

    // Flag Instructions
    CLC = 0x18,
    CLD = 0xD8,
    CLI = 0x58,
    CLV = 0xB8,
    SEC = 0x38,
    SED = 0xF8,
    SEI = 0x78,

    // INC - Increment Memory by One
    INC_ZPG = 0xE6,
    INC_ZPX = 0xF6,
    INC_ABS = 0xEE,
    INC_ABX = 0xFE,

    // Increment Index by One
    INX = 0xE8,   // INX - Increment Index X by One
    INY = 0xC8,   // INY - Increment Index Y by One

    // DEC - Decrement Memory by One
    DEC_ZPG = 0xC6,
    DEC_ZPX = 0xD6,
    DEC_ABS = 0xCE,
    DEC_ABX = 0xDE,

    // Decrement Index by One
    DEX = 0xCA,   // DEX - Decrement Index X by One
    DEY = 0x88,   // DEY - Decrement Index Y by One

    // JMP - Jump to New Location
    JMP_ABS = 0x4C,
    JMP_IND = 0x6C,

    JSR_ABS = 0x20,
    RTS = 0x60,

    // LDA - Load Accumulator with Memory
    LDA_IMM = 0xA9,
    LDA_ZPG = 0xA5,
    LDA_ZPX = 0xB5,
    LDA_ABS = 0xAD,
    LDA_ABX = 0xBD,
    LDA_ABY = 0xB9,
    LDA_IDX = 0xA1,
    LDA_IDY = 0xB1,

    // LDX - Load Index X with Memory
    LDX_IMM = 0xA2,
    LDX_ZPG = 0xA6,
    LDX_ZPY = 0xB6,
    LDX_ABS = 0xAE,
    LDX_ABY = 0xBE,

    // LDY - Load Index Y with Memory
    LDY_IMM = 0xA0,
    LDY_ZPG = 0xA4,
    LDY_ZPY = 0xB4,
    LDY_ABS = 0xAC,
    LDY_ABY = 0xBC,

    // STA - Store Accumulator in Memory
    STA_ZPG = 0x85,
    STA_ZPX = 0x95,
    STA_ABS = 0x8D,
    STA_ABX = 0x9D,
    STA_ABY = 0x99,
    STA_IDX = 0x81,
    STA_IDY = 0x91,

    // STX - Store Index X in Memory
    STX_ZPG = 0x86,
    STX_ZPY = 0x96,
    STX_ABS = 0x8E,

    // STY - Store Index Y in Memory
    STY_ZPG = 0x84,
    STY_ZPX = 0x94,
    STY_ABS = 0x8C,

    // Interregister transfer
    TAX = 0xAA,
    TAY = 0xA8,
    TSX = 0xBA,
    TXA = 0x8A,
    TXS = 0x9A,
    TYA = 0x98,

    // Stack Instructions
    PHA = 0x48,
    PHP = 0x08,
    PLA = 0x68,
    PLP = 0x28,

    // NOP - No Operation
    NOP = 0xEA,

    // BRK - Force Break
    BRK = 0x00,

    // RTI - Return from Interrupt
    RTI = 0x40,
}

impl fmt::UpperHex for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = *self as u8;

        fmt::UpperHex::fmt(&val, f) // delegate to u8's implementation
    }
}

impl From<u8> for Opcode {
    fn from(byte: u8) -> Self {
        if let Some(opcode) = Opcode::from_u8(byte) {
            opcode
        } else {
            panic!("Could not convert {:02X} into an Opcode", byte)
        }
    }
}

impl From<Opcode> for u8 {
    fn from(item: Opcode) -> Self {
        item as u8
    }
}

pub struct Instruction {
    pub opcode: Opcode,
    pub mnemonic: Mnemonic,
    pub addr_mode: AddressingMode,
    pub cycles: u8,
}

impl Instruction {
    pub fn from_opcode(opcode: Opcode) -> Result<Self, String> {
        match opcode {
            ADC_IMM => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: IMM, cycles: 2 }),
            ADC_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: ZPG, cycles: 3 }),
            ADC_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: ZPX, cycles: 4 }),
            ADC_ABS => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: ABS, cycles: 4 }),
            ADC_ABX => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: ABX, cycles: 4 /* +1 if page crossed */ }),
            ADC_ABY => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: ABY, cycles: 4 /* +1 if page crossed */ }),
            ADC_IDX => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: IDX, cycles: 6 }),
            ADC_IDY => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: IDY, cycles: 5 /* +1 if page crossed */ }),

            SBC_IMM => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: IMM, cycles: 2 }),
            SBC_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: ZPG, cycles: 3 }),
            SBC_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: ZPX, cycles: 4 }),
            SBC_ABS => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: ABS, cycles: 4 }),
            SBC_ABX => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: ABX, cycles: 4 /* +1 if page crossed */ }),
            SBC_ABY => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: ABY, cycles: 4 /* +1 if page crossed */ }),
            SBC_IDX => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: IDX, cycles: 6 }),
            SBC_IDY => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: IDY, cycles: 5 /* +1 if page crossed */ }),

            CMP_IMM => Ok(Self { opcode, mnemonic: Mnemonic::CMP, addr_mode: IMM, cycles: 2 }),
            CMP_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::CMP, addr_mode: ZPG, cycles: 3 }),
            CMP_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::CMP, addr_mode: ZPX, cycles: 4 }),
            CMP_ABS => Ok(Self { opcode, mnemonic: Mnemonic::CMP, addr_mode: ABS, cycles: 4 }),
            CMP_ABX => Ok(Self { opcode, mnemonic: Mnemonic::CMP, addr_mode: ABX, cycles: 4 /* +1 if page crossed */ }),
            CMP_ABY => Ok(Self { opcode, mnemonic: Mnemonic::CMP, addr_mode: ABY, cycles: 4 /* +1 if page crossed */ }),
            CMP_IDX => Ok(Self { opcode, mnemonic: Mnemonic::CMP, addr_mode: IDX, cycles: 6 }),
            CMP_IDY => Ok(Self { opcode, mnemonic: Mnemonic::CMP, addr_mode: IDY, cycles: 5 /* +1 if page crossed */ }),

            CPX_IMM => Ok(Self { opcode, mnemonic: Mnemonic::CPX, addr_mode: IMM, cycles: 2 }),
            CPX_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::CPX, addr_mode: ZPG, cycles: 3 }),
            CPX_ABS => Ok(Self { opcode, mnemonic: Mnemonic::CPX, addr_mode: ABS, cycles: 4 }),

            CPY_IMM => Ok(Self { opcode, mnemonic: Mnemonic::CPY, addr_mode: IMM, cycles: 2 }),
            CPY_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::CPY, addr_mode: ZPG, cycles: 3 }),
            CPY_ABS => Ok(Self { opcode, mnemonic: Mnemonic::CPY, addr_mode: ABS, cycles: 4 }),

            BCC_REL => Ok(Self { opcode, mnemonic: Mnemonic::BCC, addr_mode: REL, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BCS_REL => Ok(Self { opcode, mnemonic: Mnemonic::BCS, addr_mode: REL, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BEQ_REL => Ok(Self { opcode, mnemonic: Mnemonic::BEQ, addr_mode: REL, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BNE_REL => Ok(Self { opcode, mnemonic: Mnemonic::BNE, addr_mode: REL, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BPL_REL => Ok(Self { opcode, mnemonic: Mnemonic::BPL, addr_mode: REL, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BMI_REL => Ok(Self { opcode, mnemonic: Mnemonic::BMI, addr_mode: REL, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BVC_REL => Ok(Self { opcode, mnemonic: Mnemonic::BVC, addr_mode: REL, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BVS_REL => Ok(Self { opcode, mnemonic: Mnemonic::BVS, addr_mode: REL, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),

            BIT_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::BIT, addr_mode: ZPG, cycles: 3 }),
            BIT_ABS => Ok(Self { opcode, mnemonic: Mnemonic::BIT, addr_mode: ABS, cycles: 4 }),

            ASL_ACC => Ok(Self { opcode, mnemonic: Mnemonic::ASL, addr_mode: ACC, cycles: 2 }),
            ASL_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::ASL, addr_mode: ZPG, cycles: 5 }),
            ASL_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::ASL, addr_mode: ZPX, cycles: 6 }),
            ASL_ABS => Ok(Self { opcode, mnemonic: Mnemonic::ASL, addr_mode: ABS, cycles: 6 }),
            ASL_ABX => Ok(Self { opcode, mnemonic: Mnemonic::ASL, addr_mode: ABX, cycles: 7 }),

            LSR_ACC => Ok(Self { opcode, mnemonic: Mnemonic::LSR, addr_mode: ACC, cycles: 2 }),
            LSR_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::LSR, addr_mode: ZPG, cycles: 5 }),
            LSR_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::LSR, addr_mode: ZPX, cycles: 6 }),
            LSR_ABS => Ok(Self { opcode, mnemonic: Mnemonic::LSR, addr_mode: ABS, cycles: 6 }),
            LSR_ABX => Ok(Self { opcode, mnemonic: Mnemonic::LSR, addr_mode: ABX, cycles: 7 }),

            ROL_ACC => Ok(Self { opcode, mnemonic: Mnemonic::ROL, addr_mode: ACC, cycles: 2 }),
            ROL_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::ROL, addr_mode: ZPG, cycles: 5 }),
            ROL_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::ROL, addr_mode: ZPX, cycles: 6 }),
            ROL_ABS => Ok(Self { opcode, mnemonic: Mnemonic::ROL, addr_mode: ABS, cycles: 6 }),
            ROL_ABX => Ok(Self { opcode, mnemonic: Mnemonic::ROL, addr_mode: ABX, cycles: 7 }),

            ROR_ACC => Ok(Self { opcode, mnemonic: Mnemonic::ROR, addr_mode: ACC, cycles: 2 }),
            ROR_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::ROR, addr_mode: ZPG, cycles: 5 }),
            ROR_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::ROR, addr_mode: ZPX, cycles: 6 }),
            ROR_ABS => Ok(Self { opcode, mnemonic: Mnemonic::ROR, addr_mode: ABS, cycles: 6 }),
            ROR_ABX => Ok(Self { opcode, mnemonic: Mnemonic::ROR, addr_mode: ABX, cycles: 7 }),

            AND_IMM => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: IMM, cycles: 2 }),
            AND_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: ZPG, cycles: 3 }),
            AND_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: ZPX, cycles: 4 }),
            AND_ABS => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: ABS, cycles: 4 }),
            AND_ABX => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: ABX, cycles: 4 /* +1 if page crossed */ }),
            AND_ABY => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: ABY, cycles: 4 /* +1 if page crossed */ }),
            AND_IDX => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: IDX, cycles: 6 }),
            AND_IDY => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: IDY, cycles: 5 /* +1 if page crossed */ }),

            EOR_IMM => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: IMM, cycles: 2 }),
            EOR_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: ZPG, cycles: 3 }),
            EOR_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: ZPX, cycles: 4 }),
            EOR_ABS => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: ABS, cycles: 4 }),
            EOR_ABX => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: ABX, cycles: 4 /* +1 if page crossed */ }),
            EOR_ABY => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: ABY, cycles: 4 /* +1 if page crossed */ }),
            EOR_IDX => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: IDX, cycles: 6 }),
            EOR_IDY => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: IDY, cycles: 5 /* +1 if page crossed */ }),

            ORA_IMM => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: IMM, cycles: 2 }),
            ORA_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: ZPG, cycles: 3 }),
            ORA_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: ZPX, cycles: 4 }),
            ORA_ABS => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: ABS, cycles: 4 }),
            ORA_ABX => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: ABX, cycles: 4 /* +1 if page crossed */ }),
            ORA_ABY => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: ABY, cycles: 4 /* +1 if page crossed */ }),
            ORA_IDX => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: IDX, cycles: 6 }),
            ORA_IDY => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: IDY, cycles: 5 /* +1 if page crossed */ }),

            CLC     => Ok(Self { opcode, mnemonic: Mnemonic::CLC, addr_mode: IMP, cycles: 2 }),
            CLD     => Ok(Self { opcode, mnemonic: Mnemonic::CLD, addr_mode: IMP, cycles: 2 }),
            CLI     => Ok(Self { opcode, mnemonic: Mnemonic::CLI, addr_mode: IMP, cycles: 2 }),
            CLV     => Ok(Self { opcode, mnemonic: Mnemonic::CLV, addr_mode: IMP, cycles: 2 }),
            SEC     => Ok(Self { opcode, mnemonic: Mnemonic::SEC, addr_mode: IMP, cycles: 2 }),
            SED     => Ok(Self { opcode, mnemonic: Mnemonic::SED, addr_mode: IMP, cycles: 2 }),
            SEI     => Ok(Self { opcode, mnemonic: Mnemonic::SEI, addr_mode: IMP, cycles: 2 }),

            INC_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::INC, addr_mode: ZPG, cycles: 5 }),
            INC_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::INC, addr_mode: ZPX, cycles: 6 }),
            INC_ABS => Ok(Self { opcode, mnemonic: Mnemonic::INC, addr_mode: ABS, cycles: 6 }),
            INC_ABX => Ok(Self { opcode, mnemonic: Mnemonic::INC, addr_mode: ABX, cycles: 7 }),

            INX     => Ok(Self { opcode, mnemonic: Mnemonic::INX, addr_mode: IMP, cycles: 2 }),
            INY     => Ok(Self { opcode, mnemonic: Mnemonic::INY, addr_mode: IMP, cycles: 2 }),

            DEC_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::DEC, addr_mode: ZPG, cycles: 5 }),
            DEC_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::DEC, addr_mode: ZPX, cycles: 6 }),
            DEC_ABS => Ok(Self { opcode, mnemonic: Mnemonic::DEC, addr_mode: ABS, cycles: 6 }),
            DEC_ABX => Ok(Self { opcode, mnemonic: Mnemonic::DEC, addr_mode: ABX, cycles: 7 }),

            DEX     => Ok(Self { opcode, mnemonic: Mnemonic::DEX, addr_mode: IMP, cycles: 2 }),
            DEY     => Ok(Self { opcode, mnemonic: Mnemonic::DEY, addr_mode: IMP, cycles: 2 }),

            JMP_ABS => Ok(Self { opcode, mnemonic: Mnemonic::JMP, addr_mode: ABS, cycles: 3 }),
            JMP_IND => Ok(Self { opcode, mnemonic: Mnemonic::JMP, addr_mode: IND, cycles: 5 }),

            JSR_ABS => Ok(Self { opcode, mnemonic: Mnemonic::JSR, addr_mode: ABS, cycles: 6 }),
            RTS     => Ok(Self { opcode, mnemonic: Mnemonic::RTS, addr_mode: IMP, cycles: 6 }),

            LDA_IMM => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: IMM, cycles: 2 }),
            LDA_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: ZPG, cycles: 3 }),
            LDA_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: ZPX, cycles: 4 }),
            LDA_ABS => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: ABS, cycles: 4 }),
            LDA_ABX => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: ABX, cycles: 4 /* +1 if page crossed */ }),
            LDA_ABY => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: ABY, cycles: 4 /* +1 if page crossed */ }),
            LDA_IDX => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: IDX, cycles: 6 }),
            LDA_IDY => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: IDY, cycles: 5 /* +1 if page crossed */ }),

            LDX_IMM => Ok(Self { opcode, mnemonic: Mnemonic::LDX, addr_mode: IMM, cycles: 2 }),
            LDX_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::LDX, addr_mode: ZPG, cycles: 3 }),
            LDX_ZPY => Ok(Self { opcode, mnemonic: Mnemonic::LDX, addr_mode: ZPY, cycles: 4 }),
            LDX_ABS => Ok(Self { opcode, mnemonic: Mnemonic::LDX, addr_mode: ABS, cycles: 4 }),
            LDX_ABY => Ok(Self { opcode, mnemonic: Mnemonic::LDX, addr_mode: ABY, cycles: 4 /* +1 if page crossed */  }),

            LDY_IMM => Ok(Self { opcode, mnemonic: Mnemonic::LDY, addr_mode: IMM, cycles: 2 }),
            LDY_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::LDY, addr_mode: ZPG, cycles: 3 }),
            LDY_ZPY => Ok(Self { opcode, mnemonic: Mnemonic::LDY, addr_mode: ZPY, cycles: 4 }),
            LDY_ABS => Ok(Self { opcode, mnemonic: Mnemonic::LDY, addr_mode: ABS, cycles: 4 }),
            LDY_ABY => Ok(Self { opcode, mnemonic: Mnemonic::LDY, addr_mode: ABY, cycles: 4 /* +1 if page crossed */  }),

            STA_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: ZPG, cycles: 3 }),
            STA_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: ZPX, cycles: 4 }),
            STA_ABS => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: ABS, cycles: 4 }),
            STA_ABX => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: ABX, cycles: 5 }),
            STA_ABY => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: ABY, cycles: 5 }),
            STA_IDX => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: IDX, cycles: 6 }),
            STA_IDY => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: IDY, cycles: 6 }),

            STX_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::STX, addr_mode: ZPG, cycles: 3 }),
            STX_ZPY => Ok(Self { opcode, mnemonic: Mnemonic::STX, addr_mode: ZPY, cycles: 4 }),
            STX_ABS => Ok(Self { opcode, mnemonic: Mnemonic::STX, addr_mode: ABS, cycles: 4 }),

            STY_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::STY, addr_mode: ZPG, cycles: 3 }),
            STY_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::STY, addr_mode: ZPX, cycles: 4 }),
            STY_ABS => Ok(Self { opcode, mnemonic: Mnemonic::STY, addr_mode: ABS, cycles: 4 }),

            TAX     => Ok(Self { opcode, mnemonic: Mnemonic::TAX, addr_mode: IMP, cycles: 2 }),
            TAY     => Ok(Self { opcode, mnemonic: Mnemonic::TAY, addr_mode: IMP, cycles: 2 }),
            TSX     => Ok(Self { opcode, mnemonic: Mnemonic::TSX, addr_mode: IMP, cycles: 2 }),
            TXA     => Ok(Self { opcode, mnemonic: Mnemonic::TXA, addr_mode: IMP, cycles: 2 }),
            TXS     => Ok(Self { opcode, mnemonic: Mnemonic::TXS, addr_mode: IMP, cycles: 2 }),
            TYA     => Ok(Self { opcode, mnemonic: Mnemonic::TYA, addr_mode: IMP, cycles: 2 }),

            PHA     => Ok(Self { opcode, mnemonic: Mnemonic::PHA, addr_mode: IMP, cycles: 3 }),
            PHP     => Ok(Self { opcode, mnemonic: Mnemonic::PHP, addr_mode: IMP, cycles: 3 }),
            PLA     => Ok(Self { opcode, mnemonic: Mnemonic::PLA, addr_mode: IMP, cycles: 4 }),
            PLP     => Ok(Self { opcode, mnemonic: Mnemonic::PLP, addr_mode: IMP, cycles: 4 }),

            NOP     => Ok(Self { opcode, mnemonic: Mnemonic::NOP, addr_mode: IMP, cycles: 2 }),

            BRK     => Ok(Self { opcode, mnemonic: Mnemonic::BRK, addr_mode: IMP, cycles: 7 }),
            RTI     => Ok(Self { opcode, mnemonic: Mnemonic::RTI, addr_mode: IMP, cycles: 6 }),
        }
    }

    pub fn bytes(&self) -> u8 {
        self.addr_mode.instruction_bytes()
    }
}

impl fmt::Debug for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Instruction")
            .field("Opcode", &format!("0x{:02X}", self.opcode))
            .field("Mnemonic", &self.mnemonic)
            .field("AddrMode", &self.addr_mode)
            .field("Bytes", &self.bytes())
            .field("Cycles", &self.cycles)
            .finish()
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Mnemonic {
    ADC,    // Add with Carry
    AND,    // Logical AND
    ASL,    // Arithmetic Shift Left
    BCC,    // Branch if Carry Clear
    BCS,    // Branch if Carry Set
    BEQ,    // Branch if Equal
    BIT,    // Bit Test
    BMI,    // Branch if Minus
    BNE,    // Branch if Not Equal
    BPL,    // Branch if Positive
    BRK,    // Break
    BVC,    // Branch if Overflow Clear
    BVS,    // Branch if Overflow Set
    CLC,    // Clear Carry Flag
    CLD,    // Clear Decimal Mode
    CLI,    // Clear Interrupt Disable
    CLV,    // Clear Overflow Flag
    CMP,    // Compare Accumulator
    CPX,    // Compare X Register
    CPY,    // Compare Y Register
    DEC,    // Decrement Memory
    DEX,    // Decrement X Register
    DEY,    // Decrement Y Register
    EOR,    // Exclusive OR
    INC,    // Increment Memory
    INX,    // Increment X Register
    INY,    // Increment Y Register
    JMP,    // Jump
    JSR,    // Jump to Subroutine
    LDA,    // Load Accumulator
    LDX,    // Load X Register
    LDY,    // Load Y Register
    LSR,    // Logical Shift Right
    NOP,    // No Operation
    ORA,    // Logical OR
    PHA,    // Push Accumulator
    PHP,    // Push Processor Status
    PLA,    // Pull Accumulator
    PLP,    // Pull Processor Status
    ROL,    // Rotate Left
    ROR,    // Rotate Right
    RTI,    // Return from Interrupt
    RTS,    // Return from Subroutine
    SBC,    // Subtract with Carry
    SEC,    // Set Carry Flag
    SED,    // Set Decimal Mode
    SEI,    // Set Interrupt Disable
    STA,    // Store Accumulator
    STX,    // Store X Register
    STY,    // Store Y Register
    TAX,    // Transfer Accumulator to X
    TAY,    // Transfer Accumulator to Y
    TSX,    // Transfer Stack Pointer to X
    TXA,    // Transfer X to Accumulator
    TXS,    // Transfer X to Stack Pointer
    TYA,    // Transfer Y to Accumulator
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum AddressingMode {
    IMP,    // Implied
    ACC,    // Accumulator
    IMM,    // Immediate
    ZPG,    // Zero Page
    ZPX,    // Zero Page,X
    ZPY,    // Zero Page,Y
    REL,    // Relative
    ABS,    // Absolute
    ABX,    // Absolute,X
    ABY,    // Absolute,Y
    IND,    // Indirect
    IDX,    // Indexed Indirect
    IDY,    // Indirect Indexed
}

impl AddressingMode {
    pub fn abbr(&self) -> &'static str {
        let (abbr, _, _) = self.info();
        abbr
    }

    pub fn name(&self) -> &'static str {
        let (_, name, _) = self.info();
        name
    }

    pub fn operands(&self) -> &'static str {
        let (_, _, operands) = self.info();
        operands
    }

    fn info(&self) -> (&'static str, &'static str, &'static str) {
        match self {
            Self::IMP => ("IMP", "Implied",          ""),
            Self::ACC => ("ACC", "Accumulator",      "A"),
            Self::IMM => ("IMM", "Immediate",        "#oper"),
            Self::ZPG => ("ZPG", "Zero Page",        "oper"),
            Self::ZPX => ("ZPX", "Zero Page,X",      "oper,X"),
            Self::ZPY => ("ZPY", "Zero Page,Y",      "oper,Y"),
            Self::REL => ("REL", "Relative",         "oper"),
            Self::ABS => ("ABS", "Absolute",         "oper"),
            Self::ABX => ("ABX", "Absolute,X",       "oper,X"),
            Self::ABY => ("ABY", "Absolute,Y",       "oper,Y"),
            Self::IND => ("IND", "Indirect",         "(oper)"),
            Self::IDX => ("IDX", "Indexed Indirect", "(oper,X)"),
            Self::IDY => ("IDY", "Indirect Indexed", "(oper),Y"),
        }
    }

    pub fn instruction_bytes(&self) -> u8 {
        match self {
            Self::IMP | Self::ACC | Self::IMM => 1,
            Self::ZPG | Self::ZPX | Self::ZPY | Self::REL | Self::IDX | Self::IDY => 2,
            Self::ABS | Self::ABX | Self::ABY | Self::IND => 3,
        }
    }
}

impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbr())
    }
}
