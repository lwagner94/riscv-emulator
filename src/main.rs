use std::env;
use clap::{Arg, App, ArgMatches};

use crate::addressspace::{AddressSpace, MemoryDevice};
use crate::cpu::Cpu;

mod addressspace;
mod cpu;
mod instruction;
mod loader;
mod util;
mod gdbserver;

const NAME: &'static str = env!("CARGO_PKG_NAME");
const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const AUTHORS: &'static str = env!("CARGO_PKG_AUTHORS");
const DESCRIPTION: &'static str = env!("CARGO_PKG_DESCRIPTION");

fn main() {
    let args = parse_commandline();

    let mut memory = AddressSpace::new();
    loader::load_program(&args.path, &mut memory).unwrap();
    let mut cpu = Cpu::new(&mut memory);

    if args.debug_enabled {
        gdbserver::start_server(cpu);
    }
    else {
        cpu.run();
    }
}

struct CommandLineArgs {
    path: String,
    debug_enabled: bool
}

fn parse_commandline() -> CommandLineArgs {
    let matches: ArgMatches = App::new(NAME)
        .version(VERSION)
        .author(AUTHORS)
        .about(DESCRIPTION)
        .arg(Arg::with_name("BINARY")
            .help("Set the binary file to run")
            .required(true)
            .index(1))
        .arg(Arg::with_name("debug")
            .short("d")
            .help("Enables gdb-remote support"))
        .get_matches();


    let path = matches.value_of("BINARY").unwrap();
    let debug_enabled = matches.is_present("debug");

    CommandLineArgs {
        path: path.to_string(),
        debug_enabled
    }
}
