use std::error::Error;

use crate::cpu::Cpu;
use crate::mem::Memory;

pub mod cpu;
pub mod instruction;
pub mod mem;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[allow(unused)]
enum Verbosity {
    Quiet = 0,
    Normal = 1,
    Verbose = 2,
    VeryVerbose = 3,
}

pub struct Config {
    verbosity: Verbosity,
    cycles_to_execute: u64,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        let mut cycles_to_execute = 1;

        if args.len() > 1 {
            cycles_to_execute = args[1].parse::<u64>().unwrap();
        }

        let verbosity: Verbosity;
        if args.contains(&String::from("-q")) {
            verbosity = Verbosity::Quiet;
        } else if args.contains(&String::from("-vv")) {
            verbosity = Verbosity::VeryVerbose;
        } else if args.contains(&String::from("-v")) {
            verbosity = Verbosity::Verbose;
        } else {
            verbosity = Verbosity::Normal;
        }

        Ok(Config { verbosity, cycles_to_execute })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    if config.verbosity != Verbosity::Quiet {
        println!("rust-6502-emu");
        if config.verbosity > Verbosity::Normal {
            println!("Being verbose... {:?} [{}]", config.verbosity, config.verbosity as u8);
        }
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
