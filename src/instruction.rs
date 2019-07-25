
const OPCODE_MASK: u32 = 0b111_1111;
const REGISTER_MASK: u32 = 0b0111;

#[derive(PartialEq, Debug)]
pub enum Instruction {
    ADD(usize, usize, usize),
    SUB(usize, usize, usize),
    SLL(usize, usize, usize),
    SLT(usize, usize, usize),
    SLTU(usize, usize, usize),
    XOR(usize, usize, usize),
    SRL(usize, usize, usize),
    SRA(usize, usize, usize),
    OR(usize, usize, usize),
    AND(usize, usize, usize),
    INVALID
}

impl Instruction {

    pub fn new(code: u32) -> Self {

        let opcode = code & OPCODE_MASK;

        match opcode {
            0b011_0011 => Instruction::match_arithmetic(code),
            _ => Instruction::INVALID
        }
    }

    fn match_arithmetic(code: u32) -> Self {
        let rd = shift_and_mask(code, 7, REGISTER_MASK);
        let funct3 = shift_and_mask(code, 12, 0b111);
        let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
        let rs2 = shift_and_mask(code, 20, REGISTER_MASK);
        let funct7 = shift_and_mask(code, 25, 0b0111_1111);

        match funct3 {
            0b000 => {
                if funct7 == 0 {
                    Instruction::ADD(rd, rs1, rs2)
                }
                else {
                    Instruction::SUB(rd, rs1, rs2)
                }
            },
            0b001 => Instruction::SLL(rd, rs1, rs2),
            0b010 => Instruction::SLT(rd, rs1, rs2),
            0b011 => Instruction::SLTU(rd, rs1, rs2),
            0b100 => Instruction::XOR(rd, rs1, rs2),
            0b101 => {
                if funct7 == 0 {
                    Instruction::SRL(rd, rs1, rs2)
                }
                else {
                    Instruction::SRA(rd, rs1, rs2)
                }
            },
            0b110 => Instruction::OR(rd, rs1, rs2),
            0b111 => Instruction::AND(rd, rs1, rs2),
            _ => Instruction::INVALID
        }
    }

}

fn shift_and_mask(code: u32, shift: u32, mask: u32) -> usize {
    ((code >> shift) & mask) as usize
}


#[cfg(test)]
mod test {
    mod arithmetic_tests {
        use super::super::*;

        #[test]
        fn test_add() {
            assert_eq!(Instruction::new(0b000000_00011_00010_000_00001_0110011),
                       Instruction::ADD(1, 2, 3));
        }

        #[test]
        fn test_sub() {
            assert_eq!(Instruction::new(0b100000_00011_00010_000_00001_0110011),
                       Instruction::SUB(1, 2, 3));
        }

        #[test]
        fn test_sll() {
            assert_eq!(Instruction::new(0b000000_00011_00010_001_00001_0110011),
                       Instruction::SLL(1, 2, 3));
        }

        #[test]
        fn test_slt() {
            assert_eq!(Instruction::new(0b000000_00011_00010_010_00001_0110011),
                       Instruction::SLT(1, 2, 3));
        }

        #[test]
        fn test_sltu() {
            assert_eq!(Instruction::new(0b000000_00011_00010_011_00001_0110011),
                       Instruction::SLTU(1, 2, 3));
        }

        #[test]
        fn test_xor() {
            assert_eq!(Instruction::new(0b000000_00011_00010_100_00001_0110011),
                       Instruction::XOR(1, 2, 3));
        }

        #[test]
        fn test_srl() {
            assert_eq!(Instruction::new(0b000000_00011_00010_101_00001_0110011),
                       Instruction::SRL(1, 2, 3));
        }

        #[test]
        fn test_sra() {
            assert_eq!(Instruction::new(0b100000_00011_00010_101_00001_0110011),
                       Instruction::SRA(1, 2, 3));
        }

        #[test]
        fn test_or() {
            assert_eq!(Instruction::new(0b000000_00011_00010_110_00001_0110011),
                       Instruction::OR(1, 2, 3));
        }

        #[test]
        fn test_and() {
            assert_eq!(Instruction::new(0b000000_00011_00010_111_00001_0110011),
                       Instruction::AND(1, 2, 3));
        }
    }

}