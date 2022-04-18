use rand::Rng;

pub struct Cpu {
    v: [u8; 0x10],
    i: u16,
    pc: u16,
    sp: u8,
    delay: u8,
    sound: u8,
    stack: [u16; 0x10],
    ram: [u8; 0xFFF],
}

impl Cpu {
    pub fn new() -> Self {
        let mut ram = [0x00; 0xFFF];

        // Define sprites
        #[rustfmt::skip]
        let sprites: [u8; 5*0x10] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        // Load sprites
        for (i, elem) in sprites.iter().enumerate() {
            ram[i] = *elem;
        }

        Self {
            v: [0x0; 0x10],
            i: 0x00,
            pc: 0x00, // TODO: where does the program start?
            sp: 0x0,
            delay: 0x0,
            sound: 0x0,
            stack: [0x0; 0x10],
            ram: [0x00; 0xFFF],
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        self.ram[address as usize] = value;
    }

    fn read(&self, address: u16) -> u8 {
        self.ram[address as usize]
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        let offset = 0x200;
        for (i, val) in data.iter().enumerate() {
            self.write((offset + i) as u16, *val)
        }
    }

    pub fn load_instruction(&self, addr: u16) -> u16 {
        // TODO: assert whether addr is even
        let hi = self.read(addr) as u16;
        let lo = self.read(addr + 0x0001) as u16;

        hi << 8 | lo
    }

    pub fn run_instruction(&mut self, instruction: u16) {
        let opcode = (
            (instruction & 0xF000) >> 12,
            (instruction & 0x0F00) >> 8,
            (instruction & 0x00F0) >> 4,
            (instruction & 0x000F),
        );

        let nnn = instruction & 0x0FFF;
        let kk = (instruction & 0x00FF) as u8;

        let n = opcode.0;
        let y = opcode.1 as usize;
        let x = opcode.2 as usize;

        match opcode {
            (0x0, 0x0, 0xE, 0x0) => {
                // CLS - Clear the display
                // TODO
            }
            (0x0, 0x0, 0xE, 0xE) => {
                // RET - Return from subroutine
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }
            (0x1, _, _, _) => {
                // JP <addr> - Jump to location at <addr>
                self.pc = nnn;
            }
            (0x2, _, _, _) => {
                // CALL <addr> - Call subrouting at <addr>
                self.sp += 1;
                self.stack[self.sp as usize] = self.pc;
                self.pc = nnn;
            }
            (0x3, _, _, _) => {
                // SE <Vx> <byte> - Skip next instruction if <Vx> = <byte>
                if self.v[x] == kk {
                    self.pc += 2;
                }
            }
            (0x4, _, _, _) => {
                // SNE <Vx> <byte> - Skip next instruction if <Vx> != <byte>
                if self.v[x] != kk {
                    self.pc += 2;
                }
            }
            (0x5, _, _, 0x0) => {
                // SE <Vx> <Vy> - Skip next instruction if <Vx> = <Vy>
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            }
            (0x6, _, _, _) => {
                // LD <Vx> <byte> - Set <Vx> = <byte>
                self.v[x] = kk;
            }
            (0x7, _, _, _) => {
                // Add <Vx> <byte> - Set <Vx> = <Vx> + <byte>
                self.v[x] += kk;
            }
            (0x8, _, _, 0x0) => {
                // Store value of register <Vy> in register <Vx>, <Vy> = <Vx>
                self.v[x] = self.v[y];
            }
            (0x8, _, _, 0x1) => {
                // Store in register <Vx> the result of bitwise OR on <Vx> and <Vy>
                // <Vx> = <Vx> | <Vy>
                self.v[x] = self.v[x] | self.v[y];
            }
            (0x8, _, _, 0x2) => {
                // Store in register <Vx> the result of bitwise AND on <Vx> and <Vy>
                // <Vx> = <Vx> & <Vy>
                self.v[x] = self.v[x] & self.v[y];
            }
            (0x8, _, _, 0x3) => {
                // Store in register <Vx> the result of bitwise XOR on <Vx> and <Vy>
                // <Vx> = <Vx> ^ <Vy>
                self.v[x] = self.v[x] ^ self.v[y];
            }
            (0x8, _, _, 0x4) => {
                // Store in register <Vx> the result of ADD on <Vx> and <Vy>
                // <Vx> = <Vx> + <Vy>, set <VF> = carry, i.e, when result > 255 set <VF> to 1, else
                // 0
                let vx = self.v[x] as u16;
                let vy = self.v[y] as u16;
                let result = vx + vy;
                self.v[x] = result as u8;
                self.v[0xF] = if result > 255 { 0x1 } else { 0x0 };
            }
            (0x8, _, _, 0x5) => {
                // Store in register <Vx> the result of SUB on <Vx> and <Vy>
                // <Vx> = <Vx> - <Vy> and <VF> = 1 if <Vy> < <Vx>, else <VF> = 0

                let result = self.v[x] - self.v[y];
                self.v[0xF] = if result > 0 { 0x1 } else { 0x0 };
                self.v[x] = result;
            }
            (0x8, _, _, 0x6) => {
                // Store in register <Vx> the result of SHR on <Vx>
                // If least significant bit of Vx is 1, then <VF> = 1, else 0.
                // <Vx> = <Vx> >> 1.
                self.v[0xF] = if (self.v[x] & 0x1) == 1 { 0x1 } else { 0x0 };
                self.v[x] = self.v[x] >> 1;
            }
            (0x8, _, _, 0x7) => {
                // Store in register <Vx> the result of SUBN on <Vx> and <Vy>
                // <Vx> = <Vy> - <Vx> and <VF> = 1 if <Vy> > <Vx>, else <VF> = 0
                let result = self.v[y] - self.v[x];
                self.v[0xF] = if result > 0 { 0x1 } else { 0x0 };
                self.v[x] = result;
            }
            (0x8, _, _, 0xE) => {
                // Store in register <Vx> the result of SHL on <Vx>
                // If most significant bit of Vx is 1, then <VF> = 1, else 0.
                // <Vx> = <Vx> << 1.
                self.v[0xF] = if (self.v[x] & (0x1 << 7)) == 1 {
                    0x1
                } else {
                    0x0
                };
                self.v[x] = self.v[x] << 1;
            }
            (0x9, _, _, 0x0) => {
                // SNE
                // Skip next instruction if <Vx> != <Vy>, i.e. set program counter += 2.
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            }
            (0xA, _, _, _) => {
                // Set I = nnn
                self.i = nnn;
            }
            (0xB, _, _, _) => {
                // Jump to nnn + <V0>, i.e. set programm counter to nnn + <V0>
                let mut rng = rand::thread_rng();
                self.pc = nnn + self.v[0] as u16;
            }
            (0xC, _, _, _) => {
                // Set <Vx> to <random byte> & <byte>.
                let random_byte: u8 = rand::thread_rng().gen();
                self.v[x] = random_byte & kk;
            }
            (0xD, _, _, _) => {
                // Read n bytes from memory (sprite), starting at the address stored in register I.
                // Display the sprites on the screen at (<Vx>, <Vy>). If any pixel was errased this
                // way, set <VF> to 1, else to 0 (collision).
            }
            (0xE, _, 0x9, 0xE) => {
                // Skip next instruction if key with value of <Vx> is pressed, i.e. increase PC by
                // 2.
            }
            (0xE, _, 0xA, 0x1) => {
                // Skip next instruction if key with value of <Vx> is pressed, i.e. increase PC by
                // 2.
            }
            (0xF, _, 0x0, 0x7) => {
                // Set <Vx> = value of DT register (delay timer)
            }
            (0xF, _, 0x0, 0xA) => {
                // Wait for key press, store the value of key in <Vx>.
            }
            (0xF, _, 0x1, 0x5) => {
                // Set delay timer DT = <Vx>
            }
            (0xF, _, 0x1, 0x8) => {
                // Set sound timer ST = <Vx>
            }
            (0xF, _, 0x1, 0xE) => {
                // Set I = I + <Vx>
            }
            (0xF, _, 0x2, 0x9) => {
                // Set I = location of sprite for digit <Vx>
            }
            (0xF, _, 0x3, 0x3) => {
                // Store BCD respresentation of <Vx> in memory locations I, I+1, I+2
            }
            (0xF, _, 0x5, 0x5) => {
                // Store registers <V0> through <Vx> in memory starting at location I
            }
            (0xF, _, 0x6, 0x5) => {
                // Read registers <V0> through <Vx> from memory starting at location I.
            }
            _ => println!("Invalid Instruction!"),
        }
    }
}
