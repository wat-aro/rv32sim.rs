use crate::error::Error;
use Instruction::*;

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Add { rd: u32, rs1: u32, rs2: u32 },
    Sub { rd: u32, rs1: u32, rs2: u32 },
    Or { rd: u32, rs1: u32, rs2: u32 },
    And { rd: u32, rs1: u32, rs2: u32 },
    Addi { rd: u32, rs1: u32, imm: u32 },
    Slli { rd: u32, rs1: u32, imm: u32 },
    Beq { rs1: u32, rs2: u32, imm: u32 },
    Lw { rd: u32, rs1: u32, imm: u32 },
    Sw { rs1: u32, rs2: u32, imm: u32 },
}

impl Instruction {
    pub fn decode(inst: u32) -> Result<Instruction, Error> {
        let opcode = inst & 0x0000007f;
        let rd = (inst & 0x00000f80) >> 7;
        let funct3 = (inst & 0x00007000) >> 12;
        let rs1 = (inst & 0x000f8000) >> 15;
        let rs2 = (inst & 0x01f00000) >> 20;
        let funct7 = (inst & 0xfe000000) >> 25;

        match opcode {
            // R-Type
            0b0110011 => match funct3 {
                0x0 => match funct7 {
                    0x00 => Ok(Add { rd, rs1, rs2 }),
                    0x20 => Ok(Sub { rd, rs1, rs2 }),
                    _ => Err(Error::IllegalInstruction(inst)),
                },
                0x6 => Ok(Or { rd, rs1, rs2 }),
                0x7 => Ok(And { rd, rs1, rs2 }),
                _ => Err(Error::IllegalInstruction(inst)),
            },
            // I-Type
            0b0010011 => {
                let imm = (inst & 0xfff00000) >> 20;

                match funct3 {
                    0x0 => Ok(Addi { rd, rs1, imm }),
                    0x1 => Ok(Slli { rd, rs1, imm }),
                    _ => Err(Error::IllegalInstruction(inst)),
                }
            }
            0b0000011 => {
                let imm = (inst & 0xfff00000) >> 20;
                match funct3 {
                    0x2 => Ok(Lw { rd, rs1, imm }),
                    _ => Err(Error::IllegalInstruction(inst)),
                }
            }
            // S-Type
            0b0100011 => {
                let imm = ((inst & 0xfe000000) >> 20) | ((inst & 0x00000f80) >> 7);
                match funct3 {
                    0x2 => Ok(Sw { rs1, rs2, imm }),
                    _ => Err(Error::IllegalInstruction(inst)),
                }
            }
            // B-Type
            0b1100011 => {
                let imm = ((inst & 0x80000000) >> 19)
                    | ((inst & 0x00000080) << 4)
                    | ((inst & 0x7e000000) >> 20)
                    | ((inst & 0x00000f00) >> 7);

                match funct3 {
                    0x0 => Ok(Beq { rs1, rs2, imm }),
                    _ => Err(Error::IllegalInstruction(inst)),
                }
            }
            _ => Err(Error::IllegalInstruction(inst)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_add() {
        let instruction = Instruction::decode(0b0000000_10111_10011_000_00000_0110011);
        assert_eq!(
            Ok(Instruction::Add {
                rd: 0,
                rs1: 19,
                rs2: 23
            }),
            instruction
        );
    }

    #[test]
    fn decode_sub() {
        let instruction = Instruction::decode(0b0100000_10111_10011_000_00000_0110011);
        assert_eq!(
            Ok(Instruction::Sub {
                rd: 0,
                rs1: 19,
                rs2: 23
            }),
            instruction
        );
    }

    #[test]
    fn decode_or() {
        let instruction = Instruction::decode(0b0100000_10111_10011_110_00000_0110011);
        assert_eq!(
            Ok(Instruction::Or {
                rd: 0,
                rs1: 19,
                rs2: 23
            }),
            instruction
        );
    }

    #[test]
    fn decode_and() {
        let instruction = Instruction::decode(0b0100000_10111_10011_111_00000_0110011);
        assert_eq!(
            Ok(Instruction::And {
                rd: 0,
                rs1: 19,
                rs2: 23
            }),
            instruction
        );
    }

    #[test]
    fn decode_addi() {
        let instruction = Instruction::decode(0b010000010111_10011_000_00000_0010011);
        assert_eq!(
            Ok(Instruction::Addi {
                rd: 0,
                rs1: 19,
                imm: 1047
            }),
            instruction
        );
    }

    #[test]
    fn decode_slli() {
        let instruction = Instruction::decode(0b000000000101_10011_001_00000_0010011);
        assert_eq!(
            Ok(Instruction::Slli {
                rd: 0,
                rs1: 19,
                imm: 5
            }),
            instruction
        );
    }

    #[test]
    fn decode_lw() {
        let instruction = Instruction::decode(0b000000000101_10011_010_00100_0000011);
        assert_eq!(
            Ok(Instruction::Lw {
                rd: 4,
                rs1: 19,
                imm: 5
            }),
            instruction
        );
    }

    #[test]
    fn decode_sw() {
        let instruction = Instruction::decode(0b000000000101_10011_010_00100_0100011);
        assert_eq!(
            Ok(Instruction::Sw {
                rs1: 19,
                rs2: 5,
                imm: 4
            }),
            instruction
        );
    }

    #[test]
    fn decode_beq() {
        let instruction = Instruction::decode(0b00000000_0101_10011_000_00010_1100011);
        assert_eq!(
            Ok(Instruction::Beq {
                rs1: 19,
                rs2: 5,
                imm: 2
            }),
            instruction
        );
    }
}
