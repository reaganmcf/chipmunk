use std::time::Duration;
use rand::Rng;
use sdl2::keyboard::Keycode;
use sdl2::{event::Event, EventPump};

use crate::audio::Audio;
use crate::keyboard::Keyboard;
use crate::registers::Reg;
use crate::utils::bcd;
use crate::{
    display::{Display, DISPLAY_HEIGHT, DISPLAY_WIDTH, VRAM},
    error::EmulatorError,
    opcode::OpCode,
    registers::Registers,
};

const STACK_COUNT: usize = 12;
const MEM_SIZE: usize = 4096;
const FPS: f32 = 60.0;

const FONT_SET: [u8; 80] = [
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

const FONT_SET_START_ADDR: usize = 0x050;
const FONT_SET_END_ADDR: usize = 0x0A0;

pub struct Emulator {
    // 0x000 -> 0x1FF = interpter
    // 0x050 -> 0x0A0 = pixel font
    // 0x200 -> 0xFFF = rom and everything else
    memory: [u8; MEM_SIZE], // 4 KB of memory that lives for the entire program
    pub registers: Registers,
    stacks: Vec<u16>,
    vram: VRAM,
    context: sdl2::Sdl,
    event_pump: EventPump,
    display: Display,
    audio: Audio,
    keyboard: Keyboard,
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
        let audio = Audio::new(&mut context);
        let keyboard = Keyboard::new();

        let mut emulator = Self {
            memory,
            registers,
            stacks: Vec::with_capacity(STACK_COUNT),
            vram,
            context,
            event_pump,
            display,
            audio,
            keyboard,
        };

        emulator.load_font();
        emulator.load_rom(rom);

        return emulator;
    }

    fn load_font(&mut self) {
        let mut i = FONT_SET_START_ADDR;
        while i < FONT_SET_END_ADDR {
            self.memory[i] = FONT_SET[i - FONT_SET_START_ADDR];
            i += 1;
        }
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
        'running: loop {
            if let Err(e) = self.cycle() {
                // don't leave audio on before we panic
                self.audio.stop();

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

            std::thread::sleep(Duration::from_secs_f32(1.0 / FPS));

            self.display.draw(self.vram);

            // sound timer
            self.check_sound();
            // delay timer
            self.check_delay();

            //maybe dont actually need?
            //set keys (TODO)
        }
    }

    // TODO - better name
    fn check_sound(&mut self) {
        let sound_timer = self.registers.get(Reg::SoundTimer);

        if sound_timer > 0 {
            if sound_timer == 1 {
                self.audio.start();
            }
            self.registers.set(Reg::SoundTimer, sound_timer - 1);
        } else {
            self.audio.stop();
        }
    }

    fn check_delay(&mut self) {
        let delay_timer = self.registers.get(Reg::DelayTimer);

        if delay_timer > 0 {
            self.registers.set(Reg::DelayTimer, delay_timer - 1);
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
            OpCode::_00EE => {
                let ret_address = self.stacks.pop().expect("Must return from a subroutine");
                self.registers.goto(ret_address);
            }
            OpCode::_1NNN(nnn) => self.registers.goto(nnn),
            OpCode::_2NNN(nnn) => {
                self.stacks.push(self.registers.pc());

                self.registers.goto(nnn)
            }
            OpCode::_3XNN { reg, value } => {
                let x = self.registers.get(reg);
                if x == value {
                    self.registers.advance_pc();
                }
            }
            OpCode::_4XNN { reg, value } => {
                let x = self.registers.get(reg);
                if x != value {
                    self.registers.advance_pc();
                }
            }
            OpCode::_6XNN { reg, value } => self.registers.set(reg, value),
            OpCode::_7XNN { reg, value } => {
                let original = self.registers.get(reg);
                let new = original + value;
                self.registers.set(reg, new)
            }
            OpCode::_8XY0 { x, y } => {
                let y = self.registers.get(y);
                self.registers.set(x, y);
            }
            OpCode::_8XY2 { x, y } => {
                let val_x = self.registers.get(x);
                let val_y = self.registers.get(y);

                let value = val_x & val_y;
                self.registers.set(x, value);
            }
            OpCode::ANNN(nnn) => self.registers.set_i(nnn),
            OpCode::EXA1(reg) => {
                let expected_key = self.registers.get(reg);
                if let Some(pressed_key) = Keyboard::get_keypress(&mut self.event_pump) {
                    if pressed_key != expected_key {
                        self.registers.advance_pc();
                    }
                }
            }
            OpCode::CXNN { reg, value } => {
                let random_number: u8 = rand::thread_rng().gen();
                let value = random_number & value;
                self.registers.set(reg, value);
            }
            OpCode::DXYN { x, y, height } => {
                //println!("{:?}", self.registers);
                let x = self.registers.get(x);
                let y = self.registers.get(y);
                //println!("{}", y);
                let i: usize = self
                    .registers
                    .get_i()
                    .try_into()
                    .expect("unable to convert u16 to usize");

                for yline in 0..height {
                    let pixel = self.memory[i + (yline as usize)];
                    //println!("i = {:x}, pixel = {:x}", i, pixel);
                    for xline in 0..8 {
                        // dont know for sure, but i think we're only taking first 4 bits
                        let is_on = pixel & (0x80 >> xline) != 0;
                        let y_idx = (y + yline) as usize % DISPLAY_HEIGHT;
                        let x_idx = (x + xline) as usize % DISPLAY_WIDTH;

                        self.vram[y_idx][x_idx] = (self.vram[y_idx][x_idx] == true) ^ is_on;

                        //println!("drawing at [{}][{}]", y_idx, x_idx);
                        //self.vram[y_idx][x_idx] = is_on;
                    }
                }
            } 
            OpCode::FX07(reg) => {
                let value = self.registers.get(Reg::DelayTimer);
                self.registers.set(reg, value);
            }
            OpCode::FX0A(dest_reg) => {
                let val = Keyboard::await_keypress(&mut self.event_pump)?;
                self.registers.set(dest_reg, val);
            }
            OpCode::FX15(reg) => {
                let value = self.registers.get(reg);
                self.registers.set(Reg::DelayTimer, value);
            }
            OpCode::FX18(value) => self.registers.set(Reg::SoundTimer, value),
            OpCode::FX29(reg) => {
                let character: usize = self
                    .registers
                    .get(reg)
                    .try_into()
                    .expect("couldn't go from u8 -> usize");

                // 5 rows per character
                let offset: usize = character * 0x5;

                let sprite_addr = FONT_SET_START_ADDR + offset;

                self.registers.set_i(sprite_addr as u16);
            }
            OpCode::FX33(reg) => {
                let val = self.registers.get(reg);
                let bcd = bcd(val);

                let i: usize = self
                    .registers
                    .get_i()
                    .try_into()
                    .expect("unable to convert u16 to usize");

                self.memory[i] = bcd[2];
                self.memory[i + 1] = bcd[1];
                self.memory[i + 2] = bcd[0];
            }
            OpCode::FX65(reg) => {
                // fill v0 to vreg (inclusive) with values from memory
                let i: usize = self
                    .registers
                    .get_i()
                    .try_into()
                    .expect("unable to convert u16 to usize");
                    
                let start = 0x0;
                let end: usize = reg.into();
                for idx in start..=end {
                    let reg: Reg = (start + idx).into();
                    let val = self.memory[i + idx];
                    self.registers.set(reg, val);
                }
            }
        }

        Ok(())
    }
}
