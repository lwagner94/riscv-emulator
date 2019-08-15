use crate::memory::addressspace::Address;
use crate::memory::addressspace::AddressSpace;

pub struct InterruptController {
    interrupt_table: Address,
    counter: u64
}

impl InterruptController {
    pub fn new() -> Self {
        InterruptController { interrupt_table: 0, counter: 0 }
    }

    pub fn check_for_interrupt(&mut self, memory: &mut AddressSpace) -> Option<Address> {
        self.counter += 1;

//        if self.counter % 101 == 0 {
//            Some(0x24)
//        } else {
//            None
//        }
        None
    }

    pub fn set_interrupt_table(&mut self, address: Address) {
        self.interrupt_table = address;
    }
}
