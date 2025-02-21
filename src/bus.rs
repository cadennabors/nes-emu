use crate::cpu::CPU;

pub struct Bus {
    ram : [u8; 64 * 1024],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; 64 * 1024],
        }
    }
    
    pub fn write(&mut self, addr : u16, data : u8) -> () {
        self.ram[addr as usize] = data;
    
    }
    pub fn read(&mut self, addr : u16, read_only : Option<bool>) -> u8 {
        return self.ram[addr as usize];
    }

    pub fn load_program(&mut self, program: &[u8], starting_address : u16) {
        let program_len = program.len() as u16;
        for (i, &byte) in program.iter().enumerate() {
            let addr = starting_address + i as u16;
            self.write(addr, byte);
        }  
    }
}
