use crate::cpu::{Cpu, Status};
use anyhow::Result;

pub struct Simulator {
    cpu: Cpu,
}

impl Simulator {
    pub fn new() -> Simulator {
        Self { cpu: Cpu::new() }
    }

    pub fn initialize_memory(&mut self, data: Vec<u8>) {
        self.cpu.initialize_memory(data)
    }

    pub fn dump_registers(&self) {
        println!("{}", "-".repeat(80));
        for i in 0..=7 {
            for j in 0..=3 {
                if j != 0 {
                    print!("\t");
                }
                let n = (i * 4) + j;
                print!("x{0: >02} = 0x{1:x} ({1})", n, self.cpu.x_registers.read(n))
            }
            print!("\n");
        }
        println!("{}", "-".repeat(80));
        println!("pc = 0x{0:x} ({0})", self.cpu.pc);
    }

    pub fn start(&mut self) -> Result<()> {
        loop {
            match self.cpu.run() {
                Ok(status) => match status {
                    Status::Processing => {}
                    Status::Finished => {
                        break;
                    }
                },
                Err(e) => {
                    return Err(e);
                }
            }
        }

        Ok(())
    }
}
