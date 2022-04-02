use cpu;
use mem;

pub struct Chip8 {
    pub ram: mem::Mem,
    cpu: cpu::Cpu,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            ram: mem::Mem::new(),
            cpu: cpu::Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        let offset = 0x200;
        for (i, val) in data.iter().enumerate() {
            self.ram.write((offset + i) as u16, *val)
        }
    }

    pub fn load_instruction(&self, addr: u16) -> cpu::Instruction {
        // TODO: assert whether addr is even
        let hi = self.ram.read(addr);
        let lo = self.ram.read(addr + 0x0001);

        let hi_hi = hi >> 4;
        let hi_lo = hi & 0x0F;

        let lo_hi = lo >> 4;
        let lo_lo = lo & 0x0F;

        (hi_hi, hi_lo, lo_hi, lo_lo)
    }
}
