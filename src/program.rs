use std::fs::File;
use std::io::Read;

use opengl_graphics::{ OpenGL, GlGraphics };
use glutin_window::GlutinWindow;

use piston::window::WindowSettings;
use piston::input::{RenderEvent, RenderArgs, UpdateEvent, IdleEvent, PressEvent, ReleaseEvent, Button, Key};
use piston::event_loop::*;

use graphics::*;

use arch::cpu::Cpu;

struct Pixel {
    x: usize,
    y: usize,
    is_set: bool
}

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

pub struct Program {
    cpu: Cpu,
    window: GlutinWindow,
    opengl: GlGraphics,
}

impl Program {
    pub fn new(program_path: &str) -> Program {
        let mut program_data = Vec::new();
        let mut file = File::open(program_path).expect("Unable to find program file");
        file.read_to_end(&mut program_data).expect("Failed to read program");
        
        let opengl_spec = OpenGL::V3_2;
        
        let window = WindowSettings::new(
                    "chip8",
                    [800, 600]
                )
                .opengl(opengl_spec)
                .exit_on_esc(true)
                .build()
                .unwrap();

        let opengl = GlGraphics::new(opengl_spec);

        Program {
            cpu: Cpu::new(&program_data),
            window,
            opengl,

        }
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());

        while let Some(e) = events.next(&mut self.window) {

            e.render(|args| {
                self.render(args);
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

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let pixels = self.cpu.get_display().temp();

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        let square = rectangle::square(0.0, 0.0, 1.0);

        self.opengl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);
            for pixel in pixels {
                if pixel.2 {
                    let x = (pixel.0 as f64);
                    let y = (pixel.1 as f64);
                    let transform = c.transform.trans(x, y);
                    // Draw a box rotating around the middle of the screen.
                    rectangle(RED, square, transform, gl);
                }
                
            }
        });
        
    }
}