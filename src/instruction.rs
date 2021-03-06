use crate::util::sign_extend;

const OPCODE_MASK: u32 = 0b111_1111;
const REGISTER_MASK: u32 = 0b1_1111;
const FUNCT7_MASK: u32 = 0b111_1111;
const FUNCT3_MASK: u32 = 0b111;
const IMMEDIATE_20_MASK: u32 = 0b1111_1111_1111_1111_1111;
const IMMEDIATE_12_MASK: u32 = 0b1111_1111_1111;

#[derive(PartialEq, Debug, Clone)]
pub enum Instruction {
    LUI(usize, u32),
    AUIPC(usize, u32),
    JAL(usize, u32),
    JALR(usize, usize, i32),
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
    MUL(usize, usize, usize),
    MULH(usize, usize, usize),
    MULHSU(usize, usize, usize),
    MULHU(usize, usize, usize),
    DIV(usize, usize, usize),
    DIVU(usize, usize, usize),
    REM(usize, usize, usize),
    REMU(usize, usize, usize),
    LRW(usize, usize, usize),
    SCW(usize, usize, usize),
    AMOSWAPW(usize, usize, usize),
    AMOADDW(usize, usize, usize),
    AMOXORW(usize, usize, usize),
    AMOANDW(usize, usize, usize),
    AMOORW(usize, usize, usize),
    AMOMINW(usize, usize, usize),
    AMOMAXW(usize, usize, usize),
    AMOMINUW(usize, usize, usize),
    AMOMAXUW(usize, usize, usize),
    MRET,

    // Zicsr
    CSRRW,
    CSRRS,
    CSRRC,
    CSRRWI,
    CSRRSI,
    CSRRCI,

    INVALID,
}

#[derive(PartialEq, Debug, Clone)]
pub struct WrappedInstruction {
    pub instruction: Instruction,
    pub size: u32,
}

impl WrappedInstruction {
    pub fn new(code: u32) -> Self {
        match code & 0b11 {
            0b11 => WrappedInstruction {
                instruction: Instruction::new(code),
                size: 4,
            },
            _ => WrappedInstruction {
                instruction: Instruction::new_compressed(code as u16),
                size: 2,
            },
        }
    }
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

                immediate = sign_extend(immediate as i32, 20) as u32;

                JAL(rd, immediate)
            }
            0b110_0111 => {
                let rd = shift_and_mask(code, 7, REGISTER_MASK);
                let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
                let immediate = shift_and_mask(code, 20, IMMEDIATE_12_MASK) as i32;
                let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);

                if funct3 == 0 {
                    JALR(rd, rs1, sign_extend(immediate, 12))
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
                let _rd = shift_and_mask(code, 7, REGISTER_MASK);
                let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);
                let _rs1 = shift_and_mask(code, 15, REGISTER_MASK);
                let imm12 = shift_and_mask(code, 20, IMMEDIATE_12_MASK);

                match funct3 {
                    0b000 => match imm12 {
                        0b1 => EBREAK,
                        0b0011_0000_0010 => MRET,
                        _ => INVALID,
                    },
                    // TODO: Arg decode!
                    0b001 => CSRRW,
                    0b010 => CSRRS,
                    0b011 => CSRRC,
                    0b101 => CSRRWI,
                    0b110 => CSRRSI,
                    0b111 => CSRRCI,
                    _ => INVALID,
                }
            }
            0b010_1111 => Instruction::match_atomic(code),
            _ => INVALID,
        }
    }

    pub fn new_compressed(_code: u16) -> Self {
        INVALID
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

        immediate = sign_extend(immediate as i32, 12) as u32;

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

        match (funct3, funct7) {
            (0b000, 0b010_0000) => SUB(rd, rs1, rs2),
            (0b000, 0b000_0000) => ADD(rd, rs1, rs2),
            (0b000, 0b000_0001) => MUL(rd, rs1, rs2),
            (0b001, 0b000_0000) => SLL(rd, rs1, rs2),
            (0b001, 0b000_0001) => MULH(rd, rs1, rs2),
            (0b010, 0b000_0000) => SLT(rd, rs1, rs2),
            (0b010, 0b000_0001) => MULHSU(rd, rs1, rs2),
            (0b011, 0b000_0000) => SLTU(rd, rs1, rs2),
            (0b011, 0b000_0001) => MULHU(rd, rs1, rs2),
            (0b100, 0b000_0000) => XOR(rd, rs1, rs2),
            (0b100, 0b000_0001) => DIV(rd, rs1, rs2),
            (0b101, 0b010_0000) => SRA(rd, rs1, rs2),
            (0b101, 0b000_0000) => SRL(rd, rs1, rs2),
            (0b101, 0b000_0001) => DIVU(rd, rs1, rs2),
            (0b110, 0b000_0000) => OR(rd, rs1, rs2),
            (0b110, 0b000_0001) => REM(rd, rs1, rs2),
            (0b111, 0b000_0000) => AND(rd, rs1, rs2),
            (0b111, 0b000_0001) => REMU(rd, rs1, rs2),
            _ => INVALID,
        }
    }

    fn match_atomic(code: u32) -> Self {
        let rd = shift_and_mask(code, 7, REGISTER_MASK);
        let funct3 = shift_and_mask(code, 12, FUNCT3_MASK);
        let rs1 = shift_and_mask(code, 15, REGISTER_MASK);
        let rs2 = shift_and_mask(code, 20, REGISTER_MASK);
        let funct5 = shift_and_mask(code, 27, 0b1_1111);

        if funct3 != 0b010 {
            return INVALID;
        }

        match funct5 {
            0b0_0010 if rs2 == 0 => LRW(rd, rs1, 0),
            0b0_0011 => SCW(rd, rs1, rs2),
            0b0_0001 => AMOSWAPW(rd, rs1, rs2),
            0b0_0000 => AMOADDW(rd, rs1, rs2),
            0b0_0100 => AMOXORW(rd, rs1, rs2),
            0b0_1100 => AMOANDW(rd, rs1, rs2),
            0b0_1000 => AMOORW(rd, rs1, rs2),
            0b1_0000 => AMOMINW(rd, rs1, rs2),
            0b1_0100 => AMOMAXW(rd, rs1, rs2),
            0b1_1000 => AMOMINUW(rd, rs1, rs2),
            0b1_1100 => AMOMAXUW(rd, rs1, rs2),
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

        macro_rules! branch_test {
            ($instr:ident,  $code:expr, $expected:expr) => {{
                assert_eq!(
                    Instruction::new($code),
                    Instruction::$instr(1, 2, $expected)
                );
            }};
        }

        #[test]
        fn test_branch() {
            branch_test!(BEQ, 0b0111111_00010_00001_000_11101_1100011, 0xffc / 2);
            branch_test!(BNE, 0b0111111_00010_00001_001_11101_1100011, 0xffc / 2);
            branch_test!(BLT, 0b0111111_00010_00001_100_11101_1100011, 0xffc / 2);
            branch_test!(BGE, 0b0111111_00010_00001_101_11101_1100011, 0xffc / 2);
            branch_test!(BLTU, 0b0111111_00010_00001_110_11101_1100011, 0xffc / 2);
            branch_test!(BGEU, 0b0111111_00010_00001_111_11101_1100011, 0xffc / 2);
        }
    }

    mod load {
        use super::super::*;

        macro_rules! load_test {
            ($instr:ident,  $code:expr, $expected:expr) => {{
                assert_eq!(
                    Instruction::new($code),
                    Instruction::$instr(1, 2, $expected)
                );
            }};
        }

        #[test]
        fn test_load() {
            load_test!(LB, 0b100000000000_00010_000_00001_0000011, -2048i32);
            load_test!(LH, 0b100000000000_00010_001_00001_0000011, -2048i32);
            load_test!(LW, 0b100000000000_00010_010_00001_0000011, -2048i32);
            load_test!(LBU, 0b100000000000_00010_100_00001_0000011, -2048i32);
            load_test!(LHU, 0b100000000000_00010_101_00001_0000011, -2048i32);
        }
    }

    mod store {
        use super::super::*;

        macro_rules! store_test {
            ($instr:ident,  $code:expr, $expected:expr) => {{
                assert_eq!(
                    Instruction::new($code),
                    Instruction::$instr(1, 2, $expected)
                );
            }};
        }

        #[test]
        fn test_store() {
            store_test!(SB, 0b1000000_00010_00001_000_00000_0100011, -2048i32);
            store_test!(SH, 0b1000000_00010_00001_001_00000_0100011, -2048i32);
            store_test!(SW, 0b1000000_00010_00001_010_00000_0100011, -2048i32);
        }
    }

    mod arithmetic_immediate {
        use super::super::*;

        macro_rules! immediate_test {
            ($instr:ident,  $code:expr, $expected:expr) => {{
                assert_eq!(
                    Instruction::new($code),
                    Instruction::$instr(1, 2, $expected as u32)
                );
            }};
        }

        #[test]
        fn test_immediate() {
            immediate_test!(ADDI, 0b1000000_00000_00010_000_00001_0010011, -2048i32);
            immediate_test!(SLTI, 0b1000000_00000_00010_010_00001_0010011, -2048i32);
            immediate_test!(SLTIU, 0b1000000_00000_00010_011_00001_0010011, -2048i32);
            immediate_test!(XORI, 0b1000000_00000_00010_100_00001_0010011, -2048i32);
            immediate_test!(ORI, 0b1000000_00000_00010_110_00001_0010011, -2048i32);
            immediate_test!(ANDI, 0b1000000_00000_00010_111_00001_0010011, -2048i32);
            immediate_test!(SLLI, 0b0000000_11111_00010_001_00001_0010011, 31);
            immediate_test!(SRLI, 0b0000000_11111_00010_101_00001_0010011, 31);
            immediate_test!(SRAI, 0b0100000_11111_00010_101_00001_0010011, 31);
        }
    }

    mod arithmetic_register {
        use super::super::*;
        macro_rules! arith_test {
            ($instr:ident,  $code:expr) => {{
                assert_eq!(Instruction::new($code), Instruction::$instr(1, 2, 3));
            }};
        }
        #[test]
        fn test_arithmetic_register() {
            arith_test!(ADD, 0b0000000_00011_00010_000_00001_0110011);
            arith_test!(SUB, 0b0100000_00011_00010_000_00001_0110011);
            arith_test!(SLL, 0b0000000_00011_00010_001_00001_0110011);
            arith_test!(SLT, 0b0000000_00011_00010_010_00001_0110011);
            arith_test!(SLTU, 0b0000000_00011_00010_011_00001_0110011);
            arith_test!(XOR, 0b0000000_00011_00010_100_00001_0110011);
            arith_test!(SRL, 0b0000000_00011_00010_101_00001_0110011);
            arith_test!(SRA, 0b0100000_00011_00010_101_00001_0110011);
            arith_test!(OR, 0b0000000_00011_00010_110_00001_0110011);
            arith_test!(AND, 0b0000000_00011_00010_111_00001_0110011);
            arith_test!(MUL, 0b0000001_00011_00010_000_00001_0110011);
            arith_test!(MULH, 0b0000001_00011_00010_001_00001_0110011);
            arith_test!(MULHSU, 0b0000001_00011_00010_010_00001_0110011);
            arith_test!(MULHU, 0b0000001_00011_00010_011_00001_0110011);
            arith_test!(DIV, 0b0000001_00011_00010_100_00001_0110011);
            arith_test!(DIVU, 0b0000001_00011_00010_101_00001_0110011);
            arith_test!(REM, 0b0000001_00011_00010_110_00001_0110011);
            arith_test!(REMU, 0b0000001_00011_00010_111_00001_0110011);
        }

        #[test]
        fn test_atomics() {
            assert_eq!(
                Instruction::new(0b0001000_00000_00010_010_00001_0101111),
                Instruction::LRW(1, 2, 0)
            );
            arith_test!(SCW, 0b0001100_00011_00010_010_00001_0101111);
            arith_test!(AMOSWAPW, 0b0000100_00011_00010_010_00001_0101111);
            arith_test!(AMOADDW, 0b0000000_00011_00010_010_00001_0101111);
            arith_test!(AMOXORW, 0b0010000_00011_00010_010_00001_0101111);
            arith_test!(AMOANDW, 0b0110000_00011_00010_010_00001_0101111);
            arith_test!(AMOORW, 0b0100000_00011_00010_010_00001_0101111);
            arith_test!(AMOMINW, 0b1000000_00011_00010_010_00001_0101111);
            arith_test!(AMOMAXW, 0b1010000_00011_00010_010_00001_0101111);
            arith_test!(AMOMINUW, 0b1100000_00011_00010_010_00001_0101111);
            arith_test!(AMOMAXUW, 0b1110000_00011_00010_010_00001_0101111);
        }
    }

    mod other {
        use super::super::*;

        #[test]
        fn test_ebreak() {
            assert_eq!(Instruction::new(0x00100073), Instruction::EBREAK);
        }

        #[test]
        fn test_mret() {
            assert_eq!(Instruction::new(0x30200073), Instruction::MRET);
        }
    }
}
