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

    pub fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            (0x0, 0x0, 0xE, 0x0) => {
                // CLS - Clear the display
            }
            (0x0, 0x0, 0xE, 0xE) => {
                // RET - Return from subroutine
            }
            (0x0, nnn, nn, n) => {
                // SYS <addr> - Jump to machine code routine at <addr>
            }
            (0x1, nnn, nn, n) => {
                // JP <addr> - Jump to location at <addr>
            }
            (0x2, nnn, nn, n) => {
                // CALL <addr> - Call subrouting at <addr>
            }
            (0x3, x, kk, k) => {
                // SE <Vx> <byte> - Skip next instruction if <Vx> = <byte>
            }
            (0x4, x, kk, k) => {
                // SNE <Vx> <byte> - Skip next instruction if <Vx> != <byte>
            }
            (0x5, x, y, 0x0) => {
                // SE <Vx> <Vy> - Skip next instruction if <Vx> = <Vy>
            }
            (0x6, x, y, 0x0) => {
                // SNE <Vx> <Vy> - Skip next instruction if <Vx> != <Vy>
            }
            (0x7, x, kk, k) => {
                // Add <Vx> <byte> - Set <Vx> = <Vx> + <byte>
            }
            (0x8, x, y, 0x0) => {
                // Store value of register <Vy> in register <Vx>, <Vy> = <Vx>
            }
            (0x8, x, y, 0x1) => {
                // Store in register <Vx> the result of bitwise OR on <Vx> and <Vy>
                // <Vx> = <Vx> | <Vy>
            }
            (0x8, x, y, 0x2) => {
                // Store in register <Vx> the result of bitwise AND on <Vx> and <Vy>
                // <Vx> = <Vx> & <Vy>
            }
            (0x8, x, y, 0x3) => {
                // Store in register <Vx> the result of bitwise XOR on <Vx> and <Vy>
                // <Vx> = <Vx> ^ <Vy>
            }
            (0x8, x, y, 0x4) => {
                // Store in register <Vx> the result of ADD on <Vx> and <Vy>
                // <Vx> = <Vx> + <Vy>, set <VF> = carry, i.e, when result > 255 set <VF> to 1, else
                // 0
            }
            (0x8, x, y, 0x5) => {
                // Store in register <Vx> the result of SUB on <Vx> and <Vy>
                // <Vx> = <Vx> - <Vy> and <VF> = 1 if <Vy> < <Vx>, else <Vx> = <Vy> - <Vx>
                // and <VF> = 0
            }
            (0x8, x, _, 0x6) => {
                // Store in register <Vx> the result of SHR on <Vx>
                // <Vx> = <Vx> >> 1. If least significant bit of Vx is 1, then <VF> = 1, else 0.
            }
            (0x8, x, y, 0x7) => {
                // Store in register <Vx> the result of SUBN on <Vx> and <Vy>
                // <Vx> = <Vy> - <Vx> and <VF> = 1 if <Vy> > <Vx>, else <Vx> = <Vx> - <Vy>
                // and <VF> = 0
            }
            (0x8, x, _, 0x8) => {
                // Store in register <Vx> the result of SHL on <Vx>
                // <Vx> = <Vx> << 1. If most significant bit of Vx is 1, then <VF> = 1, else 0.
            }
            (0x9, x, y, 0x0) => {
                // SNE
                // Skip next instruction if <Vx> != <Vy>, i.e. set program counter += 2.
            }
            (0xA, nnn, nn, n) => {
                // Set I = nnn
            }
            (0xB, nnn, nn, n) => {
                // Jump to nnn + <V0>, i.e. set programm counter to nnn + <V0>
            }
            (0xC, x, kk, k) => {
                // Set <Vx> to <random byte> & <byte>.
            }
            (0xD, x, y, n) => {
                // Read n bytes from memory (sprite), starting at the address stored in register I.
                // Display the sprites on the screen at (<Vx>, <Vy>). If any pixel was errased this
                // way, set <VF> to 1, else to 0 (collision).
            }
            (0xE, x, 0x9, 0xE) => {
                // Skip next instruction if key with value of <Vx> is pressed, i.e. increase PC by
                // 2.
            }
            (0xE, x, 0xA, 0x1) => {
                // Skip next instruction if key with value of <Vx> is pressed, i.e. increase PC by
                // 2.
            }
            (0xF, x, 0x0, 0x7) => {
                // Set <Vx> = value of DT register (delay timer)
            }
            (0xF, x, 0x0, 0xA) => {
                // Wait for key press, store the value of key in <Vx>.
            }
            (0xF, x, 0x1, 0x5) => {
                // Set delay timer DT = <Vx>
            }
            (0xF, x, 0x1, 0x8) => {
                // Set sound timer ST = <Vx>
            }
            (0xF, x, 0x1, 0xE) => {
                // Set I = I + <Vx>
            }
            (0xF, x, 0x2, 0x9) => {
                // Set I = location of sprite for digit <Vx>
            }
            (0xF, x, 0x3, 0x3) => {
                // Store BCD respresentation of <Vx> in memory locations I, I+1, I+2
            }
            (0xF, x, 0x5, 0x5) => {
                // Store registers <V0> through <Vx> in memory starting at location I
            }
            (0xF, x, 0x6, 0x5) => {
                // Read registers <V0> through <Vx> from memory starting at location I.
            }
            _ => println!("Invalid Instruction!"),
        }
    }
}
