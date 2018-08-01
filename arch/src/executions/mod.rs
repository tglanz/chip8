use rand;

use {Address, Byte};
use memory::{Memory, FONT_OFFSET};
use registers::{Registers};
use instructions::{Instruction};
use display::{Display};
use keyboard::{Keyboard};
use cpu::{Stack};

use instructions::Instruction::*;

pub fn execute_instruction(
    instruction: Instruction,
    memory: &mut Memory,
    registers: &mut Registers,
    stack: &mut Stack,
    display: &mut Display,
    keyboard: &mut Keyboard,
) {
    match instruction {
        SYS { address } => {
            // Doc says modern interperters ignore, so ignore
        },
        CLS => {
            display.clear();
            registers.program_counter += 2;
        },
        RTS => {
            registers.program_counter =
                stack[registers.stack_pointer as usize];
            registers.stack_pointer -= 1;
        },
        JMP { address } => {
            registers.program_counter = address;
        },
        JSR { address } => {
            registers.stack_pointer += 1;
            stack[registers.stack_pointer as usize] = registers.program_counter;
            registers.program_counter = address;
        },
        SE { reg_id, value } => {
            if registers.vs[reg_id as usize] == value {
                registers.program_counter += 4;
            } else {
                registers.program_counter += 2;
            }
        },
        SNE { reg_id, value } => {
            if registers.vs[reg_id as usize] != value {
                registers.program_counter += 4;
            } else {
                registers.program_counter += 2;
            }
        },
        SEXY { x_reg_id, y_reg_id } => {
            let x = registers.vs[x_reg_id as usize];
            let y = registers.vs[y_reg_id as usize];
            if x == y {
                registers.program_counter += 4;
            } else {
                registers.program_counter += 2;
            }
        },
        MOV { reg_id, value } => {
            registers.vs[reg_id as usize] = value;
            registers.program_counter += 2;
        },
        ADD { reg_id, value } => {
            let res = registers.vs[reg_id as usize] as u16 + value as u16;
            registers.vs[reg_id as usize] = res as u8;
            registers.program_counter += 2;
        },
        MOVXY { x_reg_id, y_reg_id } => {
            registers.vs[x_reg_id as usize] = registers.vs[y_reg_id as usize];
            registers.program_counter += 2;
        },
        ORXY { x_reg_id, y_reg_id } => {
            registers.vs[x_reg_id as usize] |= registers.vs[y_reg_id as usize];
            registers.program_counter += 2;
        },
        ANDXY { x_reg_id, y_reg_id } => {
            registers.vs[x_reg_id as usize] &= registers.vs[y_reg_id as usize];
            registers.program_counter += 2;
        },
        XORXY { x_reg_id, y_reg_id } => {
            registers.vs[x_reg_id as usize] ^= registers.vs[y_reg_id as usize];
            registers.program_counter += 2;
        },
        ADDXY { x_reg_id, y_reg_id } => {
            let x = registers.vs[x_reg_id as usize];
            let y = registers.vs[y_reg_id as usize];
            let sum = (x as u16) + (y as u16);
            if sum > 255 {
                registers.vs[0x0F] = 1;
            }
            registers.vs[x_reg_id as usize] = sum as Byte;
            registers.program_counter += 2;
        },
        SUBXY { x_reg_id, y_reg_id } => {
            let x = registers.vs[x_reg_id as usize];
            let y = registers.vs[y_reg_id as usize];
            if x < y {
                registers.vs[0x0F] = 1;
            }
            registers.vs[x_reg_id as usize] = x - y;
            registers.program_counter += 2;
        },
        SHR { reg_id } => {
            let v = registers.vs[reg_id as usize];
            registers.vs[0x0F] = v & 0x01;
            registers.vs[reg_id as usize] >>= 1;
            registers.program_counter += 2;
        },
        RSUBXY { x_reg_id, y_reg_id } => {
            let x = registers.vs[x_reg_id as usize];
            let y = registers.vs[y_reg_id as usize];
            if y < x {
                registers.vs[0x0F] = 1;
            }
            registers.vs[x_reg_id as usize] = y - x;
            registers.program_counter += 2;
        },
        SHL { reg_id } => {
            let v = registers.vs[reg_id as usize];
            registers.vs[0x0F] = (v & 0x80) >> 7;
            registers.vs[reg_id as usize] <<= 1;
            registers.program_counter += 2;
        },
        SNEXY { x_reg_id, y_reg_id } => {
            let x = registers.vs[x_reg_id as usize];
            let y = registers.vs[y_reg_id as usize];
            if x != y {
                registers.program_counter += 4;
            } else {
                registers.program_counter += 2;
            }
        },
        MOVI { address } => {
            registers.i = address;
            registers.program_counter += 2;
        },
        JMI { address } => {
            registers.program_counter = (registers.vs[0] as u16) + address;
        },
        RAND { reg_id, value } => {
            let random_byte = rand::prelude::random::<Byte>();
            registers.vs[reg_id as usize] = random_byte + value;
            registers.program_counter += 2;
        },
        DRW { x_reg_id, y_reg_id, value } => {
            let mut sprite = Vec::with_capacity(value as usize);
            for i in 0..value as u16 {
                let location = (registers.i + i) as usize;
                sprite.push(memory.data[location]);
            }
            let x = registers.vs[x_reg_id as usize] as usize;
            let y = registers.vs[y_reg_id as usize] as usize;
            let did_flip = display.set_sprite(x, y, &sprite);
            registers.vs[0x0F] = if did_flip { 1 } else { 0 };
            registers.program_counter += 2;
        },
        SKP { reg_id } => {
            let key = registers.vs[reg_id as usize];
            if keyboard.is_pressed(key) {
                registers.program_counter += 4;
            } else {
                registers.program_counter += 2;
            }
        },
        SKNP { reg_id } => {
            let key = registers.vs[reg_id as usize];
            if keyboard.is_pressed(key) {
                registers.program_counter += 2;
            } else {
                registers.program_counter += 4;
            }
        },
        GDELAY { reg_id } => {
            registers.vs[reg_id as usize] = registers.delay_timer;
            registers.program_counter += 2;
        },
        KEY { reg_id } => {
            let value = registers.vs[reg_id as usize];
            
            let units = value % 10;
            let tens = (value / 10) % 10;
            let hundreds = (value / 100) % 10;

            memory.data[registers.i as usize] = hundreds;
            memory.data[registers.i as usize + 1] = tens;
            memory.data[registers.i as usize + 2] = units;
        },
        SDELAY { reg_id } => {
            registers.delay_timer = registers.vs[reg_id as usize];
            registers.program_counter += 2;
        },
        SSOUND { reg_id } => {
            registers.sound_timer = registers.vs[reg_id as usize];
            registers.program_counter += 2;
        },
        ADI { reg_id } => {
            registers.i += registers.vs[reg_id as usize] as u16;
            registers.program_counter += 2;
        },
        FONT { reg_id } => {
            let v = registers.vs[reg_id as usize];
            registers.i = FONT_OFFSET as u16 + v as u16;
        },
        BCD { reg_id } => {
            panic!("BCD not implemented");
        },
        STR { reg_id } => {
            let initial_location = registers.i as usize;
            for curr_reg_id in 0..reg_id as usize {
                let value = registers.vs[curr_reg_id];
                memory.data[initial_location + curr_reg_id] = value;
            }
            registers.i += reg_id as u16 + 1;
        },
        LDR { reg_id } => {
            let initial_location = registers.i as usize;
            for curr_reg_id in 0..reg_id as usize {
                let data = memory.data[initial_location + curr_reg_id];
                registers.vs[curr_reg_id] = data;
            }
            registers.i += reg_id as u16 + 1;
        },
    };
}