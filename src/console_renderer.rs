use arch::{Renderer};
use arch::display::{RESOLUTION_HEIGHT, RESOLUTION_WIDTH};
use std::string::String;

pub struct ConsoleRenderer {
    pixels: [ bool; RESOLUTION_HEIGHT * RESOLUTION_WIDTH ]
}

impl ConsoleRenderer {
    pub fn new() -> ConsoleRenderer {
        ConsoleRenderer {
            pixels: [ false; RESOLUTION_HEIGHT * RESOLUTION_WIDTH ]
        }
    }
}

impl Renderer for ConsoleRenderer {
    fn pixel(&mut self, x: usize, y: usize, is_set: bool) {
        self.pixels[x + (y * RESOLUTION_WIDTH)] = is_set;
    }

    fn flush(&mut self) {
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
    }
}