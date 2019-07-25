
const OPCODE_MASK: u32 = 0b111_1111;
const REGISTER_MASK: u32 = 0b111;
const FUNCT7_MASK: u32 = 0b111_1111;
const FUNCT3_MASK: u32 = 0b111;
const IMMEDIATE_20_MASK: u32 = 0b1111_1111_1111_1111_1111;
const IMMEDIATE_12_MASK: u32 = 0b1111_1111_1111;

#[derive(PartialEq, Debug)]
pub enum Instruction {
    LUI(usize, u32),
    AUIPC(usize, u32),
    JAL(usize, u32),
    JALR(usize, usize, u32),
    BEQ(usize, usize, u32),
    BNE(usize, usize, u32),
    BLT(usize, usize, u32),
    BGE(usize, usize, u32),
    BLTU(usize, usize, u32),
    BGEU(usize, usize, u32),
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

use Instruction::*;

impl Instruction {

    pub fn new(code: u32) -> Self {

        let opcode = code & OPCODE_MASK;

        match opcode {
            0b0110111 => {
                let rd = shift_and_mask(code, 7, REGISTER_MASK);
                let immediate = shift_and_mask(code, 12, IMMEDIATE_20_MASK) as u32;
                LUI(rd, immediate)
            },
            0b0010111 => {
                let rd = shift_and_mask(code, 7, REGISTER_MASK);
                let immediate = shift_and_mask(code, 12, IMMEDIATE_20_MASK) as u32;
                AUIPC(rd, immediate)
            },
            0b1101111 => {
                let rd = shift_and_mask(code, 7, REGISTER_MASK);

                let imm_0_to_10 = shift_and_mask(code, 21, 0b11_1111_1111) as u32;
                let imm_12_to_19 = shift_and_mask(code, 12, 0b1111_1111) as u32;
                let imm_11 = shift_and_mask(code, 20, 0b1) as u32;
                let imm_20 = shift_and_mask(code, 31, 0b1) as u32;

                let mut immediate = (imm_20 << 19);
                immediate |= (imm_11 << 10);
                immediate |= (imm_0_to_10);
                immediate |= (imm_12_to_19 << 11);

                JAL(rd, immediate)
            },
            0b1100111 => {
                let rd = shift_and_mask(code, 7, REGISTER_MASK);
                let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
                let immediate = shift_and_mask(code, 20, IMMEDIATE_12_MASK) as u32;
                let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);

                if funct3 == 0 {
                    JALR(rd, rs1, immediate)
                }
                else {
                    INVALID
                }

            },
            0b110_0011 => Instruction::match_branch(code),
            0b011_0011 => Instruction::match_arithmetic(code),
            _ => INVALID
        }
    }

    fn match_arithmetic(code: u32) -> Self {
        let rd = shift_and_mask(code, 7, REGISTER_MASK);
        let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);
        let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
        let rs2 = shift_and_mask(code, 20, REGISTER_MASK);
        let funct7 = shift_and_mask(code, 25, FUNCT7_MASK);

        match funct3 {
            0b000 => {
                if funct7 == 0b100000 {
                    SUB(rd, rs1, rs2)
                }
                else if funct7 == 0{
                    ADD(rd, rs1, rs2)
                }
                else {
                    INVALID
                }
            },
            0b001 => SLL(rd, rs1, rs2),
            0b010 => SLT(rd, rs1, rs2),
            0b011 => SLTU(rd, rs1, rs2),
            0b100 => XOR(rd, rs1, rs2),
            0b101 => {
                if funct7 == 0b100000 {
                    SRA(rd, rs1, rs2)
                }
                else if funct7 == 0 {
                    SRL(rd, rs1, rs2)
                }
                else {
                    INVALID
                }
            },
            0b110 => OR(rd, rs1, rs2),
            0b111 => AND(rd, rs1, rs2),
            _ => INVALID
        }
    }

    fn match_branch(code: u32) -> Self {
        let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);
        let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
        let rs2 = shift_and_mask(code, 20, REGISTER_MASK);

        let imm_1_to_4 = shift_and_mask(code, 8, 0b1111) as u32;
        let imm_5_to_11 = shift_and_mask(code, 25, 0b111111) as u32;
        let imm_11 = shift_and_mask(code, 7, 0b1) as u32;
        let imm_12 = shift_and_mask(code, 31, 0b1) as u32;

        let mut immediate = imm_1_to_4;
        immediate |= (imm_5_to_11 << 4);
        immediate |= (imm_11 << 10);
        immediate |= (imm_12 << 11);

        match funct3 {
            0b000 => BEQ(rs1, rs2, immediate),
            0b001 => BNE(rs1, rs2, immediate),
            0b100 => BLT(rs1, rs2, immediate),
            0b101 => BGE(rs1, rs2, immediate),
            0b110 => BLTU(rs1, rs2, immediate),
            0b111 => BGEU(rs1, rs2, immediate),
            _ => INVALID
        }
    }

}

fn shift_and_mask(code: u32, shift: u32, mask: u32) -> usize {
    ((code >> shift) & mask) as usize
}


#[cfg(test)]
mod test {
    mod u_type {
        use super::super::*;

        #[test]
        fn test_lui() {
            assert_eq!(Instruction::new(0b11111111111111111111_00001_0110111),
                       Instruction::LUI(1, 0xfffff));
        }

        #[test]
        fn test_auipc() {
            assert_eq!(Instruction::new(0b11111111111111111111_00001_0010111),
                       Instruction::AUIPC(1, 0xfffff));
        }

        #[test]
        fn test_jal() {
            assert_eq!(Instruction::new(0b01110100000100001010_00001_1101111),
                       Instruction::JAL(1, 0xaf40 / 2));
        }

    }



    mod j_type {
        use super::super::*;

        #[test]
        fn test_jal() {
            assert_eq!(Instruction::new(0b01110100000100001010_00001_1101111),
                       Instruction::JAL(1, 0xaf40 / 2));
        }

        #[test]
        fn test_jalr() {
            assert_eq!(Instruction::new(0b00010000000000010000_00001_1100111),
                       Instruction::JALR(1, 2, 0x100));
        }

    }

    mod branch_tests {
        use super::super::*;

        #[test]
        fn test_beq() {
            assert_eq!(Instruction::new(0b0111111_00010_00001_000_11101_1100011),
                       Instruction::BEQ(1, 2, 0xffc / 2));
        }

        #[test]
        fn test_bne() {
            assert_eq!(Instruction::new(0b0111111_00010_00001_001_11101_1100011),
                       Instruction::BNE(1, 2, 0xffc / 2));
        }

        #[test]
        fn test_blt() {
            assert_eq!(Instruction::new(0b0111111_00010_00001_100_11101_1100011),
                       Instruction::BLT(1, 2, 0xffc / 2));
        }

        #[test]
        fn test_bge() {
            assert_eq!(Instruction::new(0b0111111_00010_00001_101_11101_1100011),
                       Instruction::BGE(1, 2, 0xffc / 2));
        }

        #[test]
        fn test_bltu() {
            assert_eq!(Instruction::new(0b0111111_00010_00001_110_11101_1100011),
                       Instruction::BLTU(1, 2, 0xffc / 2));
        }

        #[test]
        fn test_bgeu() {
            assert_eq!(Instruction::new(0b0111111_00010_00001_111_11101_1100011),
                       Instruction::BGEU(1, 2, 0xffc / 2));
        }

    }



    mod arithmetic_tests {
        use super::super::*;

        #[test]
        fn test_add() {
            assert_eq!(Instruction::new(0b0000000_00011_00010_000_00001_0110011),
                       Instruction::ADD(1, 2, 3));
        }

        #[test]
        fn test_sub() {
            assert_eq!(Instruction::new(0b0100000_00011_00010_000_00001_0110011),
                       Instruction::SUB(1, 2, 3));
        }

        #[test]
        fn test_sll() {
            assert_eq!(Instruction::new(0b0000000_00011_00010_001_00001_0110011),
                       Instruction::SLL(1, 2, 3));
        }

        #[test]
        fn test_slt() {
            assert_eq!(Instruction::new(0b0000000_00011_00010_010_00001_0110011),
                       Instruction::SLT(1, 2, 3));
        }

        #[test]
        fn test_sltu() {
            assert_eq!(Instruction::new(0b0000000_00011_00010_011_00001_0110011),
                       Instruction::SLTU(1, 2, 3));
        }

        #[test]
        fn test_xor() {
            assert_eq!(Instruction::new(0b0000000_00011_00010_100_00001_0110011),
                       Instruction::XOR(1, 2, 3));
        }

        #[test]
        fn test_srl() {
            assert_eq!(Instruction::new(0b0000000_00011_00010_101_00001_0110011),
                       Instruction::SRL(1, 2, 3));
        }

        #[test]
        fn test_sra() {
            assert_eq!(Instruction::new(0b0100000_00011_00010_101_00001_0110011),
                       Instruction::SRA(1, 2, 3));
        }

        #[test]
        fn test_or() {
            assert_eq!(Instruction::new(0b0000000_00011_00010_110_00001_0110011),
                       Instruction::OR(1, 2, 3));
        }

        #[test]
        fn test_and() {
            assert_eq!(Instruction::new(0b0000000_00011_00010_111_00001_0110011),
                       Instruction::AND(1, 2, 3));
        }
    }

}