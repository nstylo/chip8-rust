mod mem;

use std::io::Read;

fn main() -> Result<(), std::io::Error> {
    let mem = mem::Mem::new();
    println!("{:?}", mem);

    let mut file = std::fs::File::open("./data/snake.ch8")?;
    let mut buf = Vec::<u8>::new();

    let nrof_bytes = file.read_to_end(&mut buf)?;
    println!("{}", nrof_bytes);
    println!("{:?}", &buf);

    Ok(())
}
