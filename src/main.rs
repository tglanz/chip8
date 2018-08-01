extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

extern crate arch;

mod program;
mod opengl_window_provider;

use opengl_window_provider::OpenGLWindowProvider;
use program::Program;

fn main() {

    let mut program = Program::new::<OpenGLWindowProvider>("c:/tmp/prog.chp8", OpenGLWindowProvider::new());

    println!("Startign");
    program.run();
}