use crate::cpu::Cpu;
use crate::mem::Memory;

pub mod cpu;
pub mod mem;

fn main() {
    println!("rust-6502-emu");

    let mem = Memory::create();
    let cpu = Cpu::create();
    println!("CPU values: {:?}", cpu);
}
