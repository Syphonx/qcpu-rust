extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::IntegerOrSdlError;
use sdl2::Sdl;

pub struct Optron {
    pub context: Sdl,
    pub canvas: Result<Canvas<Window>, IntegerOrSdlError>,
}

impl Optron {
    pub fn new() -> Optron {
        Optron {
            context: sdl2::init().unwrap(),
            canvas: Err(IntegerOrSdlError::SdlError("".to_string())),
        }
    }

    pub fn init(&mut self) {
        let video_subsystem = self.context.video().unwrap();
        let window = video_subsystem
            .window("qcpu optron", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        self.canvas = window.into_canvas().build();
        match &mut self.canvas {
            Ok(value) => {
                value.set_draw_color(Color::RGB(0, 255, 255));
                value.clear();
                value.present();
            }
            _ => unreachable!(),
        }
    }

    pub fn pump(&mut self) -> bool {
        let mut event_pump = self.context.event_pump().unwrap();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    return false;
                }
                _ => {}
            }
        }
        return true;
    }

    pub fn display(&mut self) {
        match &mut self.canvas {
            Ok(value) => {
                Optron::clear(value);
                Optron::render();
                Optron::present(value);
            }
            _ => unreachable!(),
        }
    }

    fn clear(canvas: &mut Canvas<sdl2::video::Window>) {
        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();
    }

    fn render() {
        // The rest of the game loop goes here..
    }

    fn present(canvas: &mut Canvas<sdl2::video::Window>) {
        canvas.present();
    }
}
