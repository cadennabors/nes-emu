use crate::cpu::AddressingMode;

pub struct Opcode {
    pub name : u8,
    pub bytes : u8,
    pub cycles : u8,
    pub addressing_mode : AddressingMode
}

impl Opcode {
    pub fn new(name : u8, bytes : u8, cycles : u8, addressing_mode : AddressingMode) -> Self {
        Opcode {
        name,
        bytes,
        cycles,
        addressing_mode
     }
    }
}

// ACCESS 
// LDA loads a memory value into the accumulator.
pub const LDA_IMM : u8 = 0xA9;
pub const LDA_ZP : u8 = 0xA5;
pub const LDA_ZP_X : u8 = 0xB5;
pub const LDA_ABS : u8 = 0xAD;
pub const LDA_ABS_X : u8 = 0xBD;
pub const LDA_ABS_Y : u8 = 0xB9;
pub const LDA_IND_X : u8 = 0xA1;
pub const LDA_IND_Y : u8 = 0xB1;

// STA stores the accumulator value into memory.
pub const STA_ZP : u8 = 0x85;
pub const STA_ZP_X : u8 = 0x95;
pub const STA_ABS : u8 = 0x8D;
pub const STA_ABS_X : u8 = 0x9D;
pub const STA_ABS_Y : u8 = 0x99;
pub const STA_IND_X : u8 = 0x81;
pub const STA_ZP_IND_Y : u8 = 0x91;

// LDX loads a memory value into the X register.
pub const LDX_IMM : u8 = 0xA2;
pub const LDX_ZP : u8 = 0xA6;
pub const LDX_ZP_Y : u8 = 0xB6;
pub const LDX_ABS : u8 = 0xAE;
pub const LDX_ABS_Y : u8 = 0xBE;

// STX stores the X register value into memory.
pub const STX_ZP : u8 = 0x86;
pub const STX_ZP_Y : u8 = 0x96;
pub const STX_ABS : u8 = 0x8E;

