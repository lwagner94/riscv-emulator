use crate::error::EmulatorError::ElfFormatError;
use crate::error::EmulatorResult;
use crate::memory::addressspace::{Address, AddressSpace, MemoryDevice};
use goblin::elf::header::{machine_to_str, EM_RISCV};
use goblin::elf::program_header::PT_LOAD;
use goblin::Object;
use std::fs;
use std::path::Path;

pub fn load_program(path: &str, memory: &mut AddressSpace) -> EmulatorResult<()> {
    let path = Path::new(path);
    let buffer = fs::read(path)?;

    match Object::parse(&buffer).unwrap() {
        Object::Elf(elf) => {
            if elf.header.e_machine != EM_RISCV {
                return Err(ElfFormatError(format!(
                    "invalid architecture: {}",
                    machine_to_str(elf.header.e_machine)
                )));
            }

            for loadable_phdr in elf
                .program_headers
                .iter()
                .filter(|phdr| phdr.p_type == PT_LOAD)
            {
                let x = &buffer[loadable_phdr.file_range()];

                for (index, &byte) in x.iter().enumerate() {
                    memory.write_byte((index + loadable_phdr.p_vaddr as usize) as Address, byte);
                }
            }
        }
        _ => {
            return Err(ElfFormatError("Invalid binary".into()));
        }
    }

    Ok(())
}
