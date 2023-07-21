use std::error::Error;

use crate::cpu::Cpu;
use crate::mem::Memory;

pub mod cpu;
pub mod instruction;
pub mod mem;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[allow(unused)]
pub enum Verbosity {
    Normal = 0,
    Verbose = 1,
    VeryVerbose = 2,
}

pub struct Config {
    pub verbosity: Verbosity,
    pub cycles_to_execute: u64,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("rust-6502-emu");
    if config.verbosity > Verbosity::Normal {
        println!("Being verbose... {:?} [{}]", config.verbosity, config.verbosity as u8);
    }

    let mut mem = Memory::create();
    mem.reset();

    if config.verbosity >= Verbosity::Verbose {
        print!("Reset vector: ");
        mem.dump(cpu::VECTOR_RES, 2);
        print!("Data at reset vector address: ");
        mem.dump(mem::ADDR_RESET_VECTOR, 16);
    }

    let mut cpu = Cpu::create();
    cpu.reset(&mut mem);

    if config.verbosity >= Verbosity::Verbose {
        println!("After reset: {:#?}", cpu);
    }

    // demo data
    mem.demo();

    cpu.exec(&mut mem, config.cycles_to_execute);
    if config.verbosity >= Verbosity::Verbose {
        println!("After exec: {:#?}", cpu);
    }

    Ok(())
}
