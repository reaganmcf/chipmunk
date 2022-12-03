#[derive(Debug)]
pub enum EmulatorError {
    Exit,
    UnknownOpCode(String),
}
