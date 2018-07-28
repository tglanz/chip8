extern crate arch;

mod console_renderer;

use std::io::*;
use std::fs::File;
use std::{thread, time};

use arch::cpu::Cpu;

use console_renderer::ConsoleRenderer;

fn main() {
    let mut renderer = ConsoleRenderer::new();

    let mut program_data = Vec::new();
    let mut file = File::open("d:/tmp/prog.ch8").expect("Unable to find program file");
    file.read_to_end(&mut program_data).expect("Failed to read program");
    let mut cpu = Cpu::new(&program_data);
    
    loop {
        cpu.tick();
        cpu.render_pass(&mut renderer);
        thread::sleep(time::Duration::from_secs(1/30));
    }
}