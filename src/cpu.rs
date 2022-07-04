use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::serial::{Serial, SERIAL_ADDRESS};
use crate::x_registers::XRegisters;
use anyhow::Result;

pub struct Cpu {
    pub pc: u32,
    pub x_registers: XRegisters,
    memory: Memory,
    serial: Serial,
    nop_count: u8,
}

pub enum Status {
    Processing,
    Finished,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            pc: 0,
            x_registers: XRegisters::new(),
            memory: Memory::new(),
            serial: Serial::new(),
            nop_count: 0,
        }
    }

    pub fn initialize_memory(&mut self, data: Vec<u8>) {
        self.memory.initialize(data);
    }

    pub fn fetch(&self) -> u32 {
        self.memory.read(self.pc)
    }

    pub fn run(&mut self) -> Result<Status> {
        let raw_inst = self.fetch();

        let inst = Instruction::decode(raw_inst)?;

        if let Instruction::Nop = inst {
            self.nop_count += 1;
        } else {
            self.nop_count = 0;
        }

        if self.nop_count >= 5 {
            return Ok(Status::Finished);
        }
        self.execute(inst)?;

        Ok(Status::Processing)
    }

    pub fn execute(&mut self, inst: Instruction) -> Result<()> {
        match inst {
            Instruction::Add { rd, rs1, rs2 } => {
                let value = self
                    .x_registers
                    .read(rs1)
                    .wrapping_add(self.x_registers.read(rs2));
                self.x_registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Sub { rd, rs1, rs2 } => {
                let value = self
                    .x_registers
                    .read(rs1)
                    .wrapping_sub(self.x_registers.read(rs2));
                self.x_registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Or { rd, rs1, rs2 } => {
                let value = self.x_registers.read(rs1) | self.x_registers.read(rs2);
                self.x_registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::And { rd, rs1, rs2 } => {
                let value = self.x_registers.read(rs1) & self.x_registers.read(rs2);
                self.x_registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Addi { rd, rs1, imm } => {
                // imm は 12bit の signed int なので 12bit 目が 0 なら正、1なら負
                let num = match (imm & 0x80) == 0 {
                    true => imm,
                    false => 0xfffff000 | imm, // 13bit目以降を 1 で埋める
                };
                let value = self.x_registers.read(rs1).wrapping_add(num);
                self.x_registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Slli { rd, rs1, imm } => {
                let value = self.x_registers.read(rs1) << (imm & 0b11111);
                self.x_registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Beq { rs1, rs2, imm } => {
                // imm は 13bit の signed int なので 13bit 目が 0 なら正、1なら負
                let num = match (imm & 0xc0) == 0 {
                    true => imm,
                    false => 0xfffff000 | imm, // 13bit目以降を 1 で埋める
                };

                if self.x_registers.read(rs1) == self.x_registers.read(rs2) {
                    self.pc = self.pc.wrapping_add(num);
                } else {
                    self.pc += 4;
                }
                Ok(())
            }
            Instruction::Lw { rd, rs1, imm } => {
                let addr = self.x_registers.read(rs1) + imm;
                let value = if addr == SERIAL_ADDRESS {
                    self.serial.read()?
                } else {
                    self.memory.read(addr)
                };
                self.x_registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = self.x_registers.read(rs1) + imm;
                let value = self.x_registers.read(rs2);
                if addr == SERIAL_ADDRESS {
                    self.serial.write(value)?;
                } else {
                    self.memory.write(addr, value);
                }
                self.pc += 4;
                Ok(())
            }
            Instruction::Nop => {
                self.pc += 4;
                Ok(())
            }
        }
    }
}
