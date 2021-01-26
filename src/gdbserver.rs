use crate::memory::addressspace::{Address, MemoryDevice};
use crate::util;
use crate::AddressSpace;
use crate::Cpu;
use std::net::TcpListener;

use crate::cpu::CpuEvent;
use gdbstub::arch::Arch;
use gdbstub::target;
use gdbstub::target::ext::base;
use gdbstub::target::ext::base::singlethread::{SingleThreadOps, StopReason};
use gdbstub::target::ext::base::ResumeAction;
use gdbstub::target::ext::breakpoints::SwBreakpoint;
use gdbstub::target::{Target, TargetResult};
use gdbstub::{arch, DisconnectReason, GdbStub, GdbStubError};

struct RISCVTarget {
    memory: AddressSpace,
    cpu: Cpu,
}

impl SingleThreadOps for RISCVTarget {
    fn resume(
        &mut self,
        action: ResumeAction,
        check_gdb_interrupt: &mut dyn FnMut() -> bool,
    ) -> Result<StopReason<<Self::Arch as Arch>::Usize>, Self::Error> {
        let event = match action {
            ResumeAction::Step => match self.cpu.step(&mut self.memory) {
                Some(e) => e,
                None => return Ok(StopReason::DoneStep),
            },
            ResumeAction::Continue => {
                let mut cycles = 0;
                loop {
                    if let Some(event) = self.cpu.step(&mut self.memory) {
                        println!("Breka");
                        return Ok(StopReason::SwBreak);
                    };

                    // check for GDB interrupt every 1024 instructions
                    cycles += 1;
                    if cycles % 1024 == 0 && check_gdb_interrupt() {
                        return Ok(StopReason::GdbInterrupt);
                    }
                }
            }
        };

        Ok(match event {
            CpuEvent::Halted => StopReason::Halted,
            CpuEvent::Breakpoint => StopReason::SwBreak,
        })
    }

    fn read_registers(
        &mut self,
        regs: &mut <Self::Arch as Arch>::Registers,
    ) -> TargetResult<(), Self> {
        regs.pc = self.cpu.get_pc();

        for i in 0..32 {
            regs.x[i] = self.cpu.get_register(i);
        }

        Ok(())
    }

    fn write_registers(
        &mut self,
        regs: &<Self::Arch as Arch>::Registers,
    ) -> TargetResult<(), Self> {
        self.cpu.set_pc(regs.pc);

        for i in 0..32 {
            self.cpu.set_register(i, regs.x[i]);
        }

        Ok(())
    }

    fn read_addrs(
        &mut self,
        start_addr: <Self::Arch as Arch>::Usize,
        data: &mut [u8],
    ) -> TargetResult<(), Self> {
        for i in 0..data.len() {
            data[i] = self.memory.read_byte(start_addr + i as Address);
        }

        Ok(())
    }

    fn write_addrs(
        &mut self,
        start_addr: <Self::Arch as Arch>::Usize,
        data: &[u8],
    ) -> TargetResult<(), Self> {
        for i in 0..data.len() {
            self.memory.write_byte(start_addr + i as Address, data[i]);
        }

        Ok(())
    }
}

impl SwBreakpoint for RISCVTarget {
    fn add_sw_breakpoint(&mut self, addr: u32) -> TargetResult<bool, Self> {
        self.cpu.add_breakpoint(addr);
        Ok(true)
    }

    fn remove_sw_breakpoint(&mut self, addr: u32) -> TargetResult<bool, Self> {
        self.cpu.remove_breakpoint(addr);
        Ok(true)
    }
}

impl Target for RISCVTarget {
    type Arch = arch::riscv::Riscv32;
    type Error = &'static str;

    fn base_ops(&mut self) -> base::BaseOps<Self::Arch, Self::Error> {
        base::BaseOps::SingleThread(self)
    }

    fn sw_breakpoint(&mut self) -> Option<target::ext::breakpoints::SwBreakpointOps<Self>> {
        Some(self)
    }
}

pub fn start_server(cpu: Cpu, memory: AddressSpace) {
    let sockaddr = format!("localhost:{}", 3000);
    eprintln!("Waiting for a GDB connection on {:?}...", sockaddr);
    let sock = TcpListener::bind(sockaddr).unwrap();
    let (stream, addr) = sock.accept().unwrap();

    eprintln!("Debugger connected from {}", addr);

    let mut target = RISCVTarget { memory, cpu };

    let mut debugger = GdbStub::new(stream);

    match debugger.run(&mut target) {
        Ok(disconnect_reason) => match disconnect_reason {
            DisconnectReason::Disconnect => println!("GDB client disconnected."),
            DisconnectReason::TargetHalted => println!("Target halted!"),
            DisconnectReason::Kill => println!("GDB client sent a kill command!"),
        },
        // Handle any target-specific errors
        Err(GdbStubError::TargetError(e)) => {
            println!("Target raised a fatal error: {:?}", e);
            // e.g: re-enter the debugging session after "freezing" a system to
            // conduct some post-mortem debugging
            debugger.run(&mut target).unwrap();
        }
        Err(e) => panic!(e.to_string()), //return Err(e.into())
    }

    eprintln!("Connection closed");
}
