use super::addressspace::Address;
use super::addressspace::MemoryDevice;
use crate::util;

#[cfg(feature = "framebuffer")]
use sdl2::event::Event;

#[cfg(feature = "framebuffer")]
use sdl2::pixels::PixelFormatEnum;

#[cfg(feature = "framebuffer")]
use sdl2::rect::Rect;

#[cfg(feature = "framebuffer")]
use sdl2::render::Texture;

use std::cell::UnsafeCell;
use std::process;
use std::ptr::copy_nonoverlapping;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const SIZE_X: u32 = 800;
const SIZE_Y: u32 = 600;
const VEC_SIZE: usize = (SIZE_X * SIZE_Y * 4) as usize;

const MEGABYTE: usize = 1 << 20;
const FRAMEBUFFER_START: usize = MEGABYTE * 0;
const FRAMEBUFFER_END: usize = MEGABYTE * 2 - 1;
const KEYBUFFER_START: usize = MEGABYTE * 2;
const KEYBUFFER_END: usize = KEYBUFFER_START + 7;

pub struct Video {
    offset: Address,
    shared_context: Arc<SharedVideoContext>,
}

struct SharedVideoContext {
    framebuffer: UnsafeCell<Vec<u8>>,
    interrupt_flags: Arc<AtomicU32>,
    keybuffer: UnsafeCell<Vec<u8>>,
}

unsafe impl Sync for SharedVideoContext {}

impl SharedVideoContext {
    fn new(interrupt_flags: Arc<AtomicU32>) -> Self {
        Self {
            framebuffer: UnsafeCell::new(vec![0u8; VEC_SIZE]),
            interrupt_flags,
            keybuffer: UnsafeCell::new(vec![0u8; 2 * 4]),
        }
    }

    fn get_framebuffer(&self) -> *mut Vec<u8> {
        self.framebuffer.get()
    }
}

impl MemoryDevice for Video {
    fn read_byte(&self, _address: Address) -> u8 {
        unimplemented!();
    }
    fn read_halfword(&self, _address: Address) -> u16 {
        unimplemented!();
    }
    fn read_word(&self, _address: Address) -> u32 {
        let relative_address = self.get_relative_address(_address) as usize;

        match relative_address {
            KEYBUFFER_START...KEYBUFFER_END => {
                let keybuffer_address = relative_address - KEYBUFFER_START;

                let keybuffer_ref;
                unsafe {
                    keybuffer_ref = &mut *self.shared_context.keybuffer.get();
                }
                util::read_u32_from_byteslice(
                    &keybuffer_ref[keybuffer_address..keybuffer_address + 4],
                )
            }
            _ => panic!("Invalid memory access at {:x}", relative_address),
        }
    }

    fn write_byte(&mut self, _address: Address, _val: u8) {
        let relative_address = self.get_relative_address(_address) as usize;
        let framebuffer_ref;
        unsafe {
            framebuffer_ref = self.shared_context.get_framebuffer().as_mut().unwrap();
        }
        framebuffer_ref[relative_address] = _val;
    }
    fn write_halfword(&mut self, _address: Address, _val: u16) {
        let relative_address = self.get_relative_address(_address) as usize;
        let framebuffer_ref;

        unsafe {
            framebuffer_ref = self.shared_context.get_framebuffer().as_mut().unwrap();
        }
        util::write_u16_to_byteslice(
            &mut framebuffer_ref[relative_address..relative_address + 2],
            _val,
        )
    }
    fn write_word(&mut self, _address: Address, _val: u32) {
        let relative_address = self.get_relative_address(_address) as usize;
        let framebuffer_ref;

        unsafe {
            framebuffer_ref = self.shared_context.get_framebuffer().as_mut().unwrap();
        }
        util::write_u32_to_byteslice(
            &mut framebuffer_ref[relative_address..relative_address + 4],
            _val,
        )
    }

    fn offset(&self) -> Address {
        self.offset
    }

    fn check_for_interrupt(&mut self) -> Option<Address> {
        None
    }
}

impl Video {
    pub fn new(offset: Address, interrupt_flags: Arc<AtomicU32>) -> Video {
        let context = Arc::new(SharedVideoContext::new(interrupt_flags));
        let context_clone = context.clone();

        Video::start_render_thread(context_clone);

        Video {
            offset,
            shared_context: context,
        }
    }

    #[cfg(not(feature = "framebuffer"))]
    fn start_render_thread(context: Arc<SharedVideoContext>) {}

    #[cfg(feature = "framebuffer")]
    fn start_render_thread(context: Arc<SharedVideoContext>) {
        let func = move || {
            let sdl_context = sdl2::init().unwrap();
            let video_subsystem = sdl_context.video().unwrap();

            let window = video_subsystem
                .window("RISCV Emulator", SIZE_X, SIZE_Y)
                .position_centered()
                .build()
                .unwrap();

            let mut canvas = window.into_canvas().build().unwrap();

            let texture_creator = canvas.texture_creator();

            let mut texture: Texture = texture_creator
                .create_texture_streaming(Some(PixelFormatEnum::RGBA8888), SIZE_X, SIZE_Y)
                .unwrap();

            let mut event_pump = sdl_context.event_pump().unwrap();

            loop {
                let keybuffer;
                unsafe {
                    keybuffer = &mut *context.keybuffer.get();
                }
                for event in event_pump.poll_iter() {
                    match event {
                        Event::Quit { .. } => process::exit(0),
                        Event::KeyDown {
                            keycode: Some(code),
                            ..
                        } => {
                            util::write_u32_to_byteslice(&mut keybuffer[0..4], 1);
                            util::write_u32_to_byteslice(&mut keybuffer[4..8], code as u32);

                            context.interrupt_flags.store(0x01, Ordering::SeqCst);
                        }
                        Event::KeyUp {
                            keycode: Some(code),
                            ..
                        } => {
                            util::write_u32_to_byteslice(&mut keybuffer[0..4], 0);
                            util::write_u32_to_byteslice(&mut keybuffer[4..8], code as u32);
                            context.interrupt_flags.store(0x01, Ordering::SeqCst);
                        }
                        _ => {}
                    }
                }

                texture
                    .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                        let dest_ptr = buffer.as_mut_ptr();

                        unsafe {
                            let framebuffer_vec = &*context.get_framebuffer();
                            copy_nonoverlapping(framebuffer_vec.as_ptr(), dest_ptr, VEC_SIZE);
                        }
                    })
                    .unwrap();
                canvas
                    .copy(&texture, None, Some(Rect::new(0, 0, SIZE_X, SIZE_Y)))
                    .unwrap();

                canvas.present();
                thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
            }
        };

        #[cfg(not(test))]
        thread::spawn(func);
    }
}
