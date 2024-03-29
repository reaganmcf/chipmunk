use rand::Rng;
use std::time::Duration;

use crate::registers::Reg;
use crate::utils::bcd;
use crate::{error::EmulatorError, opcode::OpCode, registers::Registers};
use crate::{Platform, Vram, DISPLAY_HEIGHT, DISPLAY_WIDTH};

const STACK_COUNT: usize = 12;
const MEM_SIZE: usize = 4096;

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
    platform: Box<dyn Platform>,
    // 0x000 -> 0x1FF = interpter
    // 0x050 -> 0x0A0 = pixel font
    // 0x200 -> 0xFFF = rom and everything else
    memory: [u8; MEM_SIZE], // 4 KB of memory that lives for the entire program
    pub registers: Registers,
    stacks: Vec<u16>,
    vram: Vram,
    draw_flag: bool,

    // Debug mode will wait each cycle for "f" to be pressed before continuing
    debug: bool,
}

impl Emulator {
    pub fn new(rom: Vec<u8>, platform: Box<dyn Platform>, debug: bool) -> Self {
        let memory: [u8; MEM_SIZE] = [0; MEM_SIZE];
        let registers = Registers::new();
        let vram: Vram = [[false; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

        let mut emulator = Self {
            platform,
            memory,
            registers,
            stacks: Vec::with_capacity(STACK_COUNT),
            vram,
            draw_flag: false,
            debug,
        };

        emulator.load_font();
        emulator.load_rom(rom);

        emulator
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
            self.platform.scan_keys();

            if let Err(e) = self.cycle() {
                // don't leave audio on before we panic
                self.platform.stop_beep();

                panic!("Ran into error: {:#?}", e);
            }

            if self.platform.should_quit() {
                break 'running;
            }

            std::thread::sleep(Duration::from_millis(2));
            if self.draw_flag {
                self.platform.draw(self.vram);
                self.draw_flag = false;
            }

            // sound timer
            self.check_sound();
            // delay timer
            self.check_delay();
        }
    }

    // TODO - better name
    fn check_sound(&mut self) {
        let sound_timer = self.registers.get(Reg::SoundTimer);

        if sound_timer > 0 {
            if sound_timer == 1 {
                self.platform.start_beep();
            }
            self.registers.set(Reg::SoundTimer, sound_timer - 1);
        } else {
            self.platform.stop_beep();
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

        if self.debug {
            loop {
                match self.platform.await_keypress() {
                    Ok(0xf) => break,
                    Err(e) => {
                        println!("{:#?}", e);
                        return Err(e);
                    }
                    _ => {}
                }
            }

            println!("{:#?}", self.registers);
        }

        Ok(())
    }

    fn fetch_opcode(&mut self) -> Result<OpCode, EmulatorError> {
        let index = self.registers.pc() as usize;
        let first_half = self.memory[index];
        let second_half = self.memory[index + 0x1];
        let raw_opcode: u16 = u16::from_be_bytes([first_half, second_half]);

        let opcode = raw_opcode.try_into();

        self.registers.advance_pc();

        opcode
    }

    fn exec_opcode(&mut self, op: OpCode) -> Result<(), EmulatorError> {
        match op {
            OpCode::_00E0 => {
                for row in self.vram.iter_mut() {
                    for pixel in row.iter_mut() {
                        *pixel = false;
                    }
                }
                self.draw_flag = true;
            }
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
            OpCode::_5XY0 { x, y } => {
                let x = self.registers.get(x);
                let y = self.registers.get(y);
                if x == y {
                    self.registers.advance_pc();
                }
            }
            OpCode::_6XNN { reg, value } => self.registers.set(reg, value),
            OpCode::_7XNN { reg, value } => {
                let original = self.registers.get(reg);
                let new = original.wrapping_add(value);
                self.registers.set(reg, new)
            }
            OpCode::_8XY0 { x, y } => {
                let y = self.registers.get(y);
                self.registers.set(x, y);
            }
            OpCode::_8XY1 { x, y } => {
                let val_x = self.registers.get(x);
                let val_y = self.registers.get(y);

                let value = val_x | val_y;
                self.registers.set(x, value);
            }
            OpCode::_8XY2 { x, y } => {
                let val_x = self.registers.get(x);
                let val_y = self.registers.get(y);

                let value = val_x & val_y;
                self.registers.set(x, value);
            }
            OpCode::_8XY3 { x, y } => {
                let val_x = self.registers.get(x);       
                let val_y = self.registers.get(y);
                
                let value = val_x ^ val_y;
                self.registers.set(x, value);
            }
            OpCode::_8XY4 { x, y } => {
                let val_x = self.registers.get(x);
                let val_y = self.registers.get(y);

                let (value, did_overflow) = val_x.overflowing_add(val_y);

                self.registers.set(x, value);
                self.registers.set(Reg::VF, did_overflow.into());
            }
            OpCode::_8XY5 { x, y } => {
                let val_x = self.registers.get(x);
                let val_y = self.registers.get(y);
                let (value, did_borrow) = val_x.overflowing_sub(val_y);

                self.registers.set(x, value);
                self.registers.set(Reg::VF, (!did_borrow).into());
            }
            OpCode::_8XY6 { x, y: _y } => {
                let val_x = self.registers.get(x);

                let lsb = 0x1 & val_x;
                self.registers.set(Reg::VF, lsb);

                let value = val_x >> 1;
                self.registers.set(x, value);
            }
            OpCode::_8XY7 { x, y } => {
                let val_x = self.registers.get(x);
                let val_y = self.registers.get(y);
                let (value, did_borrow) = val_y.overflowing_sub(val_x);

                self.registers.set(x, value);
                self.registers.set(Reg::VF, (!did_borrow).into());
            }
            OpCode::_8XYE { x, y: _y } => {
                let val_x = self.registers.get(x);

                let msb = 0x80 & val_x;
                self.registers.set(Reg::VF, msb);

                let value = val_x << 1;
                self.registers.set(x, value);
            }
            OpCode::_9XY0 { x, y } => {
                let x = self.registers.get(x);
                let y = self.registers.get(y);

                if x != y {
                    self.registers.advance_pc();
                }
            }
            OpCode::ANNN(nnn) => self.registers.set_i(nnn),
            OpCode::BNNN(nnn) => {
                let v0 = self.registers.get(Reg::V0);
                let address = nnn + (v0 as u16);

                self.registers.goto(address);
            }
            OpCode::CXNN { reg, value } => {
                let random_number: u8 = rand::thread_rng().gen();
                let value = random_number & value;
                self.registers.set(reg, value);
            }
            OpCode::DXYN { x, y, height } => {
                let x = self.registers.get(x);
                let y = self.registers.get(y);
                let i = self.registers.get_i() as usize;

                self.registers.set(Reg::VF, 0);

                for yline in 0..height {
                    let pixel = self.memory[i + (yline as usize)];
                    for xline in 0..8 {
                        let is_on = (pixel & (0x80 >> xline)) != 0;
                        let y_usize: usize = (y.wrapping_add(yline)) as usize;
                        let x_usize: usize = (x.wrapping_add(xline)) as usize;

                        let y_idx = y_usize % DISPLAY_HEIGHT;
                        let x_idx = x_usize % DISPLAY_WIDTH;

                        if is_on {
                            if self.vram[y_idx][x_idx] {
                                self.registers.set(Reg::VF, 1);
                            }
                            self.vram[y_idx][x_idx] ^= true;
                        }
                    }
                }

                self.draw_flag = true;
            }
            OpCode::EX9E(reg) => {
                let expected_key = self.registers.get(reg);
                if self.platform.key_is_pressed(&expected_key) {
                    self.registers.advance_pc();
                }
            }
            OpCode::EXA1(reg) => {
                let expected_key = self.registers.get(reg);
                if !self.platform.key_is_pressed(&expected_key) {
                    self.registers.advance_pc();
                }
            }
            OpCode::FX07(reg) => {
                let value = self.registers.get(Reg::DelayTimer);
                self.registers.set(reg, value);
            }
            OpCode::FX0A(dest_reg) => {
                let val = self.platform.await_keypress()?;
                self.registers.set(dest_reg, val);
            }
            OpCode::FX15(reg) => {
                let value = self.registers.get(reg);
                self.registers.set(Reg::DelayTimer, value);
            }
            OpCode::FX18(reg) => {
                let value = self.registers.get(reg);
                self.registers.set(Reg::SoundTimer, value)
            }
            OpCode::FX1E(reg) => {
                let i = self.registers.get_i();
                let val = self.registers.get(reg) as u16;
                self.registers.set_i(i + val);
            }
            OpCode::FX29(reg) => {
                let character = self.registers.get(reg) as usize;

                // 5 rows per character
                let offset: usize = character * 0x5;

                let sprite_addr = FONT_SET_START_ADDR + offset;

                // sanity check
                if sprite_addr >= FONT_SET_END_ADDR {
                    panic!("character was greater than 0xf");
                }

                self.registers.set_i(sprite_addr as u16);
            }
            OpCode::FX33(reg) => {
                let val = self.registers.get(reg);
                let bcd = bcd(val);
                let i = self.registers.get_i() as usize;

                self.memory[i] = bcd[0];
                self.memory[i + 1] = bcd[1];
                self.memory[i + 2] = bcd[2];
            }
            OpCode::FX55(reg) => {
                // store v0 to vreg (inclusive) into memory
                let i = self.registers.get_i() as usize;

                let start = 0x0;
                let end: usize = reg.into();

                let new_i = i + end + 1;

                for idx in start..=end {
                    let reg: Reg = (start + idx).into();
                    let val = self.registers.get(reg);
                    self.memory[i + idx] = val;
                }

                // On the original interpreter,
                // when the operation is done, I = I + X + 1
                self.registers.set_i(new_i as u16);
            }
            OpCode::FX65(reg) => {
                // fill v0 to vreg (inclusive) with values from memory
                let i = self.registers.get_i() as usize;

                let start = 0x0;
                let end: usize = reg.into();

                let new_i = i + end + 1;

                for idx in start..=end {
                    let reg: Reg = (start + idx).into();
                    let val = self.memory[i + idx];
                    self.registers.set(reg, val);
                }

                // On the original interpreter,
                // when the operation is done, I = I + X + 1
                self.registers.set_i(new_i as u16);
            }
        }

        Ok(())
    }
}
