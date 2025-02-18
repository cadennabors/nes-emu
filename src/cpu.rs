use crate ::bus::Bus;
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub stkp : u8,
    pub program_counter: u16,
    bus : Bus,
}
pub enum AddressingMode {
    IMMEDIATE,
    ZEROPAGE,
    ZEROPAGEx,
    ABSOLUTE,
    ABSOLUTEx,
    ABSOLUTEy,
    INDIRECTx,
    INDIRECTy,
}

impl CPU {
    pub fn new(bus : Bus) -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            stkp : 0,
            program_counter: 0,
            bus,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
 
    }

    fn get_input_address(&self, mode: AddressingMode) -> u16 {
        0xaaaa
    }

    fn ADC(&mut self, mode : AddressingMode) {

    }

    fn write(&mut self, addr : u16, data : u8) -> () {
        self.bus.write(addr, data);
    
    }
    fn read(&mut self,addr : u16, _bReadOnly : Option<bool>) -> u8 {
        return self.bus.read(addr, _bReadOnly);
    }
// ----------------STATUS----------------------
fn set_carry_bit (&mut self) {
    self.status |= 0b1000_0000
}
fn clear_carry_bit (&mut self) {
    self.status &= !0b1000_0000
}
fn set_zero_bit (&mut self) {
    self.status |= 0b0100_0000
}
fn clear_zero_bit (&mut self) {
    self.status &= !0b0100_0000
}
fn set_disable_interrupts_bit (&mut self) {
    self.status |= 0b0010_0000
}
fn clear_disable_interrupts_bit (&mut self) {
    self.status &= !0b0010_0000
}
fn set_decimal_bit (&mut self) {
    self.status |= 0b0001_0000
}
fn clear_decimal_bit (&mut self) {
    self.status &= !0b0001_0000
}
fn set_break_bit (&mut self) {
    self.status |= 0b0000_1000
}
fn clear_break_bit (&mut self) {
    self.status &= !0b0000_1000
}
fn set_unused_bit (&mut self) {
    self.status |= 0b0000_0100
}
fn clear_unused_bit (&mut self) {
    self.status &= !0b0000_0100
}
fn set_overflow_bit (&mut self) {
    self.status |= 0b0000_0010
}
fn clear_overflow_bit (&mut self) {
    self.status &= !0b0000_0010
}
fn set_negative_bit (&mut self) {
    self.status |= 0b0000_0001
}
fn clear_negative_bit (&mut self) {
    self.status &= !0b0000_0001
}
}

// C - Carry Bit 0b1000_0000
// Z - Zero 0b0100_0000
// I - Disable Interrupts 0b0010_0000
// D - Decimal Mode (unused) 0b0001_0000
// B - Break 0b0000_1000
// U - Unused 0b0000_0100
// V - Overflow 0b0000_0010
// N - Negative 0b0000_0001

