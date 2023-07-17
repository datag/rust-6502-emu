use crate::cpu::{Cpu, VECTOR_RES};
use crate::mem::{Memory, ADDR_RESET_VECTOR};

pub mod cpu;
pub mod instruction;
pub mod mem;

fn main() {
    println!("rust-6502-emu");

    let mut mem = Memory::create();
    mem.init();

    print!("Reset vector: ");
    mem.dump(VECTOR_RES, 2);
    print!("Data at reset vector address: ");
    mem.dump(ADDR_RESET_VECTOR, 16);

    let mut cpu = Cpu::create(&mut mem);
    cpu.reset();
    println!("After reset: {:#?}", cpu);

    cpu.exec(2);
    println!("After instruction: {:#?}", cpu);

    cpu.exec(2);
    println!("After instruction: {:#?}", cpu);

    cpu.exec(3);
    println!("After instruction: {:#?}", cpu);
}
