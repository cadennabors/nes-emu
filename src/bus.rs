use crate::cpu::CPU;

pub struct Bus {
    ram : [u8; 64 * 1024],
    rom : [u8; 64 * 1024],
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            ram: [0; 64 * 1024],
            rom: [0; 64 * 1024],
        }
    }
    
    pub fn write(&mut self, addr : u16, data : u8) -> () {
        self.ram[addr as usize] = data;
    
    }
    pub fn read(&mut self, addr : u16, rom : Option<bool>) -> u8 {
        if let Some(true) = rom {
            return self.rom[addr as usize];
        }
        return self.ram[addr as usize];
    }
}
