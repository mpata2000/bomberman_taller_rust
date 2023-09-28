pub enum InputError {
    InvalidInput(String),
    FileError(String),
}

impl std::fmt::Display for InputError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InputError::InvalidInput(e) => write!(f, "InvalidInput: {}", e),
            InputError::FileError(e) => write!(f, "FileError: {}", e),
        }
    }
}
