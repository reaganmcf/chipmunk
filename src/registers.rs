#[derive(Debug)]
pub enum Reg {
    V0,
    V1,
    V2,
}

impl Into<usize> for Reg {
    fn into(self) -> usize {
        match self {
            Self::V0 => 0,
            Self::V1 => 1,
            Self::V2 => 2,
        }
    }
}

impl Into<Reg> for u8 {
    fn into(self) -> Reg {
        match self {
            0x0 => Reg::V0,
            0x1 => Reg::V1,
            0x2 => Reg::V2,
            _ => panic!("Unknown register V{:x}", self)
        }
    }
}

const REG_COUNT: usize = 16;

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
            i: 0
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

    pub fn get(&self, reg: Reg) -> u8 {
        let i: usize = reg.into();
        *self.inner.get(i).expect("Reg doesnt exist")
    }

    pub fn set(&mut self, reg: Reg, value: u8) {
        let i: usize = reg.into();
        *self.inner.get_mut(i).expect("Reg doesnt exist") = value;
    }

}
