use std::fmt;
use AddressingMode::*;

// ADC - Add with Carry
pub const ADC_IMM: u8 = 0x69;
pub const ADC_ZPG: u8 = 0x65;
pub const ADC_ZPX: u8 = 0x75;
pub const ADC_ABS: u8 = 0x6D;
pub const ADC_ABX: u8 = 0x7D;
pub const ADC_ABY: u8 = 0x79;
pub const ADC_IDX: u8 = 0x61;
pub const ADC_IDY: u8 = 0x71;

// SBC - Subtract Memory from Accumulator with Borrow
pub const SBC_IMM: u8 = 0xE9;
pub const SBC_ZPG: u8 = 0xE5;
pub const SBC_ZPX: u8 = 0xF5;
pub const SBC_ABS: u8 = 0xED;
pub const SBC_ABX: u8 = 0xFD;
pub const SBC_ABY: u8 = 0xF9;
pub const SBC_IDX: u8 = 0xE1;
pub const SBC_IDY: u8 = 0xF1;

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

// LDA - Load Accumulator with Memory
pub const LDA_IMM: u8 = 0xA9;
pub const LDA_ZPG: u8 = 0xA5;
pub const LDA_ZPX: u8 = 0xB5;
pub const LDA_ABS: u8 = 0xAD;
pub const LDA_ABX: u8 = 0xBD;
pub const LDA_ABY: u8 = 0xB9;
pub const LDA_IDX: u8 = 0xA1;
pub const LDA_IDY: u8 = 0xB1;

// LDX - Load Index X with Memory
pub const LDX_IMM: u8 = 0xA2;
pub const LDX_ZPG: u8 = 0xA6;
pub const LDX_ZPY: u8 = 0xB6;
pub const LDX_ABS: u8 = 0xAE;
pub const LDX_ABY: u8 = 0xBE;

// LDY - Load Index Y with Memory
pub const LDY_IMM: u8 = 0xA0;
pub const LDY_ZPG: u8 = 0xA4;
pub const LDY_ZPY: u8 = 0xB4;
pub const LDY_ABS: u8 = 0xAC;
pub const LDY_ABY: u8 = 0xBC;

// STA - Store Accumulator in Memory
pub const STA_ZPG: u8 = 0x85;
pub const STA_ZPX: u8 = 0x95;
pub const STA_ABS: u8 = 0x8D;
pub const STA_ABX: u8 = 0x9D;
pub const STA_ABY: u8 = 0x99;
pub const STA_IDX: u8 = 0x81;
pub const STA_IDY: u8 = 0x91;

// STX - Store Index X in Memory
pub const STX_ZPG: u8 = 0x86;
pub const STX_ZPY: u8 = 0x96;
pub const STX_ABS: u8 = 0x8E;

// STY - Store Index Y in Memory
pub const STY_ZPG: u8 = 0x84;
pub const STY_ZPX: u8 = 0x94;
pub const STY_ABS: u8 = 0x8C;

// NOP - No Operation
pub const NOP: u8 = 0xEA;

pub struct Instruction {
    pub opcode: u8,
    pub mnemonic: Mnemonic,
    pub addr_mode: AddressingMode,
    pub bytes: u8,
    pub cycles: u8,
}

impl Instruction {
    pub fn from_opcode(opcode: u8) -> Result<Self, ()> {
        match opcode {
            ADC_IMM => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: IMM, bytes: 2, cycles: 2 }),
            ADC_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            ADC_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: ZPX, bytes: 2, cycles: 4 }),
            ADC_ABS => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: ABS, bytes: 3, cycles: 4 }),
            ADC_ABX => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: ABX, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ADC_ABY => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: ABY, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ADC_IDX => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: IDX, bytes: 2, cycles: 6 }),
            ADC_IDY => Ok(Self { opcode, mnemonic: Mnemonic::ADC, addr_mode: IDY, bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            SBC_IMM => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: IMM, bytes: 2, cycles: 2 }),
            SBC_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            SBC_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: ZPX, bytes: 2, cycles: 4 }),
            SBC_ABS => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: ABS, bytes: 3, cycles: 4 }),
            SBC_ABX => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: ABX, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            SBC_ABY => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: ABY, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            SBC_IDX => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: IDX, bytes: 2, cycles: 6 }),
            SBC_IDY => Ok(Self { opcode, mnemonic: Mnemonic::SBC, addr_mode: IDY, bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            BCC_REL => Ok(Self { opcode, mnemonic: Mnemonic::BCC, addr_mode: REL, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BCS_REL => Ok(Self { opcode, mnemonic: Mnemonic::BCS, addr_mode: REL, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BEQ_REL => Ok(Self { opcode, mnemonic: Mnemonic::BEQ, addr_mode: REL, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BNE_REL => Ok(Self { opcode, mnemonic: Mnemonic::BNE, addr_mode: REL, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BPL_REL => Ok(Self { opcode, mnemonic: Mnemonic::BPL, addr_mode: REL, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BMI_REL => Ok(Self { opcode, mnemonic: Mnemonic::BMI, addr_mode: REL, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BVC_REL => Ok(Self { opcode, mnemonic: Mnemonic::BVC, addr_mode: REL, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),
            BVS_REL => Ok(Self { opcode, mnemonic: Mnemonic::BVS, addr_mode: REL, bytes: 2, cycles: 2 /* +1 if branch occurs on same page, +2 if on different page */}),

            BIT_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::BIT, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            BIT_ABS => Ok(Self { opcode, mnemonic: Mnemonic::BIT, addr_mode: ABS, bytes: 3, cycles: 4 }),

            AND_IMM => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: IMM, bytes: 2, cycles: 2 }),
            AND_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            AND_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: ZPX, bytes: 2, cycles: 4 }),
            AND_ABS => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: ABS, bytes: 3, cycles: 4 }),
            AND_ABX => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: ABX, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            AND_ABY => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: ABY, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            AND_IDX => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: IDX, bytes: 2, cycles: 6 }),
            AND_IDY => Ok(Self { opcode, mnemonic: Mnemonic::AND, addr_mode: IDY, bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            EOR_IMM => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: IMM, bytes: 2, cycles: 2 }),
            EOR_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            EOR_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: ZPX, bytes: 2, cycles: 4 }),
            EOR_ABS => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: ABS, bytes: 3, cycles: 4 }),
            EOR_ABX => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: ABX, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            EOR_ABY => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: ABY, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            EOR_IDX => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: IDX, bytes: 2, cycles: 6 }),
            EOR_IDY => Ok(Self { opcode, mnemonic: Mnemonic::EOR, addr_mode: IDY, bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            ORA_IMM => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: IMM, bytes: 2, cycles: 2 }),
            ORA_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            ORA_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: ZPX, bytes: 2, cycles: 4 }),
            ORA_ABS => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: ABS, bytes: 3, cycles: 4 }),
            ORA_ABX => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: ABX, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ORA_ABY => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: ABY, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            ORA_IDX => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: IDX, bytes: 2, cycles: 6 }),
            ORA_IDY => Ok(Self { opcode, mnemonic: Mnemonic::ORA, addr_mode: IDY, bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            CLC     => Ok(Self { opcode, mnemonic: Mnemonic::CLC, addr_mode: IMP, bytes: 1, cycles: 2 }),
            CLD     => Ok(Self { opcode, mnemonic: Mnemonic::CLD, addr_mode: IMP, bytes: 1, cycles: 2 }),
            CLI     => Ok(Self { opcode, mnemonic: Mnemonic::CLI, addr_mode: IMP, bytes: 1, cycles: 2 }),
            CLV     => Ok(Self { opcode, mnemonic: Mnemonic::CLV, addr_mode: IMP, bytes: 1, cycles: 2 }),
            SEC     => Ok(Self { opcode, mnemonic: Mnemonic::SEC, addr_mode: IMP, bytes: 1, cycles: 2 }),
            SED     => Ok(Self { opcode, mnemonic: Mnemonic::SED, addr_mode: IMP, bytes: 1, cycles: 2 }),
            SEI     => Ok(Self { opcode, mnemonic: Mnemonic::SEI, addr_mode: IMP, bytes: 1, cycles: 2 }),

            INC_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::INC, addr_mode: ZPG, bytes: 2, cycles: 5 }),
            INC_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::INC, addr_mode: ZPX, bytes: 2, cycles: 6 }),
            INC_ABS => Ok(Self { opcode, mnemonic: Mnemonic::INC, addr_mode: ABS, bytes: 3, cycles: 6 }),
            INC_ABX => Ok(Self { opcode, mnemonic: Mnemonic::INC, addr_mode: ABX, bytes: 3, cycles: 7 }),

            INX     => Ok(Self { opcode, mnemonic: Mnemonic::INX, addr_mode: IMP, bytes: 1, cycles: 2 }),
            INY     => Ok(Self { opcode, mnemonic: Mnemonic::INY, addr_mode: IMP, bytes: 1, cycles: 2 }),

            DEC_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::DEC, addr_mode: ZPG, bytes: 2, cycles: 5 }),
            DEC_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::DEC, addr_mode: ZPX, bytes: 2, cycles: 6 }),
            DEC_ABS => Ok(Self { opcode, mnemonic: Mnemonic::DEC, addr_mode: ABS, bytes: 3, cycles: 6 }),
            DEC_ABX => Ok(Self { opcode, mnemonic: Mnemonic::DEC, addr_mode: ABX, bytes: 3, cycles: 7 }),

            DEX     => Ok(Self { opcode, mnemonic: Mnemonic::DEX, addr_mode: IMP, bytes: 1, cycles: 2 }),
            DEY     => Ok(Self { opcode, mnemonic: Mnemonic::DEY, addr_mode: IMP, bytes: 1, cycles: 2 }),

            JMP_ABS => Ok(Self { opcode, mnemonic: Mnemonic::JMP, addr_mode: ABS, bytes: 3, cycles: 3 }),
            JMP_IND => Ok(Self { opcode, mnemonic: Mnemonic::JMP, addr_mode: IND, bytes: 3, cycles: 5 }),

            LDA_IMM => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: IMM, bytes: 2, cycles: 2 }),
            LDA_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            LDA_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: ZPX, bytes: 2, cycles: 4 }),
            LDA_ABS => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: ABS, bytes: 3, cycles: 4 }),
            LDA_ABX => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: ABX, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            LDA_ABY => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: ABY, bytes: 3, cycles: 4 /* +1 if page crossed */ }),
            LDA_IDX => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: IDX, bytes: 2, cycles: 6 }),
            LDA_IDY => Ok(Self { opcode, mnemonic: Mnemonic::LDA, addr_mode: IDY, bytes: 2, cycles: 5 /* +1 if page crossed */ }),

            LDX_IMM => Ok(Self { opcode, mnemonic: Mnemonic::LDX, addr_mode: IMM, bytes: 2, cycles: 2 }),
            LDX_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::LDX, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            LDX_ZPY => Ok(Self { opcode, mnemonic: Mnemonic::LDX, addr_mode: ZPY, bytes: 2, cycles: 4 }),
            LDX_ABS => Ok(Self { opcode, mnemonic: Mnemonic::LDX, addr_mode: ABS, bytes: 3, cycles: 4 }),
            LDX_ABY => Ok(Self { opcode, mnemonic: Mnemonic::LDX, addr_mode: ABY, bytes: 3, cycles: 4 /* +1 if page crossed */  }),

            LDY_IMM => Ok(Self { opcode, mnemonic: Mnemonic::LDY, addr_mode: IMM, bytes: 2, cycles: 2 }),
            LDY_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::LDY, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            LDY_ZPY => Ok(Self { opcode, mnemonic: Mnemonic::LDY, addr_mode: ZPY, bytes: 2, cycles: 4 }),
            LDY_ABS => Ok(Self { opcode, mnemonic: Mnemonic::LDY, addr_mode: ABS, bytes: 3, cycles: 4 }),
            LDY_ABY => Ok(Self { opcode, mnemonic: Mnemonic::LDY, addr_mode: ABY, bytes: 3, cycles: 4 /* +1 if page crossed */  }),

            STA_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            STA_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: ZPX, bytes: 2, cycles: 4 }),
            STA_ABS => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: ABS, bytes: 3, cycles: 4 }),
            STA_ABX => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: ABX, bytes: 3, cycles: 5 }),
            STA_ABY => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: ABY, bytes: 3, cycles: 5 }),
            STA_IDX => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: IDX, bytes: 2, cycles: 6 }),
            STA_IDY => Ok(Self { opcode, mnemonic: Mnemonic::STA, addr_mode: IDY, bytes: 2, cycles: 6 }),

            STX_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::STX, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            STX_ZPY => Ok(Self { opcode, mnemonic: Mnemonic::STX, addr_mode: ZPY, bytes: 2, cycles: 4 }),
            STX_ABS => Ok(Self { opcode, mnemonic: Mnemonic::STX, addr_mode: ABS, bytes: 3, cycles: 4 }),

            STY_ZPG => Ok(Self { opcode, mnemonic: Mnemonic::STY, addr_mode: ZPG, bytes: 2, cycles: 3 }),
            STY_ZPX => Ok(Self { opcode, mnemonic: Mnemonic::STY, addr_mode: ZPX, bytes: 2, cycles: 4 }),
            STY_ABS => Ok(Self { opcode, mnemonic: Mnemonic::STY, addr_mode: ABS, bytes: 3, cycles: 4 }),

            NOP     => Ok(Self { opcode, mnemonic: Mnemonic::NOP, addr_mode: IMP, bytes: 1, cycles: 2 }),

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
}

impl fmt::Display for AddressingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.abbr())
    }
}
