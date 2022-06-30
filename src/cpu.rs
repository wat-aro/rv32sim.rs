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
        Ok(())
    }
}
