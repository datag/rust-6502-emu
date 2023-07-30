use std::error::Error;
use std::io::{self, Write};

use colored::Colorize;

use crate::cpu::Cpu;
use crate::mem::Memory;

pub mod cpu;
pub mod instruction;
pub mod mem;

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Verbosity {
    Normal = 0,
    Verbose = 1,
    VeryVerbose = 2,
}

pub struct Config {
    pub verbosity: Verbosity,
    pub cycles_to_execute: Option<u64>,
    pub load_demo: bool,
    pub load_file: Option<String>,
    pub interactive: bool,
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

    cpu.dump_state(&mem);

    if config.interactive {
        while let Ok(user_input) = get_user_input() {
            if user_input.is_empty() {
                // probably ^D
                break;
            }
            let user_input = user_input.trim();
            if ! process_user_input(&mut cpu, &mut mem, user_input) {
                break;
            }
        }
    } else if let Some(cycles_to_execute) = config.cycles_to_execute {
        cpu.exec(&mut mem, cycles_to_execute);
    } else {
        loop {
            cpu.exec(&mut mem, 1);
        }
    }

    Ok(())
}


fn get_user_input() -> Result<String, Box<dyn Error>> {
    let mut user_input = String::new();
    let stdin = io::stdin();
    print!("{} ", "?".on_blue().white().bold());
    _ = std::io::stdout().flush();
    stdin.read_line(&mut user_input)?;
    Ok(user_input)
}

fn process_user_input(cpu: &mut Cpu, mem: &mut Memory, user_input: &str) -> bool {
    let (command, _args) = user_input.split_once(' ').unwrap_or((user_input, ""));

    match command {
        "" => {},
        "h" | "?" => {
            println!("{}", "Help".bold());
            println!("{} - Quit", "q".yellow().bold());
            println!("{} - Single step", "s".yellow().bold());
            println!("{} - Run continuously", "r".yellow().bold());
        },
        "q" => return false,
        "s" => cpu.exec(mem, 1),
        "r" => {
            loop {
                cpu.exec(mem, 1);
            }
        },
        _ => println!("Unknown command '{command}'. Try 'h' or '?'  for help."),
    }

    true
}
