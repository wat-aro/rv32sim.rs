use crate::error::Error;
use Instruction::*;

#[derive(Debug)]
pub enum Instruction {
    Add { rd: u32, rs1: u32, rs2: u32 },
    Sub { rd: u32, rs1: u32, rs2: u32 },
    Or { rd: u32, rs1: u32, rs2: u32 },
    And { rd: u32, rs1: u32, rs2: u32 },
    Addi { rd: u32, rs1: u32, imm: u32 },
    Silli { rd: u32, rs1: u32, imm: u32 },
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
            0b0010011 => {
                let imm = (inst & 0xfff00000) >> 20;

                match funct3 {
                    0x0 => Ok(Addi { rd, rs1, imm }),
                    0x1 => Ok(Silli { rd, rs1, imm }),
                    _ => Err(Error::IllegalInstruction(inst)),
                }
            }
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
            0b0000011 => {
                let imm = (inst & 0xfff00000) >> 20;
                match funct3 {
                    0x2 => Ok(Lw { rd, rs1, imm }),
                    _ => Err(Error::IllegalInstruction(inst)),
                }
            }
            0b0100011 => {
                let imm = ((inst & 0xfe000000) >> 20) | ((inst & 0x00000f80) >> 7);
                match funct3 {
                    0x2 => Ok(Sw { rs1, rs2, imm }),
                    _ => Err(Error::IllegalInstruction(inst)),
                }
            }
            _ => Err(Error::IllegalInstruction(inst)),
        }
    }
}
