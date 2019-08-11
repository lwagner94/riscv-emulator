use super::ram;
use super::ram::Ram;
use crate::instruction::Instruction;
use crate::memory::debug::Debug;
use crate::util;
use std::borrow::BorrowMut;
use std::io::Write;

pub type Address = u32;

pub trait MemoryDevice {
    fn read_byte(&self, address: Address) -> u8;
    fn read_halfword(&self, address: Address) -> u16;
    fn read_word(&self, address: Address) -> u32;

    fn write_byte(&mut self, address: Address, val: u8);
    fn write_halfword(&mut self, address: Address, val: u16);
    fn write_word(&mut self, address: Address, val: u32);

    fn get_relative_address(&self, address: Address) -> Address {
        (address - self.offset())
    }

    fn offset(&self) -> Address;
}

pub struct AddressSpace {
    memory_devices: [Box<dyn MemoryDevice>; 2],
    address_lut: [u32; 4096],
}

impl AddressSpace {
    pub fn new() -> AddressSpace {
        let debug_address = (1 << 20) * 512;

        let mut lut = [0u32; 4096];
        lut[512] = 1;

        AddressSpace {
            memory_devices: [Ram::new(0), Debug::new(debug_address)],
            address_lut: lut,
        }
    }

    fn get_device_for_address_mut(&mut self, address: Address) -> &mut Box<dyn MemoryDevice> {
        let device_index = self.calculate_device_index(address);
        &mut self.memory_devices[device_index]
    }

    fn get_device_for_address(&self, address: Address) -> &Box<dyn MemoryDevice> {
        let device_index = self.calculate_device_index(address);
        &self.memory_devices[device_index]
    }

    fn calculate_device_index(&self, address: Address) -> usize {
        let index = (address >> 20) as usize;
        self.address_lut[index] as usize
    }
}

impl MemoryDevice for AddressSpace {
    fn read_byte(&self, address: Address) -> u8 {
        let device = self.get_device_for_address(address);
        device.read_byte(address)
    }

    fn read_halfword(&self, address: Address) -> u16 {
        let device = self.get_device_for_address(address);
        device.read_halfword(address)
    }

    fn read_word(&self, address: Address) -> u32 {
        let device = self.get_device_for_address(address);
        device.read_word(address)
    }

    fn write_byte(&mut self, address: Address, val: u8) {
        let device = self.get_device_for_address_mut(address);
        device.write_byte(address, val)
    }

    fn write_halfword(&mut self, address: Address, val: u16) {
        let device = self.get_device_for_address_mut(address);
        device.write_halfword(address, val)
    }

    fn write_word(&mut self, address: Address, val: u32) {
        let device = self.get_device_for_address_mut(address);
        device.write_word(address, val)
    }

    fn offset(&self) -> Address {
        0
    }
}

#[cfg(test)]
mod test {}
