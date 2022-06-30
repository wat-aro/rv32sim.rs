use crate::memory::Memory;
use crate::x_registers::XRegisters;

struct Cpu {
    pc: u32,
    x_registers: XRegisters,
    memory: Memory,
    nop_count: u8,
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
}
