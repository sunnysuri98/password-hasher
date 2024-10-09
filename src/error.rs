use std::fmt;

#[derive(Debug)]
pub enum ErrorMessage {
    LengthError,
    EmptyPasswordError,
    HashingError
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorMessage::LengthError => write!(f, "Password cannot be greater than 64 characters"),

            ErrorMessage::EmptyPasswordError => write!(f, "Password cannot be empty"),

            ErrorMessage::HashingError=> write!(f, "Hashing Error")
        }
    }
}
