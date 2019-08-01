use std::env;

use crate::addressspace::{AddressSpace, MemoryDevice};
use crate::cpu::Cpu;

mod addressspace;
mod cpu;
mod instruction;
mod loader;
mod opcode;
mod util;

fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).expect("Expected filename");

    let mut memory = AddressSpace::new();
    loader::load_program(path, &mut memory).unwrap();

    let mut cpu = Cpu::new(&mut memory);
    cpu.run();
}
