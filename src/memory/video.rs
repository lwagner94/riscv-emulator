use super::addressspace::Address;
use super::addressspace::MemoryDevice;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormat, PixelFormatEnum};
use sdl2::render::{Canvas, Texture};
use sdl2::video::Window;
use sdl2::rect::Rect;
use std::process;
use std::thread;
use std::time::Duration;

use std::sync::{Arc, Mutex};

const SIZE_X: u32 = 800;
const SIZE_Y: u32 = 600;

pub struct Video {
    offset: Address,
    framebuffer: Arc<Mutex<Vec<u8>>>
}

impl MemoryDevice for Video {
    fn read_byte(&self, _address: Address) -> u8 {
        unimplemented!();
    }
    fn read_halfword(&self, _address: Address) -> u16 {
        unimplemented!();
    }
    fn read_word(&self, _address: Address) -> u32 {
        unimplemented!();
    }

    fn write_byte(&mut self, _address: Address, _val: u8) {
        let mut framebuffer = self.framebuffer.lock().unwrap();
        let relative_address = self.get_relative_address(_address) as usize;

        framebuffer[relative_address] = _val;
    }
    fn write_halfword(&mut self, _address: Address, _val: u16) {
        unimplemented!();
    }
    fn write_word(&mut self, _address: Address, _val: u32) {
        unimplemented!();
    }

    fn offset(&self) -> Address {
        self.offset
    }
}

impl Video {
    pub fn new(offset: Address) -> Video {
        const VEC_SIZE: u32 = SIZE_X * SIZE_Y * 3;

        let data = Arc::new(Mutex::new(vec![0u8; VEC_SIZE as usize]));

        let data_clone = Arc::clone(&data);

        thread::spawn(move || {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem
                .window("RISCV Emulator", SIZE_X, SIZE_Y)
                .position_centered()
                .build()
                .unwrap();

            let mut canvas = window.into_canvas().build().unwrap();

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            canvas.present();

            let texture_creator = canvas.texture_creator();

            let mut texture: Texture = texture_creator.create_texture_streaming(
                Some(PixelFormatEnum::RGB24), SIZE_X, SIZE_Y).unwrap();

            let mut event_pump = sdl_context.event_pump().unwrap();
            let mut i = 0;
            'running: loop {
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. }
                        | Event::KeyDown {
                            keycode: Some(Keycode::Escape),
                            ..
                        } => {
                            process::exit(0);
                        }
                        _ => {}
                    }
                }
                // The rest of the game loop goes here...
                let data = data_clone.lock().unwrap();



                texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
                    buffer.copy_from_slice(data.as_slice());

                }).unwrap();
                canvas.clear();
                canvas.copy(&texture, None, Some(Rect::new(0, 0, SIZE_X, SIZE_Y))).unwrap();




                canvas.present();
                ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        });

        Video {
            offset,
            framebuffer: data
        }
    }
}
