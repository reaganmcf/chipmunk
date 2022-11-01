use std::{io::{self, BufReader, Read}, fs::File};

use crate::{emulator::Emulator};

mod emulator;
mod registers;
mod opcode;
mod error;
mod display;

fn main() -> io::Result<()> {
    // Get rom data
    let f = File::open("/home/rmcf/Code/chipmunk/roms/helloworld.rom")?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    
    // Read file into vector.
    reader.read_to_end(&mut buffer)?;
    println!("{:#?}", buffer);
    
    let mut em = Emulator::new(buffer);
    em.start();
    println!("{:#?}", em.registers);

    Ok(())
}