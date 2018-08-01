use arch::{Renderer};
use arch::display::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH};
use std::string::String;

use opengl_graphics::{ GlGraphics, OpenGL };

pub struct OpenGLRenderer {
    context: u32,
    draw_width: u32,
    draw_height: 32,
    pixels: [ bool; RESOLUTION_HEIGHT * RESOLUTION_WIDTH ]
}

impl OpenGLRenderer {
    pub fn new(draw_width: u32, draw_height: u32) -> ConsoleRenderer {
        ConsoleRenderer {
            context: OpenGL::V3_2,
            draw_width,
            draw_height,
            pixels: [ false; RESOLUTION_HEIGHT * RESOLUTION_WIDTH ]
        }
    }
}

impl Renderer for OpenGLRenderer {
    fn pixel(&mut self, x: usize, y: usize, is_set: bool) {
        self.pixels[x + (y * RESOLUTION_WIDTH)] = is_set;
    }

    fn flush(&mut self) {

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.context.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);
        });

        /*

        let mut string = String::with_capacity(RESOLUTION_HEIGHT * RESOLUTION_WIDTH + RESOLUTION_HEIGHT);

        for rowdex in 0..RESOLUTION_HEIGHT {
            let offset = rowdex * RESOLUTION_WIDTH;
            for coldex in 0..RESOLUTION_WIDTH {
                if self.pixels[offset + coldex] {
                    string.push_str("#");
                } else {
                    string.push_str(" ");
                }
            }

            string.push_str("\n");
        }

        println!("{}", string);
        */
    }
}