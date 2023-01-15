use emulator::{EmulatorError, Platform, Vram};
use wasm_bindgen::prelude::*;

static HELLO_WORLD_ROM: &[u8] = &[
    0x62, 0x78, 0xa5, 0x0, 0x63, 0x1, 0x64, 0x1, 0xf1, 0xa, 0x0, 0xe0, 0xf2, 0x18, 0xf1, 0x29,
    0xd3, 0x45, 0x12, 0x0,
];

#[wasm_bindgen]
extern {
    pub fn CHIPMUNK_draw(vram: &[u8]);
    pub fn CHIPMUNK_startBeep();
    pub fn CHIPMUNK_stopBeep();
    pub fn CHIPMUNK_scanKeys();
    pub fn CHIPMUNK_keyIsPressed(key: u8) -> bool;
    pub fn CHIPMUNK_shouldQuit() -> bool;
    pub fn CHIPMUNK_awaitKeypress() -> u8;
}

struct JsPlatform();

impl Platform for JsPlatform {
    fn draw(&mut self, vram: Vram) {
        // flatten 2d array to 1d
        let flattened: Vec<u8> = vram
            .iter()
            .flat_map(|arr| arr.iter().map(|b| *b as u8))
            .collect();
        let slice = flattened.as_slice();
        CHIPMUNK_draw(slice);
    }

    fn start_beep(&mut self) {
        CHIPMUNK_startBeep();
    }

    fn stop_beep(&mut self) {
        CHIPMUNK_stopBeep();
    }

    fn scan_keys(&mut self) {
        CHIPMUNK_scanKeys();
    }

    fn key_is_pressed(&self, key: &u8) -> bool {
        CHIPMUNK_keyIsPressed(*key)
    }

    fn should_quit(&mut self) -> bool {
        CHIPMUNK_shouldQuit()
    }

    fn await_keypress(&mut self) -> Result<u8, emulator::EmulatorError> {
        match CHIPMUNK_awaitKeypress() {
            0xFF => Err(EmulatorError::Exit),
            key => Ok(key),
        }
    }
}

#[wasm_bindgen]
pub fn start() {
    let platform = Box::new(JsPlatform());
    emulator::run(HELLO_WORLD_ROM.to_vec(), platform, false);
}
