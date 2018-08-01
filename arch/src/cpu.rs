use {Address, Byte, Renderer};
use memory::{Memory, PROGRAM_OFFSET};
use registers::{Registers};
use instructions::{Instruction};
use display::{Display};
use keyboard::{Keyboard};

use executions::execute_instruction;

const STACK_SIZE: usize = 0x0F;
pub type Stack = [Address; STACK_SIZE];

pub struct Cpu {
    memory: Memory,
    registers: Registers,
    display: Display,
    keyboard: Keyboard,
    stack: Stack
}

impl Cpu {
    pub fn new(program_data: &Vec<Byte>) -> Cpu {
        let mut registers = Registers::new();
        registers.program_counter = PROGRAM_OFFSET as u16;

        Cpu {
            memory: Memory::new(program_data),
            registers,
            display: Display::new(),
            keyboard: Keyboard::new(),
            stack: [0; STACK_SIZE],
        }
    }

    pub fn tick(&mut self) {
        let high = self.memory.data[self.registers.program_counter as usize];
        let low = self.memory.data[self.registers.program_counter as usize + 1];
        let instruction_code = ((high as u16) << 8) + low as u16;
        self.execute(instruction_code);
    }

    fn execute(&mut self, instruction_code: u16) {
        match Instruction::parse_code(instruction_code) {
            Some(instruction) => execute_instruction(
                instruction,
                &mut self.memory,
                &mut self.registers,
                &mut self.stack,
                &mut self.display,
                &mut self.keyboard,
            ),
            None => {
                panic!("Unable to parse instruction: {:X?}", instruction_code);
            },
        }
    }

    pub fn released_key(&mut self, key: Byte) {
        self.keyboard.set_released(key);
    }

    pub fn pressed_key(&mut self, key: Byte) {
        self.keyboard.set_pressed(key);
    }

    pub fn render_pass(&self, renderer: &mut Renderer) {
        self.display.traverse(|x, y, pixel| renderer.pixel(x, y, pixel));
        renderer.flush();
    }
}