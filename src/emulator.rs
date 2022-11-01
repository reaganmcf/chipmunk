use crate::{registers::Registers, opcode::OpCode, error::EmulatorError};

const STACK_COUNT: usize = 12;
const MEM_SIZE: usize = 4096;

pub struct Emulator {
    // 0x000 -> 0x1FF = interpter
    // 0x050 -> 0x0A0 = pixel font
    // 0x200 -> 0xFFF = rom and everything else
    memory: [u8; MEM_SIZE], // 4 KB of memory that lives for the entire program
    registers: Registers,
    stacks: Vec<u16>,
}

impl Emulator {
    pub fn new(rom: Vec<u8>) -> Self {
        // TODO - load rom?
        let memory: [u8; MEM_SIZE] = [0; MEM_SIZE];
        let registers = Registers::new();

        let mut emulator = Self {
            memory,
            registers,
            stacks: Vec::with_capacity(STACK_COUNT),
        };

        emulator.load_rom(rom);

        return emulator;
    }

    fn load_rom(&mut self, rom: Vec<u8>) {
        // Roms get loaded starting at memory location 0x200
        let mut idx: usize = 0x200;
        for byte in rom.into_iter() {
            *self.memory.get_mut(idx).expect("rom went over 4k") = byte;

            idx += 1;
        }
    }

    pub fn start(&mut self) {
        //loop {
        // emulate cycle
        for _ in [0,1,2,3] {
            self.cycle();
        }

        // maybe draw? (TODO)
        // set keys (TODO)
        //}
    }

    fn cycle(&mut self) {
        let opcode = self.fetch_opcode();

        println!("{:#?}", opcode);
        // fetch opcode
        // decode op code
        // execute op code
        //
        // update timers (TODO)
    }

    fn fetch_opcode(&mut self) -> Result<OpCode, EmulatorError> {
        let index: usize = self.registers.pc().try_into().expect("unable to convert u16 to usize");
        let first_half = self.memory[index];
        let second_half = self.memory[index + 0x1];
        let raw_opcode: u16 = u16::from_be_bytes([first_half, second_half]);

        println!("{:x} {:x}, together = {:x}", first_half, second_half, raw_opcode);
        let opcode = raw_opcode.try_into();

        self.registers.advance_pc();

        opcode
    }
}
