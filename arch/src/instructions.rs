use {Address, Byte};

pub enum Instruction {

    /// Jump to a machine code routin at address
    ///  - ignored by modern interperters
    SYS { address: Address },

    /// Clear the display
    CLS,

    /// Return from a subroutine
    ///  - set program counter to the top of the stack
    ///  - decrement the stack pointer
    RTS,

    /// Jump to location at adress
    JMP { address: Address },

    /// Jump to subroutine at address
    JSR { address: Address },

    /// Skip next instruction if Vreg_id == value
    ///  - compare register Vreg_id to value, if equal, increment program counter by 2
    SE { reg_id: Byte, value: Byte },

    /// Skip next instruction if Vreg_id != value
    ///  - compare register Vreg_id to value, if not equal, increment program counter by 2
    SNE { reg_id: Byte, value: Byte },

    /// Skip next instruction if Vx_reg_id == Vy_reg_id
    ///  - compare register Vx_reg_id to Vy_reg_id, if equal, increment program counter by 2
    SEXY { x_reg_id: Byte, y_reg_id: Byte },

    /// put value in Vreg_id
    MOV { reg_id: Byte, value: Byte },

    /// set Vreg_id = Vreg_id + value
    ADD { reg_id: Byte, value: Byte },

    /// set Vx_reg_id = Vy_reg_id
    MOVXY { x_reg_id: Byte, y_reg_id: Byte },

    /// set Vx_reg_id = Vx_reg_id | Vy_reg_id
    ORXY { x_reg_id: Byte, y_reg_id: Byte },

    /// set Vx_reg_id = Vx_reg_id & Vy_reg_id
    ANDXY { x_reg_id: Byte, y_reg_id: Byte },

    /// set Vx_reg_id = Vx_reg_id ^ Vy_reg_id
    XORXY { x_reg_id: Byte, y_reg_id: Byte },

    /// set Vx_reg_id = Vx_reg_id + Vy_reg_id
    /// set VF = carry
    ///  - i.e set VF = 1 iff Vx_reg_id + Vy_reg_id > 255)
    ADDXY { x_reg_id: Byte, y_reg_id: Byte },

    /// set Vx_reg_id = Vx_reg_id - Vy_reg_id
    /// set VF = burrow
    ///  - i.e set VF = 1 if initialy Vx_reg_id < Vy_reg_id
    SUBXY { x_reg_id: Byte, y_reg_id: Byte },

    /// set Vreg_id = Vreg_id >> 1
    /// set VF = initial lsb
    SHR { reg_id: Byte },

    /// set Vx_reg_id = Vy_reg_id - Vx_reg_id
    /// set VF = burrow
    ///  - i.e set VF = 1 if initialy Vy_reg_id < Vx_reg_id
    RSUBXY { x_reg_id: Byte, y_reg_id: Byte },

    /// set Vreg_id = Vreg_id << 1
    /// set VF = initial msb
    SHL { reg_id: Byte },

    /// skip next instruction if Vx_reg_id != Vy_reg_id
    SNEXY { x_reg_id: Byte, y_reg_id: Byte },

    /// move address to I
    MOVI { address: Address },

    /// jump to address + V0
    JMI { address: Address },

    /// set Vreg_id = {random_byte} AND value
    RAND { reg_id: Byte, value: Byte },
    
    /// draws to the screen
    /// Dxyn - DRW Vx, Vy, nibble        
    DRW { x_reg_id: Byte, y_reg_id: Byte, value: Byte },

    /// skip next instruction if key with value of Vreg_id is pressed
    SKP { reg_id: Byte },

    /// skip next instruction if key with value of Vreg_id is not pressed
    SKNP { reg_id: Byte },

    /// set Vreg_id = delay timer value
    GDELAY { reg_id: Byte },

    /// wait for key, put key value in Vreg_id
    KEY { reg_id: Byte },

    /// set delay timer = Vreg_id
    SDELAY { reg_id: Byte },

    /// set the sound timer = Vreg_id
    SSOUND { reg_id: Byte },

    /// add Vreg_id to index register
    ADI { reg_id: Byte },

    /// point I to the spirte for hex char in Vreg_id
    /// sprite is 5 bytes high
    FONT { reg_id: Byte },

    /// set bcd repr of Vreg_id at locations I, I+1, I+2 (doesn't change I register itself)
    BCD { reg_id: Byte },

    /// store registers V0 up to Vreg_id at locations of I and onwards
    /// I is incremented to point to the next loction (I = I + reg_id + 1)
    STR { reg_id: Byte },

    /// load registers V0 up to Vreg_id from location of I and onwards
    /// /// I is incremented to point to the next loction (I = I + reg_id + 1)
    LDR { reg_id: Byte },
}

impl Instruction {
    pub fn parse_code(code: u16) -> Option<Instruction> {
        
        let nibbles: [u8; 4] = [
            (code & 0x000F) as u8,
            ((code & 0x00F0) >> 4) as u8,
            ((code & 0x0F00) >> 8) as u8,
            ((code & 0xF000) >> 12) as u8,
        ];

        return match_nibbles(&nibbles);
    }
}

fn match_nibbles(nibbles: &[u8; 4]) -> Option<Instruction> {
    let rev = [nibbles[3], nibbles[2], nibbles[1], nibbles[0]];
    match rev {
        [ 0x00, 0x00, 0x0E, 0x00 ] => Some(Instruction::CLS),
        [ 0x00, 0x00, 0x0E, 0x0E ] => Some(Instruction::RTS),
        [ 0x01, high, middle, low ] => Some(Instruction::JMP {
            address: three_nibbles(high, middle, low)
        }),
        [ 0x02, high, middle, low ] => Some(Instruction::JSR {
            address: three_nibbles(high, middle, low)
        }),
        [ 0x03, x, high, low ] => Some(Instruction::SE {
            reg_id: x, value: two_nibbles(high, low)
        }),
        [ 0x04, x, high, low ] => Some(Instruction::SNE {
            reg_id: x, value: two_nibbles(high, low)
        }),
        [ 0x05, x, y, 0 ] => Some(Instruction::SEXY {
            x_reg_id: x, y_reg_id: y,
        }),
        [ 0x06, x, high, low ] => Some(Instruction::MOV {
            reg_id: x, value: two_nibbles(high, low)
        }),
        [ 0x07, x, high, low ] => Some(Instruction::ADD {
            reg_id: x, value: two_nibbles(high, low)
        }),
        [ 0x08, x, y, 0 ] => Some(Instruction::MOVXY {
            x_reg_id: x, y_reg_id: y,
        }),
        [ 0x08, x, y, 1 ] => Some(Instruction::ORXY {
            x_reg_id: x, y_reg_id: y,
        }),
        [ 0x08, x, y, 2 ] => Some(Instruction::ANDXY {
            x_reg_id: x, y_reg_id: y,
        }),
        [ 0x08, x, y, 3 ] => Some(Instruction::XORXY {
            x_reg_id: x, y_reg_id: y,
        }),
        [ 0x08, x, y, 4 ] => Some(Instruction::ADDXY {
            x_reg_id: x, y_reg_id: y,
        }),
        [ 0x08, x, y, 5 ] => Some(Instruction::SUBXY {
            x_reg_id: x, y_reg_id: y,
        }),
        [ 0x08, x, 0x00, 0x06 ] => Some(Instruction::SHR {
            reg_id: x
        }),
        [ 0x08, x, y, 0x07 ] => Some(Instruction::RSUBXY {
            x_reg_id: x, y_reg_id: y
        }),
        [ 0x08, x, 0, 0x0E ] => Some(Instruction::SHL {
            reg_id: x
        }),
        [ 0x09, x, y, 0x00 ] => Some(Instruction::SNEXY {
            x_reg_id: x, y_reg_id: y
        }),
        [ 0x0A, high, middle, low ] => Some(Instruction::MOVI {
            address: three_nibbles(high, middle, low)
        }),
        [ 0x0B, high, middle, low ] => Some(Instruction::JMI {
            address: three_nibbles(high, middle, low)
        }),
        [ 0x0C, x, high, low ] => Some(Instruction::RAND {
            reg_id: x, value: two_nibbles(high, low)
        }),
        [ 0x0D, x, y, n ] => Some(Instruction::DRW {
            x_reg_id: x, y_reg_id: y, value: n
        }),
        [ 0x0E, k, 9, 0x0E ] => Some(Instruction::SKP {
            reg_id: k
        }),
        [ 0x0E, k, 0x0A, 0x01 ] => Some(Instruction::SKNP {
            reg_id: k
        }),
        [ 0x0F, r, 0x00, 0x07 ] => Some(Instruction::GDELAY {
            reg_id: r
        }),
        [ 0x0F, r, 0x00, 0x0A ] => Some(Instruction::KEY {
            reg_id: r
        }),
        [ 0x0F, r, 0x01, 0x05 ] => Some(Instruction::SDELAY {
            reg_id: r
        }),
        [ 0x0F, r, 0x01, 0x08 ] => Some(Instruction::SSOUND {
            reg_id: r
        }),
        [ 0x0F, r, 0x01, 0x0E ] => Some(Instruction::ADI {
            reg_id: r
        }),
        [ 0x0F, r, 0x02, 0x09 ] => Some(Instruction::FONT {
            reg_id: r
        }),
        [ 0x0F, r, 0x03, 0x03 ] => Some(Instruction::BCD {
            reg_id: r
        }),
        [ 0x0F, r, 0x05, 0x05 ] => Some(Instruction::STR {
            reg_id: r
        }),
        [ 0x0F, r, 0x06, 0x05 ] => Some(Instruction::LDR {
            reg_id: r
        }),
        _ => None
    }
}

fn two_nibbles(high: u8, low: u8) -> u8 {
    (high << 4) + low
}

fn three_nibbles(high: u8, middle: u8, low: u8) -> u16 {
    let mut res: u16 = 0;
    
    res += high as u16; 
    res <<= 4;

    res += middle as u16;
    res <<= 4;

    res += low as u16;

    return res;
}