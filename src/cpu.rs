use crate ::bus::Bus;
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub status: u8,
    pub program_counter: u16,
    bus : Bus
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            program_counter: 0,
            bus : std::ptr::null,
        }
    }

    pub fn interpret(&mut self, program: Vec<u8>) {
 
    }

    fn write(addr : u16, data : u8) -> () {
        bus.write(addr, data)
    
    }
    fn read(addr : u16, _bReadOnly : Option<bool>) -> u8 {
        return self.ram[addr as usize];
    }
}
