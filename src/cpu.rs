use crate::instruction::Instruction;
use crate::addressspace::AddressSpace;

pub struct Cpu<'a> {
    memory: &'a mut AddressSpace,
    registers: [u32; 32],
    pc: u32
}


impl<'a> Cpu<'a> {
    pub fn new(memory: &'a mut AddressSpace) -> Cpu<'a> {
        Self {
            memory,
            registers: [0u32; 32],
            pc: 0u32

        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::LUI(rd, imm) => {
                self.set_register(rd, imm << 12);
            },
            Instruction::AUIPC(rd, imm) => {
                let result = self.pc + imm << 12;
                self.set_register(rd, result)

            },
            Instruction::JAL(rd, imm) => {

            },
            Instruction::JALR(rd, rs1, imm) => {

            },
            Instruction::BEQ(r1, rs2, imm) => {

            },
            Instruction::BNE(r1, rs2, imm) => {

            },
            Instruction::BLT(r1, rs2, imm) => {

            },
            Instruction::BGE(r1, rs2, imm) => {

            },
            Instruction::BLTU(r1, rs2, imm) => {

            },
            Instruction::BGEU(r1, rs2, imm) => {

            },
            Instruction::LB(rd, rs1, imm) => {

            },
            Instruction::LH(rd, rs1, imm) => {

            },
            Instruction::LW(rd, rs1, imm) => {

            },
            Instruction::LBU(rd, rs1, imm) => {

            },
            Instruction::LHU(rd, rs1, imm) => {

            },
            Instruction::SB(r1, rs2, imm) => {

            },
            Instruction::SH(r1, rs2, imm) => {

            },
            Instruction::SW(r1, rs2, imm) => {

            },
            Instruction::ADDI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1) as i32;
                let result = v1.wrapping_add(imm as i32);
                self.set_register(rd, result as u32);

            },
            Instruction::SLTI(rd, rs1, imm) => {

            },
            Instruction::SLTIU(rd, rs1, imm) => {

            },
            Instruction::XORI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1);
                let result = v1 ^ imm;
                self.set_register(rd, result);
            },
            Instruction::ORI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1);
                let result = v1 | imm;
                self.set_register(rd, result);
            },
            Instruction::ANDI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1);
                let result = v1 & imm;
                self.set_register(rd, result);
            },
            Instruction::SLLI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1);
                let result = v1 << imm;
                self.set_register(rd, result);
            },
            Instruction::SRLI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1);
                let result = v1 >> imm;
                self.set_register(rd, result);
            },
            Instruction::SRAI(rd, rs1, imm) => {
                let v1 = self.get_register(rs1) as i32;
                let result = v1 >> imm as i32;
                self.set_register(rd, result as u32);
            },
            Instruction::ADD(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1) as i32;
                let v2 = self.get_register(rs2) as i32;
                let result = v1.wrapping_add(v2);
                self.set_register(rd, result as u32);
            },
            Instruction::SUB(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1) as i32;
                let v2 = self.get_register(rs2) as i32;
                let result = v1.wrapping_sub(v2);
                self.set_register(rd, result as u32);
            },
            Instruction::SLL(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = v1 << v2;
                self.set_register(rd, result);
            },
            Instruction::SLT(rd, rs1, rs2) => {

            },
            Instruction::SLTU(rd, rs1, rs2) => {

           },
            Instruction::XOR(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = v1 ^ v2;
                self.set_register(rd, result);
            },
            Instruction::SRL(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = v1 >> v2;
                self.set_register(rd, result);
            },
            Instruction::SRA(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1) as i32;
                let v2 = self.get_register(rs2) as i32;
                let result = v1 >> v2;
                self.set_register(rd, result as u32);
            },
            Instruction::OR(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = v1 | v2;
                self.set_register(rd, result as u32);
            },
            Instruction::AND(rd, rs1, rs2) => {
                let v1 = self.get_register(rs1);
                let v2 = self.get_register(rs2);
                let result = v1 & v2;
                self.set_register(rd, result);
            },
            Instruction::INVALID => {

            },
            _ => {

            }
        }

    }

    pub fn get_register(&self, num: usize) -> u32 {
        return self.registers[num];
    }

    pub fn set_register(&mut self, num: usize, value: u32) {
        self.registers[num] = value
    }
}

#[cfg(test)]
mod test {
    use super::*;



    macro_rules! immediate_test {
        ($instr:ident, $first:expr, $second:expr, $result:expr) => {
            {
                let mut memory = AddressSpace::new();
                let mut cpu = Cpu::new(&mut memory);
                cpu.set_register(2, $first);

                cpu.execute_instruction(Instruction::$instr(1, 2, $second));
                assert_eq!(cpu.get_register(1), $result);
            }
        };
    }

    macro_rules! register_test {
        ($instr:ident, $first:expr, $second:expr, $result:expr) => {
            {
                let mut memory = AddressSpace::new();
                let mut cpu = Cpu::new(&mut memory);
                cpu.set_register(2, $first);
                cpu.set_register(3, $second);

                cpu.execute_instruction(Instruction::$instr(1, 2, 3));
                assert_eq!(cpu.get_register(1), $result);
            }
        };
    }

    #[test]
    fn test_immediate() {
        immediate_test!(ADDI, 10, 20, 30);
        immediate_test!(XORI, 0b1010, 0b0110, 0b1100);
        immediate_test!(ORI, 0b1010, 0b0110, 0b1110);
        immediate_test!(ANDI, 0b1010, 0b0110, 0b0010);
    }

    #[test]
    fn test_register_ops() {
        register_test!(ADD, 10, 20, 30);
        register_test!(SUB, 10, 20, -10i32 as u32);
        register_test!(XOR, 0b1010, 0b0110, 0b1100);
        register_test!(OR, 0b1010, 0b0110, 0b1110);
        register_test!(AND, 0b1010, 0b0110, 0b0010);
        register_test!(SLL, 0b1010, 2, 0b101000);
        register_test!(SRL, 0b1010, 2, 0b10);
        register_test!(SRA, 0b1000_0000_0000_0000_0000_0000_0000_0010, 1, 0b1100_0000_0000_0000_0000_0000_0000_0001);

    }

    #[test]
    fn test_register() {
        let mut memory = AddressSpace::new();
        let mut cpu = Cpu::new(&mut memory);

        cpu.set_register(0, 0xCAFEBABE);
        assert_eq!(cpu.get_register(0), 0xCAFEBABE);
    }
}
