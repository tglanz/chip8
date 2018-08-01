extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

extern crate arch;

mod program;

use program::Program;

fn main() {
    Program::new("c:/tmp/prog.chp8").run();
}