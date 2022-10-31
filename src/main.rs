use crate::emulator::Emulator;

mod emulator;
mod registers;

fn main() {
    let mut em = Emulator::new();
    println!("Hello, world!");
}
