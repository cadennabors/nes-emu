use crate::cpu::CPU;

pub struct Bus {
    cpu : CPU,
    ram : [u8; 64 * 1024]
}

impl Bus {
    pub fn new() -> Self {
        Bus {
            cpu: CPU::new(),
            ram: [0; 64 * 1024]
        }
    }
    
    pub fn write(&mut self, addr : u16, data : u8) -> () {
        self.ram[addr as usize] = data;
    
    }
    pub fn read(addr : u16, bReadOnly : Option<bool>) -> u8 {
        return 0x00
    }
}
