use bus::Bus;

pub mod cpu;
pub mod bus;
pub mod opcodes;

fn main() {
    let bus_ref = bus::Bus::new();
    let mut cpu_ref = cpu::CPU::new(bus_ref);
    cpu_ref.interpret();
}
