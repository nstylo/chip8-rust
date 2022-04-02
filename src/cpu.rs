pub type Instruction = (u8, u8, u8, u8);

pub struct Cpu {
    v: [u8; 0x10],
    i: u16,
    pc: u16,
    sp: u8,
    delay: u8,
    sound: u8,
    stack: [u16; 0x10],
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            v: [0x0; 0x10],
            i: 0x00,
            pc: 0x00, // TODO: where does the program start?
            sp: 0x0,
            delay: 0x0,
            sound: 0x0,
            stack: [0x0; 0x10],
        }
    }

    pub fn run_instruction(&mut self, instruction: Instruction) {}
}
