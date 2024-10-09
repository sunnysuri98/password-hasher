use std::fmt;

#[derive(Debug)]
pub enum ErrorMessage {
    LengthError,
    EmptyPasswordError,
    HashingError
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ErrorMessage::LengthError => "Password cannot be greater than 64 characters",
            ErrorMessage::EmptyPasswordError => "Password cannot be empty",
            ErrorMessage::HashingError => "Hashing Error",
        };
        write!(f, "{}", message)
    }
}
