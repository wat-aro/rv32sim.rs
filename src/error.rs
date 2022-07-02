use core::fmt;

#[derive(Debug)]
pub enum Error {
    IllegalInstruction(u32),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IllegalInstruction(u) => {
                write!(f, "Illegal Instruction: {}", u)
            }
        }
    }
}

impl std::error::Error for Error {}
