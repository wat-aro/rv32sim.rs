use std::io::{self, Read, Write};

use anyhow::Result;

pub const SERIAL_ADDRESS: u32 = 0x10000000;

pub struct Serial {
    input: Box<dyn Read>,
    output: Box<dyn Write>,
}

impl Serial {
    pub fn new() -> Self {
        Serial {
            input: Box::new(io::stdin()),
            output: Box::new(io::stdout()),
        }
    }

    pub fn write(&mut self, word: u32) -> Result<()> {
        let b1 = (word & 0xff) as u8;
        let b2 = ((word >> 8) & 0xff) as u8;
        let b3 = ((word >> 16) & 0xff) as u8;
        let b4 = ((word >> 24) & 0xff) as u8;
        self.output.write(&[b1, b2, b3, b4])?;
        Ok(())
    }

    pub fn read(&mut self) -> Result<u32> {
        let mut buf = [0; 4];
        self.input.read(&mut buf)?;
        let num = (buf[0]) as u32
            | ((buf[1] as u32) << 8)
            | ((buf[2] as u32) << 16)
            | ((buf[3] as u32) << 24);
        Ok(num)
    }
}
