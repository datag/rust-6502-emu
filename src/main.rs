use crate::cpu::create_cpu;

pub mod cpu;

fn main() {
    println!("rust-6502-emu");

    let cpu = create_cpu();
    println!("CPU values: {:?}", cpu);
}
