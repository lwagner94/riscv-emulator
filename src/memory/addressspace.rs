use super::ram::Ram;
use super::video::Video;
use crate::memory::debug::Debug;
use std::borrow::Borrow;
use std::mem::size_of_val;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

pub type Address = u32;

pub trait MemoryDevice {
    fn read_byte(&self, address: Address) -> u8;
    fn read_halfword(&self, address: Address) -> u16;
    fn read_word(&self, address: Address) -> u32;

    fn write_byte(&mut self, address: Address, val: u8);
    fn write_halfword(&mut self, address: Address, val: u16);
    fn write_word(&mut self, address: Address, val: u32);

    fn get_relative_address(&self, address: Address) -> Address {
        address - self.offset()
    }

    fn offset(&self) -> Address;

    fn check_for_interrupt(&mut self) -> Option<Address>;
}

pub struct AddressSpace {
    memory_devices: [Box<dyn MemoryDevice>; 3],
    address_lut: [u32; 4096],
    interrupt_flags: Arc<AtomicU32>,
}

impl AddressSpace {
    pub fn new() -> AddressSpace {
        let debug_address = (1 << 20) * 512;
        let video_address = (1 << 20) * 1024;

        let mut lut = [0u32; 4096];
        lut[512] = 1;
        lut[1024] = 2;
        lut[1025] = 2;
        lut[1026] = 2;

        let interrupt_flags = Arc::new(AtomicU32::new(0));

        AddressSpace {
            memory_devices: [
                Box::new(Ram::new(0)),
                Box::new(Debug::new(debug_address)),
                Box::new(Video::new(video_address, interrupt_flags.clone())),
            ],
            address_lut: lut,
            interrupt_flags,
        }
    }

    fn get_device_for_address_mut(&mut self, address: Address) -> &mut dyn MemoryDevice {
        let device_index = self.calculate_device_index(address);
        &mut *self.memory_devices[device_index]
    }

    fn get_device_for_address(&self, address: Address) -> &dyn MemoryDevice {
        let device_index = self.calculate_device_index(address);
        &*self.memory_devices[device_index].borrow()
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

    #[inline(always)]
    fn check_for_interrupt(&mut self) -> Option<Address> {
        let interrupt_flag: u32 = self.interrupt_flags.load(Ordering::SeqCst);
        if let Some(nr) = get_interrupt_number(interrupt_flag) {
            self.interrupt_flags.store(0, Ordering::SeqCst);
            Some(0x24)
        } else {
            None
        }
    }
}

fn get_interrupt_number(flag: u32) -> Option<u32> {
    let leading = flag.leading_zeros();
    let size = (size_of_val(&flag) * 8) as u32;

    if leading != size {
        Some(size - leading - 1)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_interrupt_number() {
        assert_eq!(
            get_interrupt_number(0b0000_0000_0000_0000_0000_0000_0000_0000u32),
            None
        );
        assert_eq!(
            get_interrupt_number(0b0000_0000_0000_0000_0000_0000_0000_0001u32),
            Some(0)
        );
        assert_eq!(
            get_interrupt_number(0b0000_0000_0000_0000_0000_0000_0000_1111u32),
            Some(3)
        );
    }
}
