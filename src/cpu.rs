
// https://www.masswerk.at/6502/6502_instruction_set.html

#[derive(Debug)]
pub struct Cpu {
    pub pc: u16,
    pub ac: u8,
    pub x: u8,
    pub y: u8,
    pub sr: u8,
    pub sp: u8
}

impl Cpu {
    pub fn create() -> Cpu {
        Cpu {
            pc: 0xfffc,
            ac: 0,
            x: 0,
            y: 0,
            sr: 0,
            sp: 0,
        }
    }
}
