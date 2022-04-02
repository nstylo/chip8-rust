struct Cpu {
    Vx: [u8; 16],
    I: u16,
    PC: u16,
    SP: u8,
    delay: u8,
    sound: u8,
    stack: [u16; 16],
}
