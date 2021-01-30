use super::ram::Ram;
use super::video::Video;
use crate::memory::debug::Debug;
use std::borrow::Borrow;
use std::sync::atomic::AtomicU32;
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
    ram: Ram,
    debug: Debug,
    video: Video,
}

impl AddressSpace {
    pub fn new() -> AddressSpace {
        let debug_address = (1 << 20) * 512;
        let video_address = (1 << 20) * 1024;

        let interrupt_flags = Arc::new(AtomicU32::new(0));

        AddressSpace {
            ram: Ram::new(0),
            debug: Debug::new(debug_address),
            video: Video::new(video_address, interrupt_flags),

        }
    }
}

impl Default for AddressSpace {
    fn default() -> Self {
        Self::new()
    }
}


const MEGABYTE: Address = 1024*1024;

const RAM_START: Address = 0;
const RAM_END: Address = MEGABYTE * 128 - 1;

const DEBUG_START: Address = MEGABYTE * 512;
const DEBUG_END: Address = MEGABYTE * 513 - 1;

const VIDEO_START: Address = MEGABYTE * 1024;
const VIDEO_END: Address = MEGABYTE * 1027 - 1;


macro_rules! read_impl {
    ($self:ident, $func:ident, $addr:expr) => {{

        let result = match $addr {
            RAM_START..=RAM_END => $self.ram.$func($addr),
            DEBUG_START..=DEBUG_END => $self.debug.$func($addr),
            VIDEO_START..=VIDEO_END => $self.video.$func($addr),
            _ => panic!("Invalid read at {}", $addr)
        };

        result
    }}
}

macro_rules! write_impl {
    ($self:ident, $func:ident, $addr:expr, $value:expr) => {{

        match $addr {
            RAM_START..=RAM_END => $self.ram.$func($addr, $value),
            DEBUG_START..=DEBUG_END => $self.debug.$func($addr, $value),
            VIDEO_START..=VIDEO_END => $self.video.$func($addr, $value),
            _ => panic!("Invalid read at {}", $addr)
        };
    }}
}

impl MemoryDevice for AddressSpace {
    fn read_byte(&self, address: Address) -> u8 {
        read_impl!(self, read_byte, address)
    }

    fn read_halfword(&self, address: Address) -> u16 {
        read_impl!(self, read_halfword, address)
    }

    fn read_word(&self, address: Address) -> u32 {
        read_impl!(self, read_word, address)
    }

    fn write_byte(&mut self, address: Address, val: u8) {
        write_impl!(self, write_byte, address, val);
    }

    fn write_halfword(&mut self, address: Address, val: u16) {
        write_impl!(self, write_halfword, address, val);
    }

    fn write_word(&mut self, address: Address, val: u32) {
        write_impl!(self, write_word, address, val);
    }

    fn offset(&self) -> Address {
        0
    }

    #[inline(always)]
    fn check_for_interrupt(&mut self) -> Option<Address> {
        // let interrupt_flag: u32 = self.interrupt_flags.load(Ordering::SeqCst);
        // if let Some(nr) = get_interrupt_number(interrupt_flag) {
        //     self.interrupt_flags.store(0, Ordering::SeqCst);
        //     Some(0x24)
        // } else {
        //     None
        // }
        None
    }
}

// fn get_interrupt_number(flag: u32) -> Option<u32> {
//     let leading = flag.leading_zeros();
//     let size = (size_of_val(&flag) * 8) as u32;
//
//     if leading != size {
//         Some(size - leading - 1)
//     } else {
//         None
//     }
// }

#[cfg(test)]
mod test {
    // use super::*;

    // #[test]
    // fn test_get_interrupt_number() {
    //     assert_eq!(
    //         get_interrupt_number(0b0000_0000_0000_0000_0000_0000_0000_0000u32),
    //         None
    //     );
    //     assert_eq!(
    //         get_interrupt_number(0b0000_0000_0000_0000_0000_0000_0000_0001u32),
    //         Some(0)
    //     );
    //     assert_eq!(
    //         get_interrupt_number(0b0000_0000_0000_0000_0000_0000_0000_1111u32),
    //         Some(3)
    //     );
    // }
}
