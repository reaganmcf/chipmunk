pub struct Registers {
    pub v0: u8,
    pub v1: u8,
    pub v2: u8,
    pub v3: u8,
    pub v4: u8,
    pub v5: u8,
    pub v6: u8,
    pub v7: u8,
    pub v8: u8,
    pub v9: u8,
    pub va: u8,
    pub vb: u8,
    pub vc: u8,
    pub vd: u8,
    pub ve: u8,

    // default: used as carry flag
    // subtraction mode: no borrow flag
    // drawing: used for collision detection
    pub vf: u8,

    // program counter
    pub pc: u16,
    // index  register - 12 bits wide
    pub i: u16,
}

impl Registers {
    pub fn new() -> Self {
        Self {
            v0: 0,
            v1: 0,
            v2: 0,
            v3: 0,
            v4: 0,
            v5: 0,
            v6: 0,
            v7: 0,
            v8: 0,
            v9: 0,
            va: 0,
            vb: 0,
            vc: 0,
            vd: 0,
            ve: 0,
            vf: 0,
            pc: 0x200,
            i: 0
        }
    }
}
