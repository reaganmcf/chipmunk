use clap::{Parser, Subcommand};
use drivers::Sdl2Platform;
use std::{
    fs::File,
    io::{self, BufReader, Read},
};

mod drivers;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a rom
    Run {
        rom: String,
        #[arg(
            short,
            long,
            help = "Enable debugging (waits for 'f' keypress between cycles)"
        )]
        debug: bool,
    },
    /// Disassemble a rom for debugging
    Dis { rom: String },
}

fn open_rom(path: String) -> io::Result<Vec<u8>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Run { rom, debug } => {
            let buffer = open_rom(rom)?;
            let platform = Box::new(Sdl2Platform::new());
            chipmunk_backend::run(buffer, platform, debug)
        }
        Commands::Dis { rom } => {
            let buffer = open_rom(rom)?;
            match chipmunk_backend::disassemble(buffer) {
                Ok(ops) => println!("{:#?}", ops),
                Err(e) => eprintln!("{:#?}", e),
            }
        }
    }
    Ok(())
}
