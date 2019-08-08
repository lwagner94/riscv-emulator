use std::env;

use crate::addressspace::{AddressSpace, MemoryDevice};
use crate::cpu::Cpu;

mod addressspace;
mod cpu;
mod instruction;
mod loader;
mod util;
mod gdbserver;


fn main() {
    let args: Vec<String> = env::args().collect();

    let path = args.get(1).expect("Expected filename");



//    let handle = gdbserver::start_server();
//
//    handle.join();

    let mut memory = AddressSpace::new();
    loader::load_program(path, &mut memory).unwrap();

    let mut cpu = Cpu::new(&mut memory);

    gdbserver::start_server(cpu);

//    cpu.run();


}
