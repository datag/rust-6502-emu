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
    pub load_demo: bool,
    pub load_file: Option<String>,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("rust-6502-emu");
    if config.verbosity > Verbosity::Normal {
        println!("Being verbose... {:?} [{}]", config.verbosity, config.verbosity as u8);
    }

    let mut mem = Memory::create();
    let mut cpu = Cpu::create();
    cpu.reset(&mut mem);

    if let Some(filename) = config.load_file {
        if let Err(error) = mem.load_from_file(mem::ADDR_RESET_VECTOR, &filename) {
            panic!("Error reading file into memory: {error}");
        }
    }

    if config.load_demo {
        mem.demo();
    }

    if config.verbosity >= Verbosity::Verbose {
        print!("Reset vector: ");
        
        mem.dump(cpu::VECTOR_RES, 2);
        print!("Data at reset vector address: ");
        mem.dump(mem::ADDR_RESET_VECTOR, 16);

        println!("After reset: {:#?}", cpu);
    }



    cpu.exec(&mut mem, config.cycles_to_execute);
    if config.verbosity >= Verbosity::Verbose {
        println!("After exec: {:#?}", cpu);
    }

    Ok(())
}
