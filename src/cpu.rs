use crate::error::Error;
use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::x_registers::XRegisters;

struct Cpu {
    pc: u32,
    x_registers: XRegisters,
    memory: Memory,
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
            nop_count: 0,
        }
    }

    pub fn initialize_memory(&mut self, data: Vec<u8>) {
        self.memory.initialize(data);
    }

    pub fn fetch(&self) -> u32 {
        self.memory.read(self.pc)
    }

    pub fn run(&mut self) -> Result<Status, Error> {
        if self.nop_count > 4 {
            return Ok(Status::Finished);
        }

        let raw_inst = self.fetch();
        let inst = Instruction::decode(raw_inst)?;

        self.execute(inst);

        Ok(Status::Processing)
    }

    pub fn execute(&mut self, inst: Instruction) -> Result<(), Error> {
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
                let value = self.x_registers.read(rs1) - self.x_registers.read(rs2);
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
                let num = match (imm & 0x80) == 0 {
                    true => imm,
                    false => !(imm & 0x7f) + 1,
                };
                let value = self.x_registers.read(rs1).wrapping_add(num);
                self.x_registers.write(rd, value);
                Ok(())
            }
            Instruction::Slli { rd, rs1, imm } => {
                let value = self.x_registers.read(rs1) << (self.x_registers.read(imm) & 0b11111);
                self.x_registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Beq { rs1, rs2, imm } => {
                let num = match (imm & 0xc0) == 0 {
                    true => imm,
                    false => !(imm & 0xbf) + 1,
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
                let value = self.memory.read(addr);
                self.x_registers.write(rd, value);
                self.pc += 4;
                Ok(())
            }
            Instruction::Sw { rs1, rs2, imm } => {
                let addr = self.x_registers.read(rs1) + imm;
                self.memory.write(addr, rs2);
                self.pc += 4;
                Ok(())
            }
        }
    }
}
