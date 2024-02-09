use std::fmt;

#[derive(Debug, Clone)]
pub struct CustomError {
    message: String,
}
impl CustomError {
    pub fn from_str(message: &str) -> CustomError {
        CustomError {
            message: message.to_string(),
        }
    }
    pub fn from(message: String) -> CustomError {
        CustomError { message: message }
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for CustomError {}

pub struct Example {
    input: String,
    output: String,
}

impl Example {
    pub fn get_input(&self) -> String {
        String::clone(&self.input)
    }
    pub fn get_output(&self) -> String {
        String::clone(&self.output)
    }

    pub fn from(input: &str, output: &str) -> Example {
        Example {
            input: input.to_string(),
            output: output.to_string(),
        }
    }
}
