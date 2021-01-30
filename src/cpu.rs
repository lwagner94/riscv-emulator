use crate::instruction::Instruction;
use crate::instruction::WrappedInstruction;
use crate::memory::addressspace::{Address, AddressSpace, MemoryDevice};
use crate::util;
use core::cmp::max;
use core::cmp::min;

#[cfg(feature = "debugger")]
use std::collections::HashSet;

#[allow(dead_code)]
pub enum CpuEvent {
    Halted,
    Breakpoint,
}

pub struct Cpu {
    registers: [u32; 32],
    pc: u32,
    running: bool,
    cycle_counter: u64,
    saved_pc: u32,
    #[cfg(feature = "debugger")]
    breakpoints: HashSet<Address>,
}

impl Cpu {
    pub fn new() -> Cpu {
        Self {
            registers: [0u32; 32],
            pc: 0u32,
            running: true,
            cycle_counter: 0,
            saved_pc: 0u32,
            #[cfg(feature = "debugger")]
            breakpoints: HashSet::new(),
        }
    }

    #[allow(dead_code)]
    pub fn reset(&mut self) {
        *self = Cpu::new();
    }

    pub fn run(&mut self, memory: &mut AddressSpace) -> Option<CpuEvent> {
        let mut instruction_cache: Vec<WrappedInstruction> = vec![
            WrappedInstruction {
                instruction: Instruction::INVALID,
                size: 0
            };
            0x10_0000 // 1 Megabyte
        ];

        while self.running {
            if let Some(new_pc) = memory.check_for_interrupt() {
                self.saved_pc = self.pc;
                self.pc = new_pc;
            }

            if instruction_cache[self.pc as usize].instruction == Instruction::INVALID {
                instruction_cache[self.pc as usize] =
                    WrappedInstruction::new(memory.read_word(self.pc));
            }

            let wrapped_instruction = &instruction_cache[self.pc as usize];

            let instruction = &wrapped_instruction.instruction;
            let size = wrapped_instruction.size;

            self.execute_instruction(instruction, size, memory);
            self.pc += size;
            self.cycle_counter += 1;
        }

        Some(CpuEvent::Halted)
    }

    #[cfg(feature = "debugger")]
    pub fn step(&mut self, memory: &mut AddressSpace) -> Option<CpuEvent> {
        let wrapped_instruction = WrappedInstruction::new(memory.read_word(self.pc));

        let instruction = &wrapped_instruction.instruction;
        let size = wrapped_instruction.size;

        self.execute_instruction(instruction, size, memory);
        self.pc += size;
        self.cycle_counter += 1;

        let breakpoint_hit = self.is_breakpoint(self.pc);

        if !self.running {
            Some(CpuEvent::Halted)
        } else if breakpoint_hit {
            Some(CpuEvent::Breakpoint)
        } else {
            None
        }
    }

    fn set_pc_for_branch(&mut self, condition: bool, imm: u32, size: u32) {
        if condition {
            let mut new_pc = self.pc as i32;
            new_pc = new_pc.wrapping_add((imm as i32) * 2);
            new_pc -= size as i32;
            self.pc = new_pc as u32;
        }
    }

    fn calculate_address(&self, base_reg: usize, offset: i32) -> u32 {
        (self.get_register(base_reg) as i32).wrapping_add(offset) as u32
    }

    #[inline(always)]
    pub fn execute_instruction(
        &mut self,
        instruction: &Instruction,
        size: u32,
        memory: &mut AddressSpace,
    ) {
        // println!("{:x}, {:?}", self.pc, instruction);

        match *instruction {
            Instruction::LUI(rd, imm) => {
                self.set_register(rd, imm << 12);
            }
            Instruction::AUIPC(rd, imm) => {
                let result = self.pc.wrapping_add(imm << 12);
                self.set_register(rd, result)
            }
            Instruction::JAL(rd, imm) => {
                let result = self.pc + size;

                let mut new_pc = self.pc as i32;
                new_pc = new_pc.wrapping_add((imm as i32) * 2);

                self.pc = (new_pc - size as i32) as u32;
                self.set_register(rd, result);
            }
            Instruction::JALR(rd, rs1, imm) => {
                let mut new_pc = self.get_register(rs1) as i32;
                new_pc = new_pc.wrapping_add(imm as i32);
                let result = self.pc + size;
                self.pc = ((new_pc as u32) & !1u32) - size;
                self.set_register(rd, result);
            }
            Instruction::BEQ(rs1, rs2, imm) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                self.set_pc_for_branch(v1 == v2, imm, size);
            }
            Instruction::BNE(rs1, rs2, imm) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                self.set_pc_for_branch(v1 != v2, imm, size);
            }
            Instruction::BLT(rs1, rs2, imm) => {
                let v1 = self.get_register(rs1) as i32;
                let v2 = self.get_register(rs2) as i32;
                self.set_pc_for_branch(v1 < v2, imm, size);
            }
            Instruction::BGE(rs1, rs2, imm) => {
                let v1 = self.get_register(rs1) as i32;
                let v2 = self.get_register(rs2) as i32;
                self.set_pc_for_branch(v1 >= v2, imm, size);
            }
            Instruction::BLTU(rs1, rs2, imm) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                self.set_pc_for_branch(v1 < v2, imm, size);
            }
            Instruction::BGEU(rs1, rs2, imm) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                self.set_pc_for_branch(v1 >= v2, imm, size);
            }
            Instruction::LB(rd, rs1, imm) => {
                let addr = self.calculate_address(rs1, imm);
                let byte = memory.read_byte(addr);
                self.set_register(rd, util::sign_extend(i32::from(byte), 8) as u32)
            }
            Instruction::LH(rd, rs1, imm) => {
                let addr = self.calculate_address(rs1, imm);
                let halfword = memory.read_halfword(addr);
                self.set_register(rd, util::sign_extend(i32::from(halfword), 16) as u32)
            }
            Instruction::LW(rd, rs1, imm) => {
                let addr = self.calculate_address(rs1, imm);
                let word = memory.read_word(addr);
                self.set_register(rd, word)
            }
            Instruction::LBU(rd, rs1, imm) => {
                let addr = self.calculate_address(rs1, imm);
                let byte = memory.read_byte(addr);
                self.set_register(rd, u32::from(byte))
            }
            Instruction::LHU(rd, rs1, imm) => {
                let addr = self.calculate_address(rs1, imm);
                let halfword = memory.read_halfword(addr);
                self.set_register(rd, u32::from(halfword))
            }
            Instruction::SB(rs1, rs2, imm) => {
                let addr = self.calculate_address(rs1, imm);
                memory.write_byte(addr, self.get_register(rs2) as u8)
            }
            Instruction::SH(rs1, rs2, imm) => {
                let addr = self.calculate_address(rs1, imm);
                memory.write_halfword(addr, self.get_register(rs2) as u16)
            }
            Instruction::SW(rs1, rs2, imm) => {
                let addr = self.calculate_address(rs1, imm);
                memory.write_word(addr, self.get_register(rs2) as u32)
            }
            Instruction::ADDI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1) as i32;
                let result = v1.wrapping_add(imm as i32);
                self.set_register(rd, result as u32);
            }
            Instruction::SLTI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1) as i32;
                let result = if v1 < (imm as i32) { 1 } else { 0 };
                self.set_register(rd, result);
            }
            Instruction::SLTIU(rd, rs1, imm) => {
                let v1 = self.get_register(rs1);
                let result = if v1 < imm { 1 } else { 0 };
                self.set_register(rd, result);
            }
            Instruction::XORI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1);
                let result = v1 ^ imm;
                self.set_register(rd, result);
            }
            Instruction::ORI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1);
                let result = v1 | imm;
                self.set_register(rd, result);
            }
            Instruction::ANDI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1);
                let result = v1 & imm;
                self.set_register(rd, result);
            }
            Instruction::SLLI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1);
                let result = v1 << imm;
                self.set_register(rd, result);
            }
            Instruction::SRLI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1);
                let result = v1 >> imm;
                self.set_register(rd, result);
            }
            Instruction::SRAI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1) as i32;
                let result = v1 >> imm as i32;
                self.set_register(rd, result as u32);
            }
            Instruction::ADD(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1) as i32;
                let v2 = self.get_register(rs2) as i32;
                let result = v1.wrapping_add(v2);
                self.set_register(rd, result as u32);
            }
            Instruction::SUB(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1) as i32;
                let v2 = self.get_register(rs2) as i32;
                let result = v1.wrapping_sub(v2);
                self.set_register(rd, result as u32);
            }
            Instruction::SLL(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = v1 << v2;
                self.set_register(rd, result);
            }
            Instruction::SLT(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1) as i32;
                let v2 = self.get_register(rs2) as i32;
                let result = if v1 < v2 { 1 } else { 0 };
                self.set_register(rd, result as u32);
            }
            Instruction::SLTU(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = if v1 < v2 { 1 } else { 0 };
                self.set_register(rd, result);
            }
            Instruction::XOR(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = v1 ^ v2;
                self.set_register(rd, result);
            }
            Instruction::SRL(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = v1 >> v2;
                self.set_register(rd, result);
            }
            Instruction::SRA(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1) as i32;
                let v2 = self.get_register(rs2) as i32;
                let result = v1 >> v2;
                self.set_register(rd, result as u32);
            }
            Instruction::OR(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = v1 | v2;
                self.set_register(rd, result as u32);
            }
            Instruction::AND(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = v1 & v2;
                self.set_register(rd, result);
            }
            Instruction::MUL(rd, rs1, rs2) => {
                let v1 = u64::from(self.get_register(rs1));
                let v2 = u64::from(self.get_register(rs2));
                let result = v1 * v2;
                self.set_register(rd, result as u32);
            }
            Instruction::MULH(rd, rs1, rs2) => {
                let v1 = i64::from(self.get_register(rs1) as i32);
                let v2 = i64::from(self.get_register(rs2) as i32);
                let result = v1 * v2;
                self.set_register(rd, (result >> 32) as u32);
            }
            Instruction::MULHSU(rd, rs1, rs2) => {
                let v1 = i64::from(self.get_register(rs1) as i32);
                let v2 = i64::from(self.get_register(rs2));
                let result = v1 * v2;
                self.set_register(rd, (result >> 32) as u32);
            }
            Instruction::MULHU(rd, rs1, rs2) => {
                let v1 = u64::from(self.get_register(rs1));
                let v2 = u64::from(self.get_register(rs2));
                let result = v1 * v2;
                self.set_register(rd, (result >> 32) as u32);
            }
            Instruction::DIV(rd, rs1, rs2) => {
                let v1 = i64::from(self.get_register(rs1) as i32);
                let v2 = i64::from(self.get_register(rs2) as i32);
                let result = if v2 == 0 { -1 } else { v1 / v2 };
                self.set_register(rd, result as u32);
            }
            Instruction::DIVU(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = if v2 == 0 { -1i32 as u32 } else { v1 / v2 };
                self.set_register(rd, result);
            }
            Instruction::REM(rd, rs1, rs2) => {
                let v1 = i64::from(self.get_register(rs1) as i32);
                let v2 = i64::from(self.get_register(rs2) as i32);
                let result = if v2 == 0 { v1 } else { v1 % v2 };
                self.set_register(rd, result as u32);
            }
            Instruction::REMU(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = if v2 == 0 { v1 } else { v1 % v2 };
                self.set_register(rd, result);
            }
            Instruction::EBREAK => {
                self.running = false;
            }
            Instruction::MRET => {
                self.pc = self.saved_pc - size;
            }
            Instruction::LRW(rd, rs1, _) => {
                // TODO: 64bit: Sign-Extension ?!?
                let addr = self.get_register(rs1) as Address;
                let v = memory.read_word(addr);
                self.set_register(rd, v);
            }
            Instruction::SCW(rd, rs1, rs2) => {
                let word = self.get_register(rs2);
                let addr = self.get_register(rs1) as Address;
                memory.write_word(addr, word);
                self.set_register(rd, 0); // Always succeed for now!
            }
            Instruction::AMOSWAPW(rd, rs1, rs2) => {
                let addr = self.get_register(rs1) as Address;
                let op1 = memory.read_word(addr);
                self.set_register(rd, op1);

                let op2 = self.get_register(rs2);
                memory.write_word(addr, op2);
            }
            Instruction::AMOADDW(rd, rs1, rs2) => {
                let addr = self.get_register(rs1) as Address;
                let op1 = memory.read_word(addr);
                let op2 = self.get_register(rs2);
                let result = op1.wrapping_add(op2);
                memory.write_word(addr, result);
                self.set_register(rd, op1);
            }
            Instruction::AMOANDW(rd, rs1, rs2) => {
                let addr = self.get_register(rs1) as Address;
                let op1 = memory.read_word(addr);
                let op2 = self.get_register(rs2);
                let result = op1 & op2;
                memory.write_word(addr, result);
                self.set_register(rd, op1);
            }
            Instruction::AMOORW(rd, rs1, rs2) => {
                let addr = self.get_register(rs1) as Address;
                let op1 = memory.read_word(addr);
                let op2 = self.get_register(rs2);
                let result = op1 | op2;
                memory.write_word(addr, result);
                self.set_register(rd, op1);
            }
            Instruction::AMOXORW(rd, rs1, rs2) => {
                let addr = self.get_register(rs1) as Address;
                let op1 = memory.read_word(addr);
                let op2 = self.get_register(rs2);
                let result = op1 ^ op2;
                memory.write_word(addr, result);
                self.set_register(rd, op1);
            }
            Instruction::AMOMAXW(rd, rs1, rs2) => {
                let addr = self.get_register(rs1) as Address;
                let op1 = memory.read_word(addr);
                let op2 = self.get_register(rs2);
                let result = max(op1 as i32, op2 as i32) as u32;
                memory.write_word(addr, result);
                self.set_register(rd, op1);
            }
            Instruction::AMOMAXUW(rd, rs1, rs2) => {
                let addr = self.get_register(rs1) as Address;
                let op1 = memory.read_word(addr);
                let op2 = self.get_register(rs2);
                let result = max(op1, op2);
                memory.write_word(addr, result);
                self.set_register(rd, op1);
            }
            Instruction::AMOMINW(rd, rs1, rs2) => {
                let addr = self.get_register(rs1) as Address;
                let op1 = memory.read_word(addr);
                let op2 = self.get_register(rs2);
                let result = min(op1 as i32, op2 as i32) as u32;
                memory.write_word(addr, result);
                self.set_register(rd, op1);
            }
            Instruction::AMOMINUW(rd, rs1, rs2) => {
                let addr = self.get_register(rs1) as Address;
                let op1 = memory.read_word(addr);
                let op2 = self.get_register(rs2);
                let result = min(op1, op2);
                memory.write_word(addr, result);
                self.set_register(rd, op1);
            }
            Instruction::CSRRW => {
                println!("Unimplemented instruction CSRRW at pc=0x{:x}", self.pc);
            }
            Instruction::CSRRS => {
                println!("Unimplemented instruction CSRRS at pc=0x{:x}", self.pc);
            }
            Instruction::CSRRC => {
                println!("Unimplemented instruction CSRRC at pc=0x{:x}", self.pc);
            }
            Instruction::CSRRWI => {
                println!("Unimplemented instruction CSRRWI at pc=0x{:x}", self.pc);
            }
            Instruction::CSRRSI => {
                println!("Unimplemented instruction CSRRSI at pc=0x{:x}", self.pc);
            }
            Instruction::CSRRCI => {
                println!("Unimplemented instruction CSRRCI at pc=0x{:x}", self.pc);
            }
            Instruction::INVALID => panic!("Invalid Instruction at pc=0x{:x} detected", self.pc),
        }
    }

    #[inline(always)]
    pub fn get_register(&self, num: usize) -> u32 {
        self.registers[num]
    }

    #[inline(always)]
    pub fn set_register(&mut self, num: usize, value: u32) {
        if num != 0 {
            self.registers[num] = value
        }
    }

    #[allow(dead_code)]
    pub fn get_pc(&self) -> u32 {
        self.pc
    }

    #[allow(dead_code)]
    pub fn set_pc(&mut self, value: u32) {
        self.pc = value;
    }

    pub fn get_cycle_counter(&self) -> u64 {
        self.cycle_counter
    }

    #[cfg(feature = "debugger")]
    pub fn add_breakpoint(&mut self, address: Address) {
        self.breakpoints.insert(address);
    }

    #[cfg(feature = "debugger")]
    pub fn remove_breakpoint(&mut self, address: Address) {
        self.breakpoints.remove(&address);
    }

    #[cfg(feature = "debugger")]
    fn is_breakpoint(&self, address: Address) -> bool {
        self.breakpoints.contains(&address)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! immediate_test {
        ($instr:ident, $first:expr, $second:expr, $result:expr) => {{
            let mut memory = AddressSpace::new();
            let mut cpu = Cpu::new();
            cpu.set_register(2, $first as u32);

            cpu.execute_instruction(&Instruction::$instr(1, 2, $second as u32), 4, &mut memory);
            assert_eq!(cpu.get_register(1), $result);
        }};
    }

    macro_rules! register_test {
        ($instr:ident, $first:expr, $second:expr, $result:expr) => {{
            let mut memory = AddressSpace::new();
            let mut cpu = Cpu::new();
            cpu.set_register(2, $first as u32);
            cpu.set_register(3, $second as u32);

            cpu.execute_instruction(&Instruction::$instr(1, 2, 3), 4, &mut memory);
            assert_eq!(cpu.get_register(1), $result as u32);
        }};
    }

    macro_rules! register_test_new {
        ($instr:ident, $result:expr, $first:expr, $second:expr) => {{
            let mut memory = AddressSpace::new();
            let mut cpu = Cpu::new();
            cpu.set_register(2, $first as u32);
            cpu.set_register(3, $second as u32);

            cpu.execute_instruction(&Instruction::$instr(1, 2, 3), 4, &mut memory);
            assert_eq!(cpu.get_register(1), $result as u32);
        }};
    }

    #[test]
    fn test_immediate() {
        immediate_test!(ADDI, 10, 20, 30);
        immediate_test!(XORI, 0b1010, 0b0110, 0b1100);
        immediate_test!(ORI, 0b1010, 0b0110, 0b1110);
        immediate_test!(ANDI, 0b1010, 0b0110, 0b0010);

        immediate_test!(SLTI, 0, 0, 0);
        immediate_test!(SLTI, 0, -1i32, 0);
        immediate_test!(SLTI, -1i32, 0, 1);

        immediate_test!(SLTIU, 0, 0, 0);
        immediate_test!(SLTIU, -1i32, 0, 0);
        immediate_test!(SLTIU, 0, 1, 1);
    }

    #[test]
    fn test_register_ops() {
        register_test!(ADD, 10, 20, 30);
        register_test!(SUB, 10, 20, -10i32);
        register_test!(XOR, 0b1010, 0b0110, 0b1100);
        register_test!(OR, 0b1010, 0b0110, 0b1110);
        register_test!(AND, 0b1010, 0b0110, 0b0010);
        register_test!(SLL, 0b1010, 2, 0b101000);
        register_test!(SRL, 0b1010, 2, 0b10);
        register_test!(
            SRA,
            0b1000_0000_0000_0000_0000_0000_0000_0010,
            1,
            0b1100_0000_0000_0000_0000_0000_0000_0001
        );

        register_test!(SLT, 0, 0, 0);
        register_test!(SLT, 0, -1i32, 0);
        register_test!(SLT, -1i32, 0, 1);

        register_test!(SLTU, 0, 0, 0);
        register_test!(SLTU, -1i32, 0, 0);
        register_test!(SLTU, 0, 1, 1);
    }

    #[test]
    fn test_mul() {
        register_test_new!(MUL, 0x00001200, 0x00007e00, 0xb6db6db7);
        register_test_new!(MUL, 0x00001240, 0x00007fc0, 0xb6db6db7);
        register_test_new!(MUL, 0x00000000, 0x00000000, 0x00000000);
        register_test_new!(MUL, 0x00000001, 0x00000001, 0x00000001);
        register_test_new!(MUL, 0x00000015, 0x00000003, 0x00000007);
        register_test_new!(MUL, 0x00000000, 0x00000000, 0xffff8000);
        register_test_new!(MUL, 0x00000000, 0x80000000, 0x00000000);
        register_test_new!(MUL, 0x00000000, 0x80000000, 0xffff8000);
        register_test_new!(MUL, 0x0000ff7f, 0xaaaaaaab, 0x0002fe7d);
        register_test_new!(MUL, 0x0000ff7f, 0x0002fe7d, 0xaaaaaaab);
        register_test_new!(MUL, 0x00000000, 0xff000000, 0xff000000);
        register_test_new!(MUL, 0x00000001, 0xffffffff, 0xffffffff);
        register_test_new!(MUL, 0xffffffff, 0xffffffff, 0x00000001);
        register_test_new!(MUL, 0xffffffff, 0x00000001, 0xffffffff);
    }

    #[test]
    fn test_mulh() {
        register_test_new!(MULH, 0x00000000, 0x00000000, 0x00000000);
        register_test_new!(MULH, 0x00000000, 0x00000001, 0x00000001);
        register_test_new!(MULH, 0x00000000, 0x00000003, 0x00000007);
        register_test_new!(MULH, 0x00000000, 0x00000000, 0xffff8000);
        register_test_new!(MULH, 0x00000000, 0x80000000, 0x00000000);
        register_test_new!(MULH, 0x00000000, 0x80000000, 0x00000000);
        register_test_new!(MULH, 0xffff0081, 0xaaaaaaab, 0x0002fe7d);
        register_test_new!(MULH, 0xffff0081, 0x0002fe7d, 0xaaaaaaab);
        register_test_new!(MULH, 0x00010000, 0xff000000, 0xff000000);
        register_test_new!(MULH, 0x00000000, 0xffffffff, 0xffffffff);
        register_test_new!(MULH, 0xffffffff, 0xffffffff, 0x00000001);
        register_test_new!(MULH, 0xffffffff, 0x00000001, 0xffffffff);
    }

    #[test]
    fn test_mulhu() {
        register_test_new!(MULHU, 0x00000000, 0x00000000, 0x00000000);
        register_test_new!(MULHU, 0x00000000, 0x00000001, 0x00000001);
        register_test_new!(MULHU, 0x00000000, 0x00000003, 0x00000007);
        register_test_new!(MULHU, 0x00000000, 0x00000000, 0xffff8000);
        register_test_new!(MULHU, 0x00000000, 0x80000000, 0x00000000);
        register_test_new!(MULHU, 0x7fffc000, 0x80000000, 0xffff8000);
        register_test_new!(MULHU, 0x0001fefe, 0xaaaaaaab, 0x0002fe7d);
        register_test_new!(MULHU, 0x0001fefe, 0x0002fe7d, 0xaaaaaaab);
        register_test_new!(MULHU, 0xfe010000, 0xff000000, 0xff000000);
        register_test_new!(MULHU, 0xfffffffe, 0xffffffff, 0xffffffff);
        register_test_new!(MULHU, 0x00000000, 0xffffffff, 0x00000001);
        register_test_new!(MULHU, 0x00000000, 0x00000001, 0xffffffff);
    }

    #[test]
    fn test_mulhsu() {
        register_test_new!(MULHSU, 0x00000000, 0x00000000, 0x00000000);
        register_test_new!(MULHSU, 0x00000000, 0x00000001, 0x00000001);
        register_test_new!(MULHSU, 0x00000000, 0x00000003, 0x00000007);
        register_test_new!(MULHSU, 0x00000000, 0x00000000, 0xffff8000);
        register_test_new!(MULHSU, 0x00000000, 0x80000000, 0x00000000);
        register_test_new!(MULHSU, 0x80004000, 0x80000000, 0xffff8000);
        register_test_new!(MULHSU, 0xffff0081, 0xaaaaaaab, 0x0002fe7d);
        register_test_new!(MULHSU, 0x0001fefe, 0x0002fe7d, 0xaaaaaaab);
        register_test_new!(MULHSU, 0xff010000, 0xff000000, 0xff000000);
        register_test_new!(MULHSU, 0xffffffff, 0xffffffff, 0xffffffff);
        register_test_new!(MULHSU, 0xffffffff, 0xffffffff, 0x00000001);
        register_test_new!(MULHSU, 0x00000000, 0x00000001, 0xffffffff);
    }

    #[test]
    fn test_div() {
        register_test_new!(DIV, 3, 20, 6);
        register_test_new!(DIV, -3i32, -20i32, 6);
        register_test_new!(DIV, -3i32, 20, -6i32);
        register_test_new!(DIV, 3, -20i32, -6i32);
        register_test_new!(DIV, -1i32 << 31, -1i32 << 31, 1);
        register_test_new!(DIV, -1i32 << 31, -1i32 << 31, -1i32);
        register_test_new!(DIV, -1i32, -1i32 << 31, 0);
        register_test_new!(DIV, -1i32, 1, 0);
        register_test_new!(DIV, -1i32, 0, 0);
    }

    #[test]
    fn test_divu() {
        register_test_new!(DIVU, 3, 20, 6);
        register_test_new!(DIVU, 715827879, -20i32, 6);
        register_test_new!(DIVU, 0, 20, -6i32);
        register_test_new!(DIVU, 0, -20i32, -6i32);
        register_test_new!(DIVU, -1i32 << 31, -1i32 << 31, 1);
        register_test_new!(DIVU, 0, -1i32 << 31, -1i32);
        register_test_new!(DIVU, -1i32, -1 << 31, 0);
        register_test_new!(DIVU, -1i32, 1, 0);
        register_test_new!(DIVU, -1i32, 0, 0);
    }

    #[test]
    fn test_rem() {
        register_test_new!(REM, 2, 20, 6);
        register_test_new!(REM, -2i32, -20i32, 6);
        register_test_new!(REM, 2, 20, -6i32);
        register_test_new!(REM, -2i32, -20i32, -6i32);
        register_test_new!(REM, 0, -1i32 << 31, 1);
        register_test_new!(REM, 0, -1i32 << 31, -1i32);
        register_test_new!(REM, -1i32 << 31, -1i32 << 31, 0);
        register_test_new!(REM, 1, 1, 0);
        register_test_new!(REM, 0, 0, 0);
    }

    #[test]
    fn test_remu() {
        register_test_new!(REMU, 2, 20, 6);
        register_test_new!(REMU, 2, -20i32, 6);
        register_test_new!(REMU, 20, 20, -6i32);
        register_test_new!(REMU, -20i32, -20i32, -6i32);
        register_test_new!(REMU, 0, -1i32 << 31, 1);
        register_test_new!(REMU, -1i32 << 31, -1i32 << 31, -1i32);
        register_test_new!(REMU, -1i32 << 31, -1i32 << 31, 0);
        register_test_new!(REMU, 1, 1, 0);
        register_test_new!(REMU, 0, 0, 0);
    }

    #[test]
    fn test_jal() {
        fn t(offset: i32) {
            let mut memory = AddressSpace::new();
            let mut cpu = Cpu::new();
            cpu.pc = 80;

            cpu.execute_instruction(&Instruction::JAL(1, offset as u32), 4, &mut memory);
            assert_eq!(cpu.get_register(1), 84);
            assert_eq!(cpu.pc, (80 + offset * 2 - 4) as u32);
        }

        t(16);
        t(-16);
    }

    #[test]
    fn test_jalr() {
        fn t(base: u32, offset: i32) {
            let mut memory = AddressSpace::new();
            let mut cpu = Cpu::new();
            cpu.set_register(2, base);
            cpu.pc = 80;

            cpu.execute_instruction(&Instruction::JALR(1, 2, offset as i32), 4, &mut memory);
            assert_eq!(cpu.get_register(1), 84);
            assert_eq!(cpu.pc, ((base as i32 + offset - 4) as u32) & !1u32);
        }

        t(400, 4);
        t(400, -4);
        t(400, 1);
        t(400, -1);
    }

    macro_rules! branch_test {
        ($instr:ident, $first:expr, $second:expr, $offset:expr, $expect_jump:expr) => {{
            let first = $first as u32;
            let second = $second as u32;

            let mut memory = AddressSpace::new();
            let mut cpu = Cpu::new();
            cpu.set_register(2, first);
            cpu.set_register(3, second);
            cpu.pc = 80;

            cpu.execute_instruction(&Instruction::$instr(2, 3, $offset), 4, &mut memory);

            if $expect_jump {
                assert_eq!((80i32).wrapping_add(2 * $offset) as u32 - 4, cpu.pc);
            } else {
                assert_eq!(cpu.pc, 80);
            }
        }};
    }

    #[test]
    fn test_branching() {
        branch_test!(BEQ, 10, 10, 8, true);
        branch_test!(BEQ, -10i32, -10i32, 8, true);
        branch_test!(BEQ, 10, 9, 8, false);
        branch_test!(BEQ, -10i32, -9i32, 8, false);

        branch_test!(BNE, 10, 10, 8, false);
        branch_test!(BNE, -10i32, -10i32, 8, false);
        branch_test!(BNE, 10, 9, 8, true);
        branch_test!(BNE, -10i32, -9i32 as u32, 8, true);

        branch_test!(BLT, 10, 10, 8, false);
        branch_test!(BLT, 11, 10, 8, false);
        branch_test!(BLT, 9, 10, 8, true);
        branch_test!(BLT, -9i32, 10, 8, true);

        branch_test!(BGE, 10, 10, 8, true);
        branch_test!(BGE, 11, 10, 8, true);
        branch_test!(BGE, 9, 10, 8, false);
        branch_test!(BGE, -9i32, 10, 8, false);

        branch_test!(BLTU, -10i32, 10, 8, false);
        branch_test!(BLTU, 9, 10, 8, true);
        branch_test!(BLTU, 10, 10, 8, false);
        branch_test!(BLTU, -10i32, -9i32, 8, true);

        branch_test!(BGEU, -10i32, 10, 8, true);
        branch_test!(BGEU, 9, 10, 8, false);
        branch_test!(BGEU, 10, 10, 8, true);
        branch_test!(BGEU, -10i32, -9i32, 8, false);
    }

    #[test]
    fn test_byte_store() {
        let mut memory = AddressSpace::new();
        let mut cpu = Cpu::new();

        cpu.set_register(1, 0xF0);
        cpu.set_register(2, 0xCAFEBABE);
        cpu.execute_instruction(&Instruction::SB(1, 2, 16), 4, &mut memory);
        assert_eq!(0, memory.read_byte(0xF0 + 16 - 1));
        assert_eq!(0xBE, memory.read_byte(0xF0 + 16));
        assert_eq!(0, memory.read_byte(0xF0 + 16 + 1));
    }

    #[test]
    fn test_halfword_store() {
        let mut memory = AddressSpace::new();
        let mut cpu = Cpu::new();

        cpu.set_register(1, 0xF0);
        cpu.set_register(2, 0xCAFEBABE);
        cpu.execute_instruction(&Instruction::SH(1, 2, 16), 4, &mut memory);
        assert_eq!(0, memory.read_byte(0xF0 + 16 - 1));
        assert_eq!(0xBABE, memory.read_halfword(0xF0 + 16));
        assert_eq!(0, memory.read_byte(0xF0 + 16 + 3));
    }

    #[test]
    fn test_word_store() {
        let mut memory = AddressSpace::new();
        let mut cpu = Cpu::new();

        cpu.set_register(1, 0xF0);
        cpu.set_register(2, 0xCAFEBABE);
        cpu.execute_instruction(&Instruction::SW(1, 2, 16), 4, &mut memory);
        assert_eq!(0, memory.read_byte(0xF0 + 16 - 1));
        assert_eq!(0xCAFEBABE, memory.read_word(0xF0 + 16));
        assert_eq!(0, memory.read_byte(0xF0 + 16 + 5));
    }

    macro_rules! load_test {
        ($instr:ident,  $memop:ident, $value:expr, $expected:expr) => {{
            let mut memory = AddressSpace::new();
            memory.$memop(0xF0 + 16, $value);
            let mut cpu = Cpu::new();

            cpu.set_register(2, 0xF0);
            cpu.execute_instruction(&Instruction::$instr(1, 2, 16), 4, &mut memory);
            assert_eq!(cpu.get_register(1), $expected);
        }};
    }

    #[test]
    fn test_loads() {
        load_test!(LB, write_byte, 0b1111_1111, !0);
        load_test!(LB, write_byte, 0b0111_1111, 0b0111_1111);
        load_test!(LBU, write_byte, 0b1111_1111, 0b1111_1111);

        load_test!(LH, write_halfword, 0b1111_1111_1111_1111, !0);
        load_test!(
            LH,
            write_halfword,
            0b0111_1111_1111_1111,
            0b0111_1111_1111_1111
        );
        load_test!(
            LHU,
            write_halfword,
            0b1111_1111_1111_1111,
            0b1111_1111_1111_1111
        );

        load_test!(LW, write_word, 0xCAFEBABE, 0xCAFEBABE);
    }

    #[test]
    fn test_register() {
        let mut cpu = Cpu::new();

        cpu.set_register(1, 0xCAFEBABE);
        assert_eq!(cpu.get_register(1), 0xCAFEBABE);
        assert_eq!(cpu.get_register(0), 0);
        cpu.set_register(0, 0xCAFEBABE);
        assert_eq!(cpu.get_register(0), 0);
    }
}
