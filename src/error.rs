use core::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    IllegalInstruction(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IllegalInstruction(u) => {
                write!(f, "Illegal Instruction: {:x}", u)
            }
        }
    }
}

impl std::error::Error for Error {}
