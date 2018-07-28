extern crate rand;

pub type Address = u16;
pub type Byte = u8;

pub trait Renderer {
    fn pixel(&mut self, x: usize, y: usize, is_set: bool);
    fn flush(&mut self);
}

mod memory;
mod registers;
mod instructions;
mod keyboard;

mod executions;

pub mod display;
pub mod cpu;