#[derive(Debug)]
pub enum Reg {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7,
    V8,
    V9,
    VA,
    VB,
    VC,
    VD,
    VE,
    VF,
    DelayTimer,
    SoundTimer
}

impl Into<usize> for Reg {
    fn into(self) -> usize {
        match self {
            Self::V0 => 0x0,
            Self::V1 => 0x1,
            Self::V2 => 0x2,
            Self::V3 => 0x3,
            Self::V4 => 0x4,
            Self::V5 => 0x5,
            Self::V6 => 0x6,
            Self::V7 => 0x7,
            Self::V8 => 0x8,
            Self::V9 => 0x9,
            Self::VA => 0xA,
            Self::VB => 0xB,
            Self::VC => 0xC,
            Self::VD => 0xD,
            Self::VE => 0xE,
            Self::VF => 0xF,
            Self::DelayTimer => 0x10,
            Self::SoundTimer => 0x11
        }
    }
}

impl Into<Reg> for u8 {
    fn into(self) -> Reg {
        match self {
            0x0 => Reg::V0,
            0x1 => Reg::V1,
            0x2 => Reg::V2,
            0x3 => Reg::V3,
            0x4 => Reg::V4,
            0x5 => Reg::V5,
            0x6 => Reg::V6,
            0x7 => Reg::V7,
            0x8 => Reg::V8,
            0x9 => Reg::V9,
            0xA => Reg::VA,
            0xB => Reg::VB,
            0xC => Reg::VC,
            0xD => Reg::VD,
            0xE => Reg::VE,
            0xF => Reg::VF,
            0x10 => panic!("Should never convert u8 to Reg::DelayTimer"),
            0x11 => panic!("Should never convert u8 to Reg::SoundTimer"),
            _ => panic!("Unknown register V{:x}", self)
        }
    }
}

const TIMER_COUNT: usize = 2;
const REG_COUNT: usize = 16 + TIMER_COUNT;

#[derive(Debug)]
pub struct Registers {
    inner: [u8; REG_COUNT],
    // default: used as carry flag
    // subtraction mode: no borrow flag
    // drawing: used for collision detection

    // program counter
    pc: u16,
    // index  register - 12 bits wide
    i: u16,
}

impl Registers {
    pub fn new() -> Self {
        let inner = [0; REG_COUNT];
        Self {
            inner,
            pc: 0x200,
            i: 0,
        }
    }

    pub fn pc(&self) -> u16 {
        self.pc
    }

    // move pc forward 2 bytes
    pub fn advance_pc(&mut self) {
        // TODO bounds a check
        self.pc += 0x2;
    }

    pub fn goto(&mut self, address: u16) {
        self.pc = address;
    }

    pub fn get_i(&self) -> u16 {
        self.i
    }

    pub fn set_i(&mut self, value: u16) {
        self.i = value;
    }

    pub fn get(&self, reg: Reg) -> u8 {
        let i: usize = reg.into();
        *self.inner.get(i).expect("Reg doesnt exist")
    }

    pub fn set(&mut self, reg: Reg, value: u8) {
        let i: usize = reg.into();
        *self.inner.get_mut(i).expect("Reg doesnt exist") = value;
    }

}
