use crate::cpu::{Cpu, VECTOR_RES};
use crate::mem::{Memory, ADDR_RESET_VECTOR};

pub mod cpu;
pub mod mem;

fn main() {
    println!("rust-6502-emu");


    let mut mem = Memory::create();
    mem.init();

    //println!("Mem @ 0xfffc: 0x{:02X}  0xfffd: 0x{:02X}", mem.read_u8(VECTOR_RES), mem.read_u8(VECTOR_RES + 1));
    //println!("Mem @ 0x{:04X}: 0x{:04X}", VECTOR_RES, mem.read_u16(VECTOR_RES));
    mem.dump(VECTOR_RES, 2);
    mem.dump(ADDR_RESET_VECTOR, 16);

    let mut cpu = Cpu::create(&mut mem);
    cpu.reset();

    println!("CPU values: {:#?}", cpu);
}
