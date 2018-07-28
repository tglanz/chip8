use { Byte };

/// Regitsers
/// 
/// Vx
///     16 registers each known as Vx, where x range from 0 to F.
///     The VF register should not be used by any program because it is used as a flag by some of the instructions.
/// 
/// I - Mainly to store memory addresses. Because the memomry space is of size 0FFF, then the top half byte is 0
/// 
/// Sound and Delay timers (Dont know what currently)
/// 
/// PC -
///     Program couter, points to the currently executed command
/// 
/// SP - 
///     Stack Pointer, points to the top of the stack
pub struct Registers {
    pub vs: [Byte; 0xF],
    pub i: u16,              
    pub sound_timer: Byte,
    pub delay_timer: Byte,
    pub program_counter: u16,
    pub stack_pointer: u8,
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            vs: [0; 0xF],
            i: 0,
            delay_timer: 0,
            sound_timer: 0,
            stack_pointer: 0,
            program_counter: 0,
        }
    }
}