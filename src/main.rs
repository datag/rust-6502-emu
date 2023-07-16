use crate::cpu::Cpu;
use crate::mem::Memory;

pub mod cpu;
pub mod mem;

fn main() {
    println!("rust-6502-emu");

    let mut mem = Memory::create();
    mem.init();

    let cpu = Cpu::create(&mem);
    println!("CPU values: {:?}", cpu);
}
