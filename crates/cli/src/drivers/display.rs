#![allow(clippy::needless_range_loop)]

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;

use emulator::{DISPLAY_HEIGHT, DISPLAY_WIDTH, Vram};

const SCALE: usize = 10;

pub struct Sdl2Display {
    canvas: Canvas<sdl2::video::Window>,
}

impl Sdl2Display {
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

    // TODO: performance opts?
    pub fn draw(&mut self, vram: Vram) {
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        for j in 0..DISPLAY_HEIGHT {
            for i in 0..DISPLAY_WIDTH {
                if vram[j][i] {
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
}
