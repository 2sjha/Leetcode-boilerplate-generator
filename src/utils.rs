use std::fmt;

#[derive(Debug, Clone)]
pub struct CustomError {
    message: String,
}
impl CustomError {
    pub fn from(message: String) -> CustomError {
        CustomError { message }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CustomError {}
