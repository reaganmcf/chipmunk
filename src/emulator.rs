use crate::registers::Registers;

const STACK_COUNT: usize = 12;

pub struct Emulator {
    // 0x000 -> 0x1FF = interpter
    // 0x050 -> 0x0A0 = pixel font
    // 0x200 -> 0xFFF = rom and everything else
    memory: &'static [u8], // 4 KB of memory that lives for the entire program
    registers: Registers,
    stacks: Vec<u16>
}

const MEM_SIZE: usize = 4096;

impl Emulator {
    pub fn new() -> Self {
        // TODO - load rom?
        let memory = vec![0; MEM_SIZE].leak(); 
        let registers = Registers::new();


        Self {
            memory,
            registers,
            stacks: Vec::with_capacity(STACK_COUNT)
        }
    }
}
