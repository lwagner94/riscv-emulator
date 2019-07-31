use std::io;
use std::io::prelude::*;
use std::fs::File;


mod cpu;
mod instruction;
mod opcode;
mod addressspace;
mod util;

fn main() -> io::Result<()> {
    let mut f = File::open("programs/notmain.bin")?;

    let mut buffer = Vec::new();
    // read the whole file
    f.read_to_end(&mut buffer)?;




    for byte in &buffer {
        println!("{:x}", byte)
    }

    // and more! See the other methods for more details.
    Ok(())
}