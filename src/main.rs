use crate::cpu::Cpu;
use crate::mem::Memory;

pub mod cpu;
pub mod mem;

fn main() {
    println!("rust-6502-emu");

    let mut mem = Memory::create();
    mem.init();

    let mut cpu = Cpu::create(&mut mem);
    cpu.reset();

    println!("CPU values: {:#?}", cpu);


    println!("Mem @ 0xfffc: 0x{:02X}  0xfffd: 0x{:02X}", mem.read_u8(0xfffc), mem.read_u8(0xfffd));
    println!("Mem @ 0xfffc: 0x{:04X}", mem.read_u16(0xfffc));
}
