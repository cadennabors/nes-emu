use std::array;
use crate::cpu::{self, CPU};

pub struct Bus {
    cpu : CPU,
    ram : [u8; 64 * 1024]
}

impl Bus {
    pub fn write(address : u16, data : u8) -> () {

    }
    pub fn read(address : u16, bReadOnly : Option<bool>) -> u8 {
        return 0x00
    }
}
