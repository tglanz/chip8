extern crate rand;

pub type Address = u16;
pub type Byte = u8;

mod display;
mod memory;
mod registers;
mod instructions;
mod keyboard;

mod executions;

pub mod cpu;