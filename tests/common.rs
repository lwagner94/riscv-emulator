use riscv_emu::cpu::Cpu;
use riscv_emu::loader;
use riscv_emu::memory::addressspace::{Address, AddressSpace, MemoryDevice};

// TODO: Extract constants!
const DEBUG_BASE: Address = 0x20000000;

const DEBUG_BASE_OUTPUT_LENGTH: Address = DEBUG_BASE + 1024;
const DEBUG_BASE_OUTPUT: Address = DEBUG_BASE_OUTPUT_LENGTH + 4;
const DEBUG_BASE_INPUT_LENGTH: Address = DEBUG_BASE + 2 * 1024;
const DEBUG_BASE_INPUT: Address = DEBUG_BASE_INPUT_LENGTH + 4;

pub struct TestRun {
    memory: AddressSpace,
    write_address: Address,
}

pub struct TestRunResult {
    memory: AddressSpace,
    read_address: Address,
}

impl TestRun {
    pub fn new(path: &str) -> Self {
        let mut memory = AddressSpace::new();
        loader::load_program(path, &mut memory).unwrap();

        Self {
            memory,
            write_address: DEBUG_BASE_INPUT,
        }
    }

    pub fn write_byte(&mut self, value: u8) -> &mut Self {
        self.memory.write_byte(self.write_address, value);
        self.write_address += 1;
        self
    }

    pub fn write_halfword(&mut self, value: u16) -> &mut Self {
        self.memory.write_halfword(self.write_address, value);
        self.write_address += 2;
        self
    }

    pub fn write_word(&mut self, value: u32) -> &mut Self {
        self.memory.write_word(self.write_address, value);
        self.write_address += 4;
        self
    }

    pub fn write_string(&mut self, s: &str) -> &mut Self {
        // let mut ret = self;

        for character in s.chars() {
            self.write_byte(character as u8);
        }

        self.write_byte(0u8);

        self
    }

    pub fn run(mut self) -> TestRunResult {
        self.memory.write_word(
            DEBUG_BASE_INPUT_LENGTH,
            self.write_address - DEBUG_BASE_INPUT,
        );
        let mut cpu = Cpu::new();
        cpu.run(&mut self.memory);

        TestRunResult {
            memory: self.memory,
            read_address: DEBUG_BASE_OUTPUT,
        }
    }
}

impl TestRunResult {
    pub fn read_byte(&mut self) -> u8 {
        let result = self.memory.read_byte(self.read_address);
        self.read_address += 1;
        result
    }

    pub fn read_halfword(&mut self) -> u16 {
        let result = self.memory.read_halfword(self.read_address);
        self.read_address += 2;
        result
    }

    pub fn read_word(&mut self) -> u32 {
        let result = self.memory.read_word(self.read_address);
        self.read_address += 4;
        result
    }

    pub fn read_string(&mut self) -> String {
        let mut s = String::new();

        loop {
            let c = self.read_byte();
            if c == 0 {
                break;
            }

            s.push(char::from(c));
        }

        s
    }
}
