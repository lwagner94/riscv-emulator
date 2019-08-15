use super::addressspace::Address;
use super::addressspace::MemoryDevice;
use crate::util;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::Texture;
use std::process;
use std::ptr::copy_nonoverlapping;
use std::thread;
use std::time::Duration;

const SIZE_X: u32 = 800;
const SIZE_Y: u32 = 600;
const VEC_SIZE: usize = (SIZE_X * SIZE_Y * 4) as usize;

static mut FRAMEBUFFER: [u8; VEC_SIZE] = [0; VEC_SIZE];

pub struct Video {
    offset: Address,
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
        let relative_address = self.get_relative_address(_address) as usize;
        //        self.framebuffer[relative_address] = _val;
        unsafe {
            FRAMEBUFFER[relative_address] = _val;
        }
    }
    fn write_halfword(&mut self, _address: Address, _val: u16) {
        unimplemented!();
    }
    fn write_word(&mut self, _address: Address, _val: u32) {
        let relative_address = self.get_relative_address(_address) as usize;
        //        self.framebuffer[relative_address] = _val;
        unsafe {
//            FRAMEBUFFER[relative_address] = _val;
            util::write_u32_to_byteslice(&mut FRAMEBUFFER[relative_address..relative_address+4], _val);
        }
    }

    fn offset(&self) -> Address {
        self.offset
    }
}

impl Video {
    pub fn new(offset: Address) -> Video {
        #[cfg(not(test))]
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

            let mut texture: Texture = texture_creator
                .create_texture_streaming(Some(PixelFormatEnum::RGBA8888), SIZE_X, SIZE_Y)
                .unwrap();

            let mut event_pump = sdl_context.event_pump().unwrap();

            loop {
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

                texture
                    .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                        let dest_ptr = buffer.as_mut_ptr();
                        unsafe {
                            copy_nonoverlapping(FRAMEBUFFER.as_ptr(), dest_ptr, VEC_SIZE);
                        }
                    })
                    .unwrap();
                canvas.clear();
                canvas
                    .copy(&texture, None, Some(Rect::new(0, 0, SIZE_X, SIZE_Y)))
                    .unwrap();

                canvas.present();
                thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        });

        Video { offset }
    }
}
