// Subsystem components
struct BIOS;
impl BIOS {
    pub fn execute(&self) {
        println!("Executing BIOS...");
    }
}
struct CPU;
impl CPU {
    pub fn load(&self) {
        println!("Loading CPU...");
    }
}
struct Memory;
impl Memory {
    pub fn check(&self) {
        println!("Checking memory...");
    }
}
struct HardDrive;
impl HardDrive {
    pub fn read(&self) {
        println!("Reading from hard drive...");
    }
}
// Facade
struct ComputerFacade {
    bios: BIOS,
    cpu: CPU,
    memory: Memory,
    hard_drive: HardDrive,
}
impl ComputerFacade {
    pub fn new() -> Self {
        ComputerFacade {
            bios: BIOS {},
            cpu: CPU {},
            memory: Memory {},
            hard_drive: HardDrive {},
        }
    }
    pub fn start(&self) {
        self.bios.execute();
        self.cpu.load();
        self.memory.check();
        self.hard_drive.read();
    }
}
// Client code
// cargo run --example facade_ex1
fn main() {
    let computer = ComputerFacade::new();
    computer.start();
}
