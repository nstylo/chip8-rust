mod chip8;
mod cpu;
mod mem;

use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut file = std::fs::File::open("./data/greeting.ch8")?;
    let mut buf = Vec::<u8>::new();
    let nrof_bytes = file.read_to_end(&mut buf)?;
    println!("{}", nrof_bytes);

    let mut chip8 = chip8::Chip8::new();
    chip8.load_rom(&buf);

    println!("{:02X?}", chip8.ram.read(0x200));
    println!("{:02X?}", chip8.ram.read(0x200 + 1));

    println!("{:#02X?}", chip8.load_instruction(0x200));

    Ok(())
}
