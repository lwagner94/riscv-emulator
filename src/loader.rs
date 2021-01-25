use std::fs::File;
use std::{io, fs};
use std::io::prelude::*;

use crate::memory::addressspace::{Address, AddressSpace, MemoryDevice};
use std::path::Path;
use goblin::Object;
use goblin::elf::program_header::PT_LOAD;
use goblin::elf::header::{EM_RISCV, machine_to_str};

pub fn load_program(path: &str, memory: &mut AddressSpace) -> io::Result<()> {
    let path = Path::new(path);
    let buffer = fs::read(path)?;

    match Object::parse(&buffer).unwrap() {
        Object::Elf(elf) => {
            if elf.header.e_machine != EM_RISCV {
                panic!("Invalid architecture: {}", machine_to_str(elf.header.e_machine));
            }


            for loadable_phdr in elf.program_headers.iter().filter(|phdr| {phdr.p_type == PT_LOAD}) {

                let x = &buffer[loadable_phdr.file_range()];
                println!("{:?}", x);

                for (index, &byte) in x.iter().enumerate() {
                    memory.write_byte((index + loadable_phdr.p_vaddr as usize) as Address, byte);
                }


            }

            // println!("elf: {:#?}", &elf);
        },
        Object::PE(pe) => {
            println!("pe: {:#?}", &pe);
        },
        Object::Mach(mach) => {
            println!("mach: {:#?}", &mach);
        },
        Object::Archive(archive) => {
            println!("archive: {:#?}", &archive);
        },
        Object::Unknown(magic) => { println!("unknown magic: {:#x}", magic) }
    }

    Ok(())
}
