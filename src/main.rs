use std::process;
use clap::Parser;
use rust_6502_emu::{Config, Verbosity};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Cycles to execute
    #[arg(short,long)]
    cycles: Option<u64>,

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
        cycles_to_execute: args.cycles,
        load_demo: args.demo,
        load_file: args.file,
        verbosity,
    };

    if let Err(err) = rust_6502_emu::run(config) {
        println!("Application error: {err}");
        process::exit(1);
    }
}
