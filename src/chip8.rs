mod cpu;
mod mem;

struct Chip8 {
    ram: mem::Mem,
}

impl Chip8 {
    fn new(&self) -> Self {
        Self {
            ram: mem::Mem::new(),
        }
    }

    fn load_rom(&mut self, data: &Vec<u8>) {
        let offset = 0x200;
        for (i, val) in data.iter().enumerate() {
            self.ram.write((offset + i) as u16, *val)
        }
    }
}
