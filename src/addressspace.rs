
use crate::util;

pub type Address = u32;

pub trait MemoryDevice {
    fn read_byte(&self, address: Address) -> u8;
    fn read_halfword(&self, address: Address) -> u16;
    fn read_word(&self, address: Address) -> u32;

    fn write_byte(&mut self, address: Address, val: u8);
    fn write_halfword(&mut self, address: Address, val: u16);
    fn write_word(&mut self, address: Address, val: u32);
}



pub struct AddressSpace {
    memory: Vec<u8>
}

impl AddressSpace {
    pub fn new() -> Self {
        AddressSpace {
            memory: vec![0; 4096] // 4KB for now
        }
    }
}

impl MemoryDevice for AddressSpace {
    fn read_byte(&self, address: Address) -> u8 {
        return self.memory[address as usize];
    }

    fn read_halfword(&self, address: Address) -> u16 {
        let index = address as usize;
        util::read_u16_from_byteslice(&self.memory[index..index+2])
    }

    fn read_word(&self, address: Address) -> u32 {
        let index = address as usize;
        util::read_u32_from_byteslice(&self.memory[index..index+4])
    }

    fn write_byte(&mut self, address: Address, val: u8) {
        self.memory[address as usize] = val;
    }

    fn write_halfword(&mut self, address: Address, val: u16) {
        let index = address as usize;
        util::write_u16_to_byteslice(&mut self.memory[index..index+2], val);
    }

    fn write_word(&mut self, address: Address, val: u32) {
        let index = address as usize;
        util::write_u32_to_byteslice(&mut self.memory[index..index+4], val);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_byte_access() {
        let mut mem = AddressSpace::new();

        mem.write_byte(0, 0xCA);
        assert_eq!(0xCA, mem.read_byte(0))
    }

    #[test]
    fn test_halfword_access() {
        for i in 0..4 {
            let mut mem = AddressSpace::new();

            mem.write_halfword(0 + i, 0xCAFE);
            assert_eq!(0xCAFE, mem.read_halfword(0 + i))
        }
    }

    #[test]
    fn test_word_access() {
        for i in 0..4 {
            let mut mem = AddressSpace::new();

            mem.write_word(0 + i, 0xCAFEBABE);
            assert_eq!(0xCAFEBABE, mem.read_word(0 + i))
        }
    }
}