use std::fs::File;
use std::io;
use std::io::prelude::*;

use crate::memory::addressspace::{Address, AddressSpace, MemoryDevice};

pub fn load_program(path: &str, memory: &mut AddressSpace) -> io::Result<()> {
    let mut f = File::open(path)?;

    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;

    for (index, &byte) in buffer.iter().enumerate() {
        memory.write_byte(index as Address, byte)
    }
    Ok(())
}
