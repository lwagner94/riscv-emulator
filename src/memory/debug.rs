use super::addressspace::Address;
use super::addressspace::MemoryDevice;
use crate::util;
use std::io::Write;

pub struct Debug {
    offset: Address,
}

impl Debug {
    pub fn new(offset: Address) -> Box<Debug> {
        if offset % 1 << 20 != 0 {
            panic!("Offset not aligned properly");
        }

        Box::new(Self { offset })
    }
}

impl MemoryDevice for Debug {
    fn read_byte(&self, address: Address) -> u8 {
        unimplemented!();
    }

    fn read_halfword(&self, address: Address) -> u16 {
        unimplemented!();
    }

    fn read_word(&self, address: Address) -> u32 {
        unimplemented!();
    }

    fn write_byte(&mut self, address: Address, val: u8) {
        let relative_address = self.get_relative_address(address);
        match self.get_relative_address(address) {
            0 => {
                print!("{}", val as char);
                std::io::stdout().flush();
            }
            _ => panic!(
                "Invalid debug device access at {:x}, relative address {:x}",
                address, relative_address
            ),
        }
    }

    fn write_halfword(&mut self, address: Address, val: u16) {
        unimplemented!();
    }

    fn write_word(&mut self, address: Address, val: u32) {
        unimplemented!();
    }

    fn offset(&self) -> Address {
        self.offset
    }
}
