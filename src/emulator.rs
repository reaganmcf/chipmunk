use sdl2::keyboard::Keycode;
use sdl2::{event::Event, EventPump};

use crate::{
    display::{Display, DISPLAY_HEIGHT, DISPLAY_WIDTH, VRAM},
    error::EmulatorError,
    opcode::OpCode,
    registers::Registers,
};

const STACK_COUNT: usize = 12;
const MEM_SIZE: usize = 4096;

pub struct Emulator {
    // 0x000 -> 0x1FF = interpter
    // 0x050 -> 0x0A0 = pixel font
    // 0x200 -> 0xFFF = rom and everything else
    memory: [u8; MEM_SIZE], // 4 KB of memory that lives for the entire program
    pub registers: Registers,
    stacks: Vec<u16>,
    vram: VRAM,
    display: Display,
    context: sdl2::Sdl,
    event_pump: EventPump,
}

impl Emulator {
    pub fn new(rom: Vec<u8>) -> Self {
        // TODO - load rom?
        let memory: [u8; MEM_SIZE] = [0; MEM_SIZE];
        let registers = Registers::new();
        let vram: VRAM = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

        let mut context = sdl2::init().unwrap();
        let event_pump = context.event_pump().unwrap();
        let display = Display::new(&mut context);

        let mut emulator = Self {
            memory,
            registers,
            stacks: Vec::with_capacity(STACK_COUNT),
            vram,
            display,
            context,
            event_pump,
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
        for _ in [0, 1, 2, 3, 4] {
            println!("{:#?}", self.registers);
        }

        let mut i: usize = 0;
        let mut j: usize = 0;

        'running: loop {
            if let Err(e) = self.cycle() {
                panic!("Ran into error: {:#?}", e);
            }

            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running,
                    _ => {}
                }
            }

            self.vram[j][i] = true;
            i += 1;
            if i % DISPLAY_WIDTH == 0 {
                i %= DISPLAY_WIDTH;
                j += 1;
            }

            self.display.draw(self.vram);
            //set keys (TODO)
        }
    }

    fn cycle(&mut self) -> Result<(), EmulatorError> {
        let opcode = self.fetch_opcode()?;
        println!("{:#?}", opcode);
        self.exec_opcode(opcode)?;

        // update timers (TODO)
        Ok(())
    }

    fn fetch_opcode(&mut self) -> Result<OpCode, EmulatorError> {
        let index: usize = self
            .registers
            .pc()
            .try_into()
            .expect("unable to convert u16 to usize");
        let first_half = self.memory[index];
        let second_half = self.memory[index + 0x1];
        let raw_opcode: u16 = u16::from_be_bytes([first_half, second_half]);

        let opcode = raw_opcode.try_into();

        self.registers.advance_pc();

        opcode
    }

    fn exec_opcode(&mut self, op: OpCode) -> Result<(), EmulatorError> {
        match op {
            OpCode::_00E0 => self.display.clear(),
            OpCode::_6XNN { register, value } => self.registers.set(register, value),
            OpCode::ANNN(nnn) => self.registers.set_i(nnn),
            OpCode::FX0A(dest_reg) => {
                // TODO - use full hex keyboard
                loop {
                    match self.event_pump.wait_event() {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::F),
                            ..
                        } => break,
                        _ => {}
                    }
                }
                self.registers.set(dest_reg, 0xF);
            }
        }

        Ok(())
    }
}
