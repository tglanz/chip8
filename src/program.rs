use std::fs::File;
use std::io::Read;

use glutin_window::GlutinWindow;

use piston::input::{RenderEvent, UpdateEvent, IdleEvent, PressEvent, ReleaseEvent, Button, Key};
use piston::event_loop::*;

use arch::cpu::Cpu;

fn match_key_to_cpu(key: Key) -> Option<u8> {
    match key {
        Key::Q => Some(0x00),
        Key::W => Some(0x01),
        Key::E => Some(0x02),
        Key::R => Some(0x03),
        Key::A => Some(0x04),
        Key::S => Some(0x05),
        Key::D => Some(0x06),
        Key::F => Some(0x07),
        Key::Y => Some(0x08),
        Key::U => Some(0x09),
        Key::I => Some(0x0A),
        Key::O => Some(0x0B),
        Key::H => Some(0x0C),
        Key::J => Some(0x0D),
        Key::K => Some(0x0E),
        Key::L => Some(0x0F),
        _ => None,
    }
}

pub trait WindowProvider {
    fn into_window(self) -> GlutinWindow;
}

pub struct Program {
    cpu: Cpu,
    window: GlutinWindow,
}

impl Program {
    pub fn new<T>(program_path: &str, window_provider: T) -> Program
        where T: WindowProvider {
        let mut program_data = Vec::new();
        let mut file = File::open(program_path).expect("Unable to find program file");
        file.read_to_end(&mut program_data).expect("Failed to read program");
        let mut cpu = Cpu::new(&program_data);

        let mut window = window_provider.into_window();

        Program {
            cpu,
            window
        }
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());

        while let Some(e) = events.next(&mut self.window) {

            e.render(|args| {
                self.render(args.draw_width, args.draw_height);
            });

            e.update(|args| {
                self.update(args.dt);
            });

            e.idle(|args| {
            });

            if let Some(Button::Keyboard(key)) = e.release_args() {
                if let Some(cpu_key) = match_key_to_cpu(key) {
                    self.cpu.released_key(cpu_key);
                }
            }

            if let Some(Button::Keyboard(key)) = e.press_args() {
                if let Some(cpu_key) = match_key_to_cpu(key) {
                    self.cpu.pressed_key(cpu_key);
                }
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.cpu.tick();
    }

    pub fn render(&mut self, width: u32, height: u32) {
    }
}