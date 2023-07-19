pub mod cpu;
pub mod instruction;
pub mod mem;

use crate::cpu::{Cpu, VECTOR_RES, StatusFlags};
use crate::mem::{Memory, ADDR_RESET_VECTOR};

fn main() {
    println!("rust-6502-emu");

    let mut mem = Memory::create();
    mem.reset();

    print!("Reset vector: ");
    mem.dump(VECTOR_RES, 2);
    print!("Data at reset vector address: ");
    mem.dump(ADDR_RESET_VECTOR, 16);

    let mut cpu = Cpu::create();
    cpu.reset(&mut mem);
    //println!("After reset: {:#?}", cpu);

    // cpu.exec(2);
    // println!("After #1 ADC: {:#?}", cpu);

    // cpu.exec(2);
    // println!("After #2 ADC: {:#?}", cpu);

    // cpu.exec(3);
    // println!("After JMP: {:#?}", cpu);

    // cpu.exec(2);
    // println!("After ADC #1 again: {:#?}", cpu);

    cpu.sr.set(StatusFlags::V, false);
    cpu.exec(&mut mem, 1);
    println!("After B**: {:?}", cpu);

    cpu.exec(&mut mem, 10);

}
