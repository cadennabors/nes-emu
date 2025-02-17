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