use clap::{App, Arg, ArgMatches};
use std::env;
use std::time::SystemTime;

use crate::addressspace::{AddressSpace, MemoryDevice};
use crate::cpu::Cpu;
use crate::loader::load_program;

mod addressspace;
mod cpu;
mod gdbserver;
mod instruction;
mod loader;
mod ram;
mod util;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let args = parse_commandline();

    let mut memory = AddressSpace::new();
    loader::load_program(&args.path, &mut memory).unwrap();
    let mut cpu = Cpu::new();

    if args.debug_enabled {
        gdbserver::start_server(cpu, memory);
    } else {
        let before = SystemTime::now();
        cpu.run(&mut memory);
        let after = SystemTime::now();

        let elapsed = after.duration_since(before).unwrap().as_micros();
        eprintln!(
            "\nExecuted {} instructions in {:?} µs",
            cpu.get_cycle_counter(),
            elapsed
        );
        eprintln!(
            "Frequency: {} MHz",
            (cpu.get_cycle_counter() as f64 / elapsed as f64)
        );
    }
}

struct CommandLineArgs {
    path: String,
    debug_enabled: bool,
}

fn parse_commandline() -> CommandLineArgs {
    let matches: ArgMatches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(
            Arg::with_name("BINARY")
                .help("Set the binary file to run")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("debug")
                .short("d")
                .help("Enables gdb-remote support"),
        )
        .get_matches();

    let path = matches.value_of("BINARY").unwrap();
    let debug_enabled = matches.is_present("debug");

    CommandLineArgs {
        path: path.to_string(),
        debug_enabled,
    }
}
