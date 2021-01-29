use super::addressspace::Address;
use super::addressspace::MemoryDevice;
use std::io::Write;
use super::super::util;

const MEGABYTE: usize = 1024 * 1024;

pub struct Debug {
    offset: Address,
    memory: Vec<u8>,
}

impl Debug {
    pub fn new(offset: Address) -> Debug {
        if offset % (1 << 20) != 0 {
            panic!("Offset not aligned properly");
        }

        Self { offset, memory: vec![0; MEGABYTE] }
    }

    fn output_hook(&self, address: Address) {
        let relative_address = self.get_relative_address(address);
        match self.get_relative_address(address) {
            0 => {
                print!("{}", self.read_byte(address) as char);
                std::io::stdout().flush().unwrap();
            }
            _ => {}
        }
    }
}

impl MemoryDevice for Debug {
    fn read_byte(&self, address: Address) -> u8 {
        let index = self.get_relative_address(address)  as usize;
        self.memory[index]
    }

    fn read_halfword(&self, address: Address) -> u16 {
        let index = self.get_relative_address(address) as usize;
        util::read_u16_from_byteslice(&self.memory[index..index + 2])
    }

    fn read_word(&self, address: Address) -> u32 {
        let index = self.get_relative_address(address)  as usize;
        util::read_u32_from_byteslice(&self.memory[index..index + 4])
    }

    fn write_byte(&mut self, address: Address, val: u8) {
        let index = self.get_relative_address(address)  as usize;
        self.memory[index] = val;
        self.output_hook(address);
    }

    fn write_halfword(&mut self, address: Address, val: u16) {
        let index = self.get_relative_address(address)  as usize;
        util::write_u16_to_byteslice(&mut self.memory[index..index + 2], val);
        self.output_hook(address);
    }

    fn write_word(&mut self, address: Address, val: u32) {
        let index = self.get_relative_address(address)  as usize;
        util::write_u32_to_byteslice(&mut self.memory[index..index + 4], val);
        self.output_hook(address);
    }

    fn offset(&self) -> Address {
        self.offset
    }

    fn check_for_interrupt(&mut self) -> Option<Address> {
        None
    }
}
