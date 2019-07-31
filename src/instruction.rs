use crate::util::sign_extend;


const OPCODE_MASK: u32 = 0b111_1111;
const REGISTER_MASK: u32 = 0b1_1111;
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
    LB(usize, usize, i32),
    LH(usize, usize, i32),
    LW(usize, usize, i32),
    LBU(usize, usize, i32),
    LHU(usize, usize, i32),
    SB(usize, usize, i32),
    SH(usize, usize, i32),
    SW(usize, usize, i32),
    ADDI(usize, usize, u32),
    SLTI(usize, usize, u32),
    SLTIU(usize, usize, u32),
    XORI(usize, usize, u32),
    ORI(usize, usize, u32),
    ANDI(usize, usize, u32),
    SLLI(usize, usize, u32),
    SRLI(usize, usize, u32),
    SRAI(usize, usize, u32),
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
    EBREAK,
    INVALID,
}

use Instruction::*;

impl Instruction {
    pub fn new(code: u32) -> Self {
        let opcode = code & OPCODE_MASK;

        match opcode {
            0b011_0111 => {
                let rd = shift_and_mask(code, 7, REGISTER_MASK);
                let immediate = shift_and_mask(code, 12, IMMEDIATE_20_MASK) as u32;
                LUI(rd, immediate)
            }
            0b001_0111 => {
                let rd = shift_and_mask(code, 7, REGISTER_MASK);
                let immediate = shift_and_mask(code, 12, IMMEDIATE_20_MASK) as u32;
                AUIPC(rd, immediate)
            }
            0b110_1111 => {
                let rd = shift_and_mask(code, 7, REGISTER_MASK);

                let imm_0_to_10 = shift_and_mask(code, 21, 0b11_1111_1111) as u32;
                let imm_12_to_19 = shift_and_mask(code, 12, 0b1111_1111) as u32;
                let imm_11 = shift_and_mask(code, 20, 0b1) as u32;
                let imm_20 = shift_and_mask(code, 31, 0b1) as u32;

                let mut immediate = imm_20 << 19;
                immediate |= imm_11 << 10;
                immediate |= imm_0_to_10;
                immediate |= imm_12_to_19 << 11;

                JAL(rd, immediate)
            }
            0b110_0111 => {
                let rd = shift_and_mask(code, 7, REGISTER_MASK);
                let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
                let immediate = shift_and_mask(code, 20, IMMEDIATE_12_MASK) as u32;
                let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);

                if funct3 == 0 {
                    JALR(rd, rs1, immediate)
                } else {
                    INVALID
                }
            }
            0b110_0011 => Instruction::match_branch(code),
            0b000_0011 => Instruction::match_load(code),
            0b010_0011 => Instruction::match_store(code),
            0b001_0011 => Instruction::match_arithmetic_immediate(code),
            0b011_0011 => Instruction::match_arithmetic(code),
            0b111_0011 => {
                let rd = shift_and_mask(code, 7, REGISTER_MASK);
                let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);
                let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
                let imm12 = shift_and_mask(code, 20, IMMEDIATE_12_MASK);

                if rd == 0 && funct3 == 0 && rs1 == 0 && imm12 == 1 {
                    EBREAK
                } else {
                    INVALID
                }


            }
            _ => INVALID,
        }
    }

    fn match_branch(code: u32) -> Self {
        let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);
        let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
        let rs2 = shift_and_mask(code, 20, REGISTER_MASK);

        let imm_1_to_4 = shift_and_mask(code, 8, 0b1111) as u32;
        let imm_5_to_11 = shift_and_mask(code, 25, 0b11_1111) as u32;
        let imm_11 = shift_and_mask(code, 7, 0b1) as u32;
        let imm_12 = shift_and_mask(code, 31, 0b1) as u32;

        let mut immediate = imm_1_to_4;
        immediate |= imm_5_to_11 << 4;
        immediate |= imm_11 << 10;
        immediate |= imm_12 << 11;

        match funct3 {
            0b000 => BEQ(rs1, rs2, immediate),
            0b001 => BNE(rs1, rs2, immediate),
            0b100 => BLT(rs1, rs2, immediate),
            0b101 => BGE(rs1, rs2, immediate),
            0b110 => BLTU(rs1, rs2, immediate),
            0b111 => BGEU(rs1, rs2, immediate),
            _ => INVALID,
        }
    }

    fn match_load(code: u32) -> Self {
        let rd = shift_and_mask(code, 7, REGISTER_MASK);
        let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);
        let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
        let immediate = shift_and_mask(code, 20, IMMEDIATE_12_MASK) as i32;
        let immediate_sign_extended = sign_extend(immediate, 12);

        match funct3 {
            0b000 => LB(rd, rs1, immediate_sign_extended),
            0b001 => LH(rd, rs1, immediate_sign_extended),
            0b010 => LW(rd, rs1, immediate_sign_extended),
            0b100 => LBU(rd, rs1, immediate_sign_extended),
            0b101 => LHU(rd, rs1, immediate_sign_extended),
            _ => INVALID,
        }
    }

    fn match_store(code: u32) -> Self {
        let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
        let rs2 = shift_and_mask(code, 20, REGISTER_MASK);
        let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);

        let imm_1_to_5 = shift_and_mask(code, 7, 0b1_1111) as u32;
        let imm_6_to_12 = shift_and_mask(code, 25, 0b111_1111) as u32;

        let mut immediate = imm_1_to_5;
        immediate |= imm_6_to_12 << 5;

        let immediate_sign_extended = sign_extend(immediate as i32, 12);

        match funct3 {
            0b000 => SB(rs1, rs2, immediate_sign_extended),
            0b001 => SH(rs1, rs2, immediate_sign_extended),
            0b010 => SW(rs1, rs2, immediate_sign_extended),
            _ => INVALID,
        }
    }

    fn match_arithmetic_immediate(code: u32) -> Self {
        let rd = shift_and_mask(code, 7, REGISTER_MASK);
        let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);
        let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
        let immediate = shift_and_mask(code, 20, IMMEDIATE_12_MASK) as u32;
        let immediate_sign_extended = sign_extend(immediate as i32, 12) as u32;

        let shift_amount = immediate & 0b1_1111;
        let funct7 = shift_and_mask(code, 25, FUNCT7_MASK);

        match funct3 {
            0b000 => ADDI(rd, rs1, immediate_sign_extended),
            0b010 => SLTI(rd, rs1, immediate_sign_extended),
            0b011 => SLTIU(rd, rs1, immediate_sign_extended),
            0b100 => XORI(rd, rs1, immediate_sign_extended),
            0b110 => ORI(rd, rs1, immediate_sign_extended),
            0b111 => ANDI(rd, rs1, immediate_sign_extended),
            0b001 => SLLI(rd, rs1, shift_amount),
            0b101 => {
                if funct7 == 0b010_0000 {
                    SRAI(rd, rs1, shift_amount)
                } else if funct7 == 0 {
                    SRLI(rd, rs1, shift_amount)
                } else {
                    INVALID
                }
            }
            _ => INVALID,
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
                if funct7 == 0b010_0000 {
                    SUB(rd, rs1, rs2)
                } else if funct7 == 0 {
                    ADD(rd, rs1, rs2)
                } else {
                    INVALID
                }
            }
            0b001 => SLL(rd, rs1, rs2),
            0b010 => SLT(rd, rs1, rs2),
            0b011 => SLTU(rd, rs1, rs2),
            0b100 => XOR(rd, rs1, rs2),
            0b101 => {
                if funct7 == 0b010_0000 {
                    SRA(rd, rs1, rs2)
                } else if funct7 == 0 {
                    SRL(rd, rs1, rs2)
                } else {
                    INVALID
                }
            }
            0b110 => OR(rd, rs1, rs2),
            0b111 => AND(rd, rs1, rs2),
            _ => INVALID,
        }
    }
}

fn shift_and_mask(code: u32, shift: u32, mask: u32) -> usize {
    ((code >> shift) & mask) as usize
}



#[cfg(test)]
mod test {
    mod load_immediate {
        use super::super::*;

        #[test]
        fn test_lui() {
            assert_eq!(
                Instruction::new(0b11111111111111111111_00001_0110111),
                Instruction::LUI(1, 0xfffff)
            );
        }

        #[test]
        fn test_auipc() {
            assert_eq!(
                Instruction::new(0b11111111111111111111_00001_0010111),
                Instruction::AUIPC(1, 0xfffff)
            );
        }
    }

    mod jump {
        use super::super::*;

        #[test]
        fn test_jal() {
            assert_eq!(
                Instruction::new(0b01110100000100001010_00001_1101111),
                Instruction::JAL(1, 0xaf40 / 2)
            );
        }

        #[test]
        fn test_jalr() {
            assert_eq!(
                Instruction::new(0b00010000000000010000_00001_1100111),
                Instruction::JALR(1, 2, 0x100)
            );
        }

    }

    mod branch {
        use super::super::*;

        #[test]
        fn test_beq() {
            assert_eq!(
                Instruction::new(0b0111111_00010_00001_000_11101_1100011),
                Instruction::BEQ(1, 2, 0xffc / 2)
            );
        }

        #[test]
        fn test_bne() {
            assert_eq!(
                Instruction::new(0b0111111_00010_00001_001_11101_1100011),
                Instruction::BNE(1, 2, 0xffc / 2)
            );
        }

        #[test]
        fn test_blt() {
            assert_eq!(
                Instruction::new(0b0111111_00010_00001_100_11101_1100011),
                Instruction::BLT(1, 2, 0xffc / 2)
            );
        }

        #[test]
        fn test_bge() {
            assert_eq!(
                Instruction::new(0b0111111_00010_00001_101_11101_1100011),
                Instruction::BGE(1, 2, 0xffc / 2)
            );
        }

        #[test]
        fn test_bltu() {
            assert_eq!(
                Instruction::new(0b0111111_00010_00001_110_11101_1100011),
                Instruction::BLTU(1, 2, 0xffc / 2)
            );
        }

        #[test]
        fn test_bgeu() {
            assert_eq!(
                Instruction::new(0b0111111_00010_00001_111_11101_1100011),
                Instruction::BGEU(1, 2, 0xffc / 2)
            );
        }

    }

    mod load {
        use super::super::*;

        #[test]
        fn test_lb() {
            assert_eq!(
                Instruction::new(0b100000000000_00010_000_00001_0000011),
                Instruction::LB(1, 2, -2048)
            );
        }

        #[test]
        fn test_lh() {
            assert_eq!(
                Instruction::new(0b100000000000_00010_001_00001_0000011),
                Instruction::LH(1, 2, -2048)
            );
        }

        #[test]
        fn test_lw() {
            assert_eq!(
                Instruction::new(0b100000000000_00010_010_00001_0000011),
                Instruction::LW(1, 2, -2048)
            );
        }

        #[test]
        fn test_lbu() {
            assert_eq!(
                Instruction::new(0b100000000000_00010_100_00001_0000011),
                Instruction::LBU(1, 2, -2048)
            );
        }

        #[test]
        fn test_lhu() {
            assert_eq!(
                Instruction::new(0b100000000000_00010_101_00001_0000011),
                Instruction::LHU(1, 2, -2048)
            );
        }
    }

    mod store {
        use super::super::*;

        #[test]
        fn test_sb() {
            assert_eq!(
                Instruction::new(0b1000000_00010_00001_000_00000_0100011),
                Instruction::SB(1, 2, -2048)
            );
        }

        #[test]
        fn test_sh() {
            assert_eq!(
                Instruction::new(0b1000000_00010_00001_001_00000_0100011),
                Instruction::SH(1, 2, -2048)
            );
        }

        #[test]
        fn test_sw() {
            assert_eq!(
                Instruction::new(0b1000000_00010_00001_010_00000_0100011),
                Instruction::SW(1, 2, -2048)
            );
        }

    }

    mod arithmetic_immediate {
        use super::super::*;

        #[test]
        fn test_addi() {
            assert_eq!(
                Instruction::new(0b1000000_00000_00010_000_00001_0010011),
                Instruction::ADDI(1, 2, (-2048i32 as u32))
            );
        }

        #[test]
        fn test_slti() {
            assert_eq!(
                Instruction::new(0b1000000_00000_00010_010_00001_0010011),
                Instruction::SLTI(1, 2, (-2048i32 as u32))
            );
        }

        #[test]
        fn test_sltiu() {
            assert_eq!(
                Instruction::new(0b1000000_00000_00010_011_00001_0010011),
                Instruction::SLTIU(1, 2, (-2048i32 as u32))
            );
        }

        #[test]
        fn test_andi() {
            assert_eq!(
                Instruction::new(0b1000000_00000_00010_100_00001_0010011),
                Instruction::XORI(1, 2, (-2048i32 as u32))
            );
        }

        #[test]
        fn test_ori() {
            assert_eq!(
                Instruction::new(0b1000000_00000_00010_110_00001_0010011),
                Instruction::ORI(1, 2, (-2048i32 as u32))
            );
        }

        #[test]
        fn test_xori() {
            assert_eq!(
                Instruction::new(0b1000000_00000_00010_111_00001_0010011),
                Instruction::ANDI(1, 2, (-2048i32 as u32))
            );
        }

        #[test]
        fn test_slli() {
            assert_eq!(
                Instruction::new(0b0000000_11111_00010_001_00001_0010011),
                Instruction::SLLI(1, 2, 31)
            );
        }

        #[test]
        fn test_srli() {
            assert_eq!(
                Instruction::new(0b0000000_11111_00010_101_00001_0010011),
                Instruction::SRLI(1, 2, 31)
            );
        }

        #[test]
        fn test_srai() {
            assert_eq!(
                Instruction::new(0b0100000_11111_00010_101_00001_0010011),
                Instruction::SRAI(1, 2, 31)
            );
        }

    }

    mod arithmetic_register {
        use super::super::*;

        #[test]
        fn test_add() {
            assert_eq!(
                Instruction::new(0b0000000_00011_00010_000_00001_0110011),
                Instruction::ADD(1, 2, 3)
            );
        }

        #[test]
        fn test_sub() {
            assert_eq!(
                Instruction::new(0b0100000_00011_00010_000_00001_0110011),
                Instruction::SUB(1, 2, 3)
            );
        }

        #[test]
        fn test_sll() {
            assert_eq!(
                Instruction::new(0b0000000_00011_00010_001_00001_0110011),
                Instruction::SLL(1, 2, 3)
            );
        }

        #[test]
        fn test_slt() {
            assert_eq!(
                Instruction::new(0b0000000_00011_00010_010_00001_0110011),
                Instruction::SLT(1, 2, 3)
            );
        }

        #[test]
        fn test_sltu() {
            assert_eq!(
                Instruction::new(0b0000000_00011_00010_011_00001_0110011),
                Instruction::SLTU(1, 2, 3)
            );
        }

        #[test]
        fn test_xor() {
            assert_eq!(
                Instruction::new(0b0000000_00011_00010_100_00001_0110011),
                Instruction::XOR(1, 2, 3)
            );
        }

        #[test]
        fn test_srl() {
            assert_eq!(
                Instruction::new(0b0000000_00011_00010_101_00001_0110011),
                Instruction::SRL(1, 2, 3)
            );
        }

        #[test]
        fn test_sra() {
            assert_eq!(
                Instruction::new(0b0100000_00011_00010_101_00001_0110011),
                Instruction::SRA(1, 2, 3)
            );
        }

        #[test]
        fn test_or() {
            assert_eq!(
                Instruction::new(0b0000000_00011_00010_110_00001_0110011),
                Instruction::OR(1, 2, 3)
            );
        }

        #[test]
        fn test_and() {
            assert_eq!(
                Instruction::new(0b0000000_00011_00010_111_00001_0110011),
                Instruction::AND(1, 2, 3)
            );
        }
    }

    mod other {
        use super::super::*;

        #[test]
        fn test_ebreak() {
            assert_eq!(
                Instruction::new(0x00100073),
                Instruction::EBREAK
            );
        }
    }
}
