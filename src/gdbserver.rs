use gdb_remote_protocol::{
    process_packets_from, Breakpoint, Error, Handler, MemoryRegion, ProcessType, StopReason,
    ThreadId, Watchpoint,
};
use std::net::TcpListener;

use std::cell::RefCell;

use crate::memory::addressspace::MemoryDevice;
use crate::util;
use crate::AddressSpace;
use crate::Cpu;

const BIND_ADDRESS: &'static str = "0.0.0.0:3000";

struct NoopHandler {
    cpu: RefCell<Cpu>,
    memory: RefCell<AddressSpace>,
}

impl<'a> Handler for NoopHandler {
    fn query_supported_features(&self) -> Vec<String> {
        let mut v = vec![];
        v
    }

    fn attached(&self, _pid: Option<u64>) -> Result<ProcessType, Error> {
        Ok(ProcessType::Created)
    }
    fn detach(&self, _pid: Option<u64>) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
    fn kill(&self, _pid: Option<u64>) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
    fn ping_thread(&self, _id: ThreadId) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn read_memory(&self, region: MemoryRegion) -> Result<Vec<u8>, Error> {
        let mut memory = self.memory.borrow_mut();
        let mut memory_content = Vec::with_capacity(region.length as usize);
        for address in region.address..region.address + region.length {
            memory_content.push(memory.read_byte(address as u32));
        }

        Ok(memory_content)
    }

    /// Write the provided bytes to memory at the given address.
    fn write_memory(&self, _address: u64, _bytes: &[u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
    fn read_register(&self, _register: u64) -> Result<Vec<u8>, Error> {
        Err(Error::Unimplemented)
    }
    fn write_register(&self, _register: u64, _contents: &[u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }
    fn read_general_registers(&self) -> Result<Vec<u8>, Error> {
        let size = std::mem::size_of::<u32>();

        let mut v = vec![0u8; 33 * size];
        let cpu = self.cpu.borrow();

        for i in 0..=31 {
            let start_index = i * size;
            let end_index = start_index + size;
            util::write_u32_to_byteslice(&mut v[start_index..end_index], cpu.get_register(i));
        }

        util::write_u32_to_byteslice(&mut v[32 * size..33 * size], cpu.get_pc());
        Ok(v)
    }
    fn write_general_registers(&self, _contents: &[u8]) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn current_thread(&self) -> Result<Option<ThreadId>, Error> {
        Ok(None)
    }

    fn set_current_thread(&self, _id: ThreadId) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn search_memory(
        &self,
        _address: u64,
        _length: u64,
        _bytes: &[u8],
    ) -> Result<Option<u64>, Error> {
        Err(Error::Unimplemented)
    }

    fn halt_reason(&self) -> Result<StopReason, Error> {
        Ok(StopReason::Signal(5))
    }

    fn invoke(&self, data: &[u8]) -> Result<String, Error> {
        Err(Error::Unimplemented)
    }

    fn set_address_randomization(&self, _enable: bool) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn catch_syscalls(&self, _syscalls: Option<Vec<u64>>) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn set_pass_signals(&self, _signals: Vec<u64>) -> Result<(), Error> {
        Ok(())
    }

    fn set_program_signals(&self, _signals: Vec<u64>) -> Result<(), Error> {
        Ok(())
    }

    fn thread_info(&self, _thread: ThreadId) -> Result<String, Error> {
        Err(Error::Unimplemented)
    }

    fn insert_software_breakpoint(&self, breakpoint: Breakpoint) -> Result<(), Error> {
        let mut cpu = self.cpu.borrow_mut();
        cpu.add_breakpoint(breakpoint.addr as u32);
        Ok(())
    }

    fn insert_hardware_breakpoint(&self, _breakpoint: Breakpoint) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn insert_write_watchpoint(&self, _watchpoint: Watchpoint) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn insert_read_watchpoint(&self, _watchpoint: Watchpoint) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn insert_access_watchpoint(&self, _watchpoint: Watchpoint) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn remove_software_breakpoint(&self, breakpoint: Breakpoint) -> Result<(), Error> {
        let mut cpu = self.cpu.borrow_mut();
        cpu.remove_breakpoint(breakpoint.addr as u32);
        Ok(())
    }

    fn remove_hardware_breakpoint(&self, _breakpoint: Breakpoint) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn remove_write_watchpoint(&self, _watchpoint: Watchpoint) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn remove_read_watchpoint(&self, _watchpoint: Watchpoint) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn remove_access_watchpoint(&self, _watchpoint: Watchpoint) -> Result<(), Error> {
        Err(Error::Unimplemented)
    }

    fn step(&self) -> Result<StopReason, Error> {
        let mut cpu = self.cpu.borrow_mut();
        let mut memory = self.memory.borrow_mut();
        cpu.step(&mut memory);
        Ok(StopReason::Signal(5))
    }

    fn cont(&self) -> Result<StopReason, Error> {
        let mut cpu = self.cpu.borrow_mut();
        let mut memory = self.memory.borrow_mut();
        cpu.cont(&mut memory);
        Ok(StopReason::Signal(5))
    }
}

pub fn start_server(cpu: Cpu, memory: AddressSpace) {
    //    drop(env_logger::init());
    let listener = TcpListener::bind(BIND_ADDRESS).unwrap();
    eprintln!("Listening on {}", BIND_ADDRESS);
    let res = listener.incoming().next().unwrap();

    eprintln!("Got connection");
    if let Ok(stream) = res {
        let h = NoopHandler {
            cpu: RefCell::new(cpu),
            memory: RefCell::new(memory),
        };
        process_packets_from(stream.try_clone().unwrap(), stream, h);
    }
    eprintln!("Connection closed");
}
