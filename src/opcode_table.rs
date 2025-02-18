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

pub const OPCODE_TABLE : [Opcode ; 256] = [
Opcode::new(0x00, 1, 7, AddressingMode::NONE), // BRK implied
Opcode::new(0x01, 2, 6, AddressingMode::INDIRECTx), // ORA indirect (x-indexed)

Opcode::new(0x02, 1, 1, AddressingMode::UNIMPLEMENTED), 
Opcode::new(0x03, 1, 1, AddressingMode::UNIMPLEMENTED), 
Opcode::new(0x04, 1, 1, AddressingMode::UNIMPLEMENTED), 

Opcode::new(0x05, 2, 3, AddressingMode::ZEROPAGE), // ORA zero page
Opcode::new(0x06, 2, 5, AddressingMode::ZEROPAGE), // ASL zero page

Opcode::new(0x07, 1, 1, AddressingMode::UNIMPLEMENTED), 

Opcode::new(0x08, 1, 3, AddressingMode::NONE), // PHP implied
Opcode::new(0x09, 2, 2, AddressingMode::IMMEDIATE), // ORA immediate
Opcode::new(0x0A, 1, 2, AddressingMode::ACCUMULATOR), // ASL accumulator

Opcode::new(0x0B, 1, 1, AddressingMode::UNIMPLEMENTED), 
Opcode::new(0x0C, 1, 1, AddressingMode::UNIMPLEMENTED), 

Opcode::new(0x0D, 3, 4, AddressingMode::ABSOLUTE), // ORA absolute
Opcode::new(0x0E, 3, 6, AddressingMode::ABSOLUTE), // ASL absolute

Opcode::new(0x0F, 1, 1, AddressingMode::UNIMPLEMENTED), 

Opcode::new(0x10, 2, 2 /* VARIED */, AddressingMode::RELATIVE), //BPL relative
Opcode::new(0x11, 2, 5 /* VARIED */, AddressingMode::INDIRECTy), // ORA indirect

Opcode::new(0x12, 1, 1, AddressingMode::UNIMPLEMENTED), 
Opcode::new(0x13, 1, 1, AddressingMode::UNIMPLEMENTED), 
Opcode::new(0x14, 1, 1, AddressingMode::UNIMPLEMENTED), 

Opcode::new(0x15, 2, 4, AddressingMode::ZEROPAGEx), // ORA zero page x
Opcode::new(0x16, 2, 6, AddressingMode::ZEROPAGEx), // ASL zero page x

Opcode::new(0x17, 1, 1, AddressingMode::UNIMPLEMENTED), 

Opcode::new(0x18, 1, 2, AddressingMode::NONE), // CLC implied
Opcode::new(0x19, 3, 4 /* VARIED */, AddressingMode::ABSOLUTEy), // ORA absolute y

Opcode::new(0x1A, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x1B, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x1C, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x1D, 3, 4 /* VARIED */, AddressingMode::ABSOLUTEx), // ORA absolute x
Opcode::new(0x1E, 3, 7, AddressingMode::ABSOLUTEx), // ASL absolute x

Opcode::new(0x1F, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x20, 3, 6, AddressingMode::ABSOLUTE), // JSR absolute
Opcode::new(0x21, 2, 6, AddressingMode::INDIRECTx), // AND indirect x

Opcode::new(0x22, 1, 1, AddressingMode::UNIMPLEMENTED), 
Opcode::new(0x23, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x24, 2, 3, AddressingMode::ZEROPAGE), // BIT zero page
Opcode::new(0x25, 2, 3, AddressingMode::ZEROPAGE), // AND zero page
Opcode::new(0x26, 2, 5, AddressingMode::ZEROPAGE), // ROL zero page

Opcode::new(0x27, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x28, 1, 4, AddressingMode::NONE), // PLP implied
Opcode::new(0x29, 2, 2, AddressingMode::IMMEDIATE), // AND immediate
Opcode::new(0x2A, 1, 2, AddressingMode::ACCUMULATOR), // ROL accumulator

Opcode::new(0x2B, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x2C, 3, 4, AddressingMode::ABSOLUTE), // BIT absolute
Opcode::new(0x2D, 3, 4, AddressingMode::ABSOLUTE), // AND absolute
Opcode::new(0x2E, 3, 6, AddressingMode::ABSOLUTE), // ROL absolute

Opcode::new(0x2F, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x30, 2, 2 /* VARIES */, AddressingMode::RELATIVE), // BMI relative
Opcode::new(0x31, 2, 5 /* VARIES */, AddressingMode::INDIRECTy), // AND indirect y

Opcode::new(0x32, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x33, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x34, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x35, 2, 4, AddressingMode::ZEROPAGEx), // AND zero page x
Opcode::new(0x36, 2, 6, AddressingMode::ZEROPAGEy), // ROL zero page x

Opcode::new(0x37, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x38, 1, 2, AddressingMode::NONE), // SEC implied
Opcode::new(0x39, 3, 4 /* VARIES */, AddressingMode::ABSOLUTEy), // AND absolute y

Opcode::new(0x3A, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x3B, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x3C, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x3D, 3, 4 /* VARIES */, AddressingMode::ABSOLUTEx), // AND absolute x
Opcode::new(0x3E, 3, 7, AddressingMode::ABSOLUTEy), // ROL absolute x

Opcode::new(0x3F, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x40, 1, 6, AddressingMode::NONE), // RTI implied
Opcode::new(0x41, 2, 6, AddressingMode::INDIRECTx), // EOR indirect x

Opcode::new(0x42, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x43, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x44, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x45, 2, 3, AddressingMode::ZEROPAGE), // EOR zero page
Opcode::new(0x46, 2, 5, AddressingMode::ZEROPAGE), // LSR zero page

Opcode::new(0x47, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x48, 1, 3, AddressingMode::NONE), // PHA implied
Opcode::new(0x49, 2, 2, AddressingMode::IMMEDIATE), // EOR immediate
Opcode::new(0x4A, 1, 2, AddressingMode::ACCUMULATOR), // LSR accumulator

Opcode::new(0x4B, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x4C, 3, 3, AddressingMode::ABSOLUTE), // JMP absolute
Opcode::new(0x4D, 3, 4, AddressingMode::ABSOLUTE), // EOR absolute
Opcode::new(0x4E, 3, 6, AddressingMode::ABSOLUTE), // LSR absolute

Opcode::new(0x4F, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x50, 2, 2 /* VARIES */, AddressingMode::RELATIVE), // BVC relative
Opcode::new(0x51, 2, 5 /* VARIES */, AddressingMode::INDIRECTy), // EOR indirect y

Opcode::new(0x52, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x53, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x54, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x55, 2, 4, AddressingMode::ZEROPAGEx), // EOR zero page x
Opcode::new(0x56, 2, 6, AddressingMode::ZEROPAGEx), // LSR zero page x

Opcode::new(0x57, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x58, 1, 2, AddressingMode::NONE), // CLI implied
Opcode::new(0x59, 3, 4 /* VARIES */, AddressingMode::ABSOLUTEy), // EOR absolute y

Opcode::new(0x5A, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x5B, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x5C, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x5D, 3, 4 /* VARIES */, AddressingMode::ABSOLUTEx), // EOR absolute x
Opcode::new(0x5E, 3, 7, AddressingMode::ABSOLUTEx), // LSR absolute x

Opcode::new(0x5F, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x60, 1, 6, AddressingMode::NONE), // RTS implied
Opcode::new(0x61, 2, 6, AddressingMode::INDIRECTx), // ADC indirect x

Opcode::new(0x62, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x63, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x64, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x65, 2, 3, AddressingMode::ZEROPAGE), // ADC zero page
Opcode::new(0x66, 2, 5, AddressingMode::ZEROPAGE), // ROR zero page

Opcode::new(0x67, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x68, 1, 4, AddressingMode::NONE), // PLA implied
Opcode::new(0x69, 2, 2, AddressingMode::IMMEDIATE), // ADC immediate
Opcode::new(0x6A, 1, 2, AddressingMode::ACCUMULATOR), // ROR accumulator

Opcode::new(0x6B, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x6C, 3, 5, AddressingMode::INDIRECT), // JMP indirect
Opcode::new(0x6D, 3, 4, AddressingMode::ABSOLUTE), // ADC absolute
Opcode::new(0x6E, 3, 6, AddressingMode::ABSOLUTE), // ROR absolute

Opcode::new(0x6F, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x70, 2, 2 /* VARIES */, AddressingMode::RELATIVE), // BVS relative
Opcode::new(0x71, 2, 5 /* VARIES */, AddressingMode::INDIRECTy), // ADC indirect y

Opcode::new(0x72, 1, 1, AddressingMode::UNIMPLEMENTED), 
Opcode::new(0x73, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x74, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x75, 2, 4, AddressingMode::ZEROPAGEx), // ADC zero page x
Opcode::new(0x76, 2, 6, AddressingMode::ZEROPAGEx), // ROR zero page x

Opcode::new(0x77, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x78, 1, 2, AddressingMode::NONE), // SEI implied
Opcode::new(0x79, 3, 4 /* VARIES */, AddressingMode::ABSOLUTEy), // ADC absolute y

Opcode::new(0x7A, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x7B, 1, 1, AddressingMode::UNIMPLEMENTED),
Opcode::new(0x7C, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x7D, 3, 4 /* VARIES */, AddressingMode::ABSOLUTEx), // ADC absolute x
Opcode::new(0x7E, 3, 7, AddressingMode::ABSOLUTEx), // ROR absolute X

Opcode::new(0x7F, 1, 1, AddressingMode::UNIMPLEMENTED),

Opcode::new(0x80, bytes, cycles, addressing_mode),
Opcode::new(0x81, bytes, cycles, addressing_mode),
Opcode::new(0x82, bytes, cycles, addressing_mode),
Opcode::new(0x83, bytes, cycles, addressing_mode),
Opcode::new(0x84, bytes, cycles, addressing_mode),
Opcode::new(0x85, bytes, cycles, addressing_mode),
Opcode::new(0x86, bytes, cycles, addressing_mode),
Opcode::new(0x87, bytes, cycles, addressing_mode),
Opcode::new(0x88, bytes, cycles, addressing_mode),
Opcode::new(0x89, bytes, cycles, addressing_mode),
Opcode::new(0x8A, bytes, cycles, addressing_mode),
Opcode::new(0x8B, bytes, cycles, addressing_mode),
Opcode::new(0x8C, bytes, cycles, addressing_mode),
Opcode::new(0x8D, bytes, cycles, addressing_mode),
Opcode::new(0x8E, bytes, cycles, addressing_mode),
Opcode::new(0x8F, bytes, cycles, addressing_mode),
Opcode::new(0x90, bytes, cycles, addressing_mode),
Opcode::new(0x91, bytes, cycles, addressing_mode),
Opcode::new(0x92, bytes, cycles, addressing_mode),
Opcode::new(0x93, bytes, cycles, addressing_mode),
Opcode::new(0x94, bytes, cycles, addressing_mode),
Opcode::new(0x95, bytes, cycles, addressing_mode),
Opcode::new(0x96, bytes, cycles, addressing_mode),
Opcode::new(0x97, bytes, cycles, addressing_mode),
Opcode::new(0x98, bytes, cycles, addressing_mode),
Opcode::new(0x99, bytes, cycles, addressing_mode),
Opcode::new(0x9A, bytes, cycles, addressing_mode),
Opcode::new(0x9B, bytes, cycles, addressing_mode),
Opcode::new(0x9C, bytes, cycles, addressing_mode),
Opcode::new(0x9D, bytes, cycles, addressing_mode),
Opcode::new(0x9E, bytes, cycles, addressing_mode),
Opcode::new(0x9F, bytes, cycles, addressing_mode),
Opcode::new(0xA0, bytes, cycles, addressing_mode),
Opcode::new(0xA1, bytes, cycles, addressing_mode),
Opcode::new(0xA2, bytes, cycles, addressing_mode),
Opcode::new(0xA3, bytes, cycles, addressing_mode),
Opcode::new(0xA4, bytes, cycles, addressing_mode),
Opcode::new(0xA5, bytes, cycles, addressing_mode),
Opcode::new(0xA6, bytes, cycles, addressing_mode),
Opcode::new(0xA7, bytes, cycles, addressing_mode),
Opcode::new(0xA8, bytes, cycles, addressing_mode),
Opcode::new(0xA9, bytes, cycles, addressing_mode),
Opcode::new(0xAA, bytes, cycles, addressing_mode),
Opcode::new(0xAB, bytes, cycles, addressing_mode),
Opcode::new(0xAC, bytes, cycles, addressing_mode),
Opcode::new(0xAD, bytes, cycles, addressing_mode),
Opcode::new(0xAE, bytes, cycles, addressing_mode),
Opcode::new(0xAF, bytes, cycles, addressing_mode),
Opcode::new(0xB0, bytes, cycles, addressing_mode),
Opcode::new(0xB1, bytes, cycles, addressing_mode),
Opcode::new(0xB2, bytes, cycles, addressing_mode),
Opcode::new(0xB3, bytes, cycles, addressing_mode),
Opcode::new(0xB4, bytes, cycles, addressing_mode),
Opcode::new(0xB5, bytes, cycles, addressing_mode),
Opcode::new(0xB6, bytes, cycles, addressing_mode),
Opcode::new(0xB7, bytes, cycles, addressing_mode),
Opcode::new(0xB8, bytes, cycles, addressing_mode),
Opcode::new(0xB9, bytes, cycles, addressing_mode),
Opcode::new(0xBA, bytes, cycles, addressing_mode),
Opcode::new(0xBB, bytes, cycles, addressing_mode),
Opcode::new(0xBC, bytes, cycles, addressing_mode),
Opcode::new(0xBD, bytes, cycles, addressing_mode),
Opcode::new(0xBE, bytes, cycles, addressing_mode),
Opcode::new(0xBF, bytes, cycles, addressing_mode),
Opcode::new(0xC0, bytes, cycles, addressing_mode),
Opcode::new(0xC1, bytes, cycles, addressing_mode),
Opcode::new(0xC2, bytes, cycles, addressing_mode),
Opcode::new(0xC3, bytes, cycles, addressing_mode),
Opcode::new(0xC4, bytes, cycles, addressing_mode),
Opcode::new(0xC5, bytes, cycles, addressing_mode),
Opcode::new(0xC6, bytes, cycles, addressing_mode),
Opcode::new(0xC7, bytes, cycles, addressing_mode),
Opcode::new(0xC8, bytes, cycles, addressing_mode),
Opcode::new(0xC9, bytes, cycles, addressing_mode),
Opcode::new(0xCA, bytes, cycles, addressing_mode),
Opcode::new(0xCB, bytes, cycles, addressing_mode),
Opcode::new(0xCC, bytes, cycles, addressing_mode),
Opcode::new(0xCD, bytes, cycles, addressing_mode),
Opcode::new(0xCE, bytes, cycles, addressing_mode),
Opcode::new(0xCF, bytes, cycles, addressing_mode),
Opcode::new(0xD0, bytes, cycles, addressing_mode),
Opcode::new(0xD1, bytes, cycles, addressing_mode),
Opcode::new(0xD2, bytes, cycles, addressing_mode),
Opcode::new(0xD3, bytes, cycles, addressing_mode),
Opcode::new(0xD4, bytes, cycles, addressing_mode),
Opcode::new(0xD5, bytes, cycles, addressing_mode),
Opcode::new(0xD6, bytes, cycles, addressing_mode),
Opcode::new(0xD7, bytes, cycles, addressing_mode),
Opcode::new(0xD8, bytes, cycles, addressing_mode),
Opcode::new(0xD9, bytes, cycles, addressing_mode),
Opcode::new(0xDA, bytes, cycles, addressing_mode),
Opcode::new(0xDB, bytes, cycles, addressing_mode),
Opcode::new(0xDC, bytes, cycles, addressing_mode),
Opcode::new(0xDD, bytes, cycles, addressing_mode),
Opcode::new(0xDE, bytes, cycles, addressing_mode),
Opcode::new(0xDF, bytes, cycles, addressing_mode),
Opcode::new(0xE0, bytes, cycles, addressing_mode),
Opcode::new(0xE1, bytes, cycles, addressing_mode),
Opcode::new(0xE2, bytes, cycles, addressing_mode),
Opcode::new(0xE3, bytes, cycles, addressing_mode),
Opcode::new(0xE4, bytes, cycles, addressing_mode),
Opcode::new(0xE5, bytes, cycles, addressing_mode),
Opcode::new(0xE6, bytes, cycles, addressing_mode),
Opcode::new(0xE7, bytes, cycles, addressing_mode),
Opcode::new(0xE8, bytes, cycles, addressing_mode),
Opcode::new(0xE9, bytes, cycles, addressing_mode),
Opcode::new(0xEA, bytes, cycles, addressing_mode),
Opcode::new(0xEB, bytes, cycles, addressing_mode),
Opcode::new(0xEC, bytes, cycles, addressing_mode),
Opcode::new(0xED, bytes, cycles, addressing_mode),
Opcode::new(0xEE, bytes, cycles, addressing_mode),
Opcode::new(0xEF, bytes, cycles, addressing_mode),
Opcode::new(0xF0, bytes, cycles, addressing_mode),
Opcode::new(0xF1, bytes, cycles, addressing_mode),
Opcode::new(0xF2, bytes, cycles, addressing_mode),
Opcode::new(0xF3, bytes, cycles, addressing_mode),
Opcode::new(0xF4, bytes, cycles, addressing_mode),
Opcode::new(0xF5, bytes, cycles, addressing_mode),
Opcode::new(0xF6, bytes, cycles, addressing_mode),
Opcode::new(0xF7, bytes, cycles, addressing_mode),
Opcode::new(0xF8, bytes, cycles, addressing_mode),
Opcode::new(0xF9, bytes, cycles, addressing_mode),
Opcode::new(0xFA, bytes, cycles, addressing_mode),
Opcode::new(0xFB, bytes, cycles, addressing_mode),
Opcode::new(0xFC, bytes, cycles, addressing_mode),
Opcode::new(0xFD, bytes, cycles, addressing_mode),
Opcode::new(0xFE, bytes, cycles, addressing_mode),
Opcode::new(0xFF, bytes, cycles, addressing_mode),
];