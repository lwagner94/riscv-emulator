use super::addressspace::Address;
use super::addressspace::MemoryDevice;
use std::io::Write;

pub struct Debug {
    offset: Address,
}

impl Debug {
    pub fn new(offset: Address) -> Debug {
        if offset % (1 << 20) != 0 {
            panic!("Offset not aligned properly");
        }

        Self { offset }
    }
}

impl MemoryDevice for Debug {
    fn read_byte(&self, _address: Address) -> u8 {
        unimplemented!();
    }

    fn read_halfword(&self, _address: Address) -> u16 {
        unimplemented!();
    }

    fn read_word(&self, _address: Address) -> u32 {
        unimplemented!();
    }

    fn write_byte(&mut self, address: Address, val: u8) {
        let relative_address = self.get_relative_address(address);
        match self.get_relative_address(address) {
            0 => {
                print!("{}", val as char);
                std::io::stdout().flush().unwrap();
            }
            _ => panic!(
                "Invalid debug device access at {:x}, relative address {:x}",
                address, relative_address
            ),
        }
    }

    fn write_halfword(&mut self, _address: Address, _val: u16) {
        unimplemented!();
    }

    fn write_word(&mut self, _address: Address, _val: u32) {
        unimplemented!();
    }

    fn offset(&self) -> Address {
        self.offset
    }
}
