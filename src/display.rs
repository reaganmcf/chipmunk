use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use std::time::Duration;

pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_WIDTH: usize = 64;
// TODO bool should be replaced with u8's and bitwise ops
pub type VRAM = [[bool; DISPLAY_WIDTH]; DISPLAY_HEIGHT];

const SCALE: usize = 10;

pub struct Display {
    canvas: Canvas<sdl2::video::Window>,
}

impl Display {
    pub fn new(context: &mut sdl2::Sdl) -> Self {
        let video_subsystem = context.video().unwrap();

        let window = video_subsystem
            .window(
                "chipmunk",
                (DISPLAY_WIDTH * SCALE) as u32,
                (DISPLAY_HEIGHT * SCALE) as u32,
            )
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Self { canvas }
    }

    // TODO: collision detection
    // TODO: flickering on WSL?
    // TODO: performance opts?
    pub fn draw(&mut self, vram: VRAM) {
        self.clear();

        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for j in 0..DISPLAY_HEIGHT {
            for i in 0..DISPLAY_WIDTH {
                if vram[j][i] == true {
                    self.canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    self.canvas.set_draw_color(Color::RGB(0, 0, 0));
                }

                self.canvas
                    .fill_rect(Rect::new(
                        (i * SCALE) as i32,
                        (j * SCALE) as i32,
                        SCALE as u32,
                        SCALE as u32,
                    ))
                    .expect("failed to draw!");
            }
        }

        self.canvas.present();
    }

    pub fn clear(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.present();
    }
}
