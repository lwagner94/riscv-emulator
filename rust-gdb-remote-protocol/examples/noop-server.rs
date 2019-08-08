extern crate env_logger;
extern crate gdb_remote_protocol;

use gdb_remote_protocol::{Error,Handler,ProcessType,process_packets_from,StopReason, MemoryRegion};
use std::net::TcpListener;
use std::cell::RefCell;

struct NoopHandler {
    pc: RefCell<u8>
}

impl Handler for NoopHandler {
    fn attached(&self, _pid: Option<u64>) -> Result<ProcessType, Error> {
        Ok(ProcessType::Created)
    }

    fn halt_reason(&self) -> Result<StopReason, Error> {
        Ok(StopReason::Signal(5))
    }

    fn read_memory(&self, _region: MemoryRegion) -> Result<Vec<u8>, Error> {
        println!("Foobar");
        Ok(vec![10; 1])
    }

    fn read_general_registers(&self) -> Result<Vec<u8>, Error> {
        let mut v = vec![0; 32*4];
        v.push(*self.pc.borrow());
        v.push(0);
        v.push(0);
        v.push(0);

        Ok(v)
    }

    fn step(&self) -> Result<StopReason, Error> {
        *self.pc.borrow_mut() += 4;
        Ok(StopReason::Signal(5))
    }

    fn cont(&self) -> Result<StopReason, Error> {
        Ok(StopReason::Signal(5))
    }
}

#[cfg_attr(test, allow(dead_code))]
fn main() {
    drop(env_logger::init());
    let listener = TcpListener::bind("0.0.0.0:2424").unwrap();
    println!("Listening on port 2424");
    for res in listener.incoming() {
        println!("Got connection");
        if let Ok(stream) = res {
            let h = NoopHandler {
                pc: RefCell::new(0x24)
            };
            process_packets_from(stream.try_clone().unwrap(), stream, h);
        }
        println!("Connection closed");
    }
}
