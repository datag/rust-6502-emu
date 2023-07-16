use crate::cpu::Cpu;
use crate::mem::Memory;

pub mod cpu;
pub mod mem;

fn main() {
    println!("rust-6502-emu");

    let mut mem = Memory::create();

    let mut cpu = Cpu::create(&mut mem);
    cpu.reset();

    println!("CPU values: {:#?}", cpu);
}
