use std::io;
use std::io::prelude::*;
use std::fs::File;


use crate::addressspace::{AddressSpace, MemoryDevice, Address};

pub fn load_program(path: &str, memory: &mut AddressSpace) -> io::Result<()> {
    let mut f = File::open(path)?;

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    for (index, &byte) in buffer.iter().enumerate() {
        memory.write_byte(index as Address, byte)
    }
    Ok(())
}