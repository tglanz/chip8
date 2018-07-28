use {Address, Byte};
use memory::{Memory};
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
        Cpu {
            memory: Memory::new(program_data),
            registers: Registers::new(),
            display: Display::new(),
            keyboard: Keyboard::new(),
            stack: [0; STACK_SIZE],
        }
    }

    pub fn execute(&mut self, instruction_code: u16) {
        match Instruction::parse_code(instruction_code) {
            Some(instruction) => execute_instruction(
                instruction,
                &mut self.memory,
                &mut self.registers,
                &mut self.stack,
                &mut self.display,
                &mut self.keyboard,
            ),
            None => {},
        }
    }
}