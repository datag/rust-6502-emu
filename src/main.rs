use crate::cpu::{Cpu, VECTOR_RES, StatusFlags};
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
    //println!("After reset: {:#?}", cpu);

    // cpu.exec(2);
    // println!("After #1 ADC: {:#?}", cpu);

    // cpu.exec(2);
    // println!("After #2 ADC: {:#?}", cpu);

    // cpu.exec(3);
    // println!("After JMP: {:#?}", cpu);

    // cpu.exec(2);
    // println!("After ADC #1 again: {:#?}", cpu);

    cpu.sr.set(StatusFlags::C, false);
    cpu.exec(1);
    println!("After BCC: {:#?}", cpu);

    cpu.exec(10);
}
