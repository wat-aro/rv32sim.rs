#[derive(Debug)]
pub struct Memory {
    data: Vec<u8>,
}

const MEMORY_SIZE: u32 = 1024 * 1024;

impl Memory {
    pub fn new() -> Self {
        Memory {
            data: vec![0; MEMORY_SIZE as usize],
        }
    }

    pub fn write(&mut self, addr: u32, word: u32) {
        let index = addr as usize;
        self.data[index] = (word & 0xff) as u8;
        self.data[index + 1] = ((word >> 8) & 0xff) as u8;
        self.data[index + 2] = ((word >> 16) & 0xff) as u8;
        self.data[index + 3] = ((word >> 24) & 0xff) as u8;
    }

    pub fn read(&self, addr: u32) -> u32 {
        let index = addr as usize;
        self.data[index] as u32
            | (self.data[index + 1] as u32) << 8
            | (self.data[index + 2] as u32) << 16
            | (self.data[index + 3] as u32) << 24
    }

    pub fn initialize(&mut self, data: Vec<u8>) {
        self.data.splice(..data.len(), data);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_empty_memory() {
        let memory = Memory::new();

        assert_eq!(memory.read(0), 0);
    }

    #[test]
    fn read_and_write_full_word() {
        let full_word = (2u64.pow(32) - 1) as u32;
        let mut memory = Memory::new();
        let addr: u32 = 5;
        memory.write(addr, full_word);
        assert_eq!(memory.read(addr), full_word);
    }

    #[test]
    fn test_read() {
        let mut memory = Memory::new();
        memory.initialize(vec![0x93, 0x00, 0x01, 0x01, 0x94, 0x00, 0x01, 0x01]);
        let addr = 0;
        assert_eq!(0x01010093, memory.read(addr));

        let addr = 4;
        assert_eq!(0x01010094, memory.read(addr));

        let addr = 8;
        assert_eq!(0, memory.read(addr));
    }

    #[test]
    fn test_set_data() {
        let mut memory = Memory::new();
        memory.initialize(vec![0x93, 0x00, 0x01, 0x01, 0x94, 0x00, 0x01, 0x01]);

        assert_eq!(0x01010093, memory.read(0));
        assert_eq!(0x01010094, memory.read(4));

        memory.write(0, 0x01010095);
        memory.write(4, 0x01010096);

        assert_eq!(0x01010095, memory.read(0));
        assert_eq!(0x01010096, memory.read(4));
    }
}
