extern crate sdl2;

use qcpu::{OpArgs, QCPU};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::IntegerOrSdlError;
use sdl2::Sdl;

pub struct Optron {
    pub context: Sdl,
    pub canvas: Result<Canvas<Window>, IntegerOrSdlError>,
    pub texture: Option<Texture>,
    pub value: i16,
}

impl Optron {
    pub fn new() -> Optron {
        Optron {
            context: sdl2::init().unwrap(),
            canvas: Err(IntegerOrSdlError::SdlError("".to_string())),
            texture: None,
            value: 69,
        }
    }

    pub fn closure_test<'a>(&mut self) -> Box<dyn FnMut(&mut QCPU<'a>, &OpArgs) -> () + '_> {
        Box::new(move |x, y| print!("hello, {}", self.value))
    }

    pub fn set_pixel(&mut self, x: usize, y: usize) {
        match &mut self.texture {
            Some(texture) => {
                texture
                    .with_lock(None, |buffer: &mut [u8], pitch: usize| {
                        let offset = y * pitch + x * 3;
                        buffer[offset] = 255;
                        buffer[offset + 1] = 0;
                        buffer[offset + 2] = 0;
                    })
                    .ok();
            }
            _ => unreachable!(),
        }
    }

    pub fn try_create_texture(creator: &TextureCreator<WindowContext>) -> Result<Texture, String> {
        let mut texture = creator
            .create_texture_streaming(PixelFormatEnum::RGB24, 256, 256)
            .map_err(|e| e.to_string())?;
        texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..256 {
                for x in 0..256 {
                    let offset = y * pitch + x * 3;
                    buffer[offset] = x as u8;
                    buffer[offset + 1] = y as u8;
                    buffer[offset + 2] = 0;
                }
            }
        })?;
        return Ok(texture);
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
            Ok(canvas) => {
                canvas.set_draw_color(Color::RGB(0, 255, 255));
                let creator = canvas.texture_creator();
                self.texture = Some(Optron::try_create_texture(&creator).unwrap());
                canvas.clear();
                canvas
                    .copy(
                        &self.texture.as_ref().unwrap(),
                        None,
                        Some(Rect::new(100, 100, 256, 256)),
                    )
                    .ok();
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
            Ok(canvas) => {
                Optron::clear(canvas);
                Optron::render();
                canvas.clear();
                canvas
                    .copy(
                        &self.texture.as_ref().unwrap(),
                        None,
                        Some(Rect::new(100, 100, 256, 256)),
                    )
                    .ok();
                Optron::present(canvas);
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
