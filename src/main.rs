use crate::cpu::Cpu;

pub mod cpu;

fn main() {
    println!("rust-6502-emu");

    let cpu = Cpu::create();
    println!("CPU values: {:?}", cpu);
}
