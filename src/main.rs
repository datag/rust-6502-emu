use std::process;
use clap::Parser;
use rust_6502_emu::{Config, Verbosity};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Maximum cycles to execute
    #[arg(default_value_t = 1)]
    cycles_to_execute: u64,

    /// Load demo data
    #[arg(short, long)]
    demo: bool,

    /// Load data from file
    #[arg(short, long)]
    file: Option<String>,

    /// Verbosity; can be specified multiple times
    #[arg(short, long, action = clap::ArgAction::Count, default_value_t = 0)]
    verbose: u8,
}

fn main() {
    let args = Cli::parse();

    let verbosity = match args.verbose {
        0 => Verbosity::Normal,
        1 => Verbosity::Verbose,
        2 => Verbosity::VeryVerbose,
        _ => Verbosity::Normal,
    };

    let config = Config {
        cycles_to_execute: args.cycles_to_execute,
        load_demo: args.demo,
        load_file: args.file,
        verbosity,
    };

    tinker();

    if let Err(err) = rust_6502_emu::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}

#[allow(unused)]
fn tinker() {
    // 10000000 (binary, -128 in decimal) to 11111111 (binary, -1 in decimal)
    for (m, a, c) in [
        // (0b01000000 as i8, 0b01000000 as i8, 0 as i8),      // NV

        // (0b10000000 as i8, 0b10000000 as i8, 0 as i8),      // ZCV
        // (0b10000000 as i8, 0b10000000 as i8, 1 as i8),      // N

        // (0b10000000 as i8, 0b01111111 as i8, 0 as i8),      // CV


        (-0b01111101 as i8, 0b01111111 as i8, 0 as i8),      // 
        
    ] {
        let result_u8 = m.wrapping_add(a).wrapping_add(c);
        println!("{:08b} [${:02X}] [{:>4}] [{:>4}]  +  {:08b} [${:02X}] [{:>4}] [{:>4}]  +  {}  =  {:08b} [${:02X}] [{:>4}] [{:>4}]",
            m, m, m, m as i8,
            a, a, a, a as i8,
            c,
            result_u8, result_u8, result_u8, result_u8 as i8);
    }
}
