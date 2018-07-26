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
struct Registers {
    v_registers: [Byte; 0xF],
    i_register: u16,              

    // Sound
    // Delay

    program_counter: u16,
    stack_pointer: u8,
}