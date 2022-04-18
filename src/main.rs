mod cpu;

use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mut file = std::fs::File::open("./data/greeting.ch8")?;
    let mut buf = Vec::<u8>::new();
    let nrof_bytes = file.read_to_end(&mut buf)?;
    println!("{}", nrof_bytes);

    Ok(())
}
