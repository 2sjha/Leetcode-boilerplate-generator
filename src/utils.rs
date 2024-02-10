use std::{collections::HashMap, fmt};

pub const LANG_CPP: &str = "cpp";
pub const LANG_RUST: &str = "rust";
pub const LANG_JAVA: &str = "java";
pub const LANG_PYTHON: &str = "python3";

pub const EXTNSN_CPP: &str = "cpp";
pub const EXTNSN_RUST: &str = "rs";
pub const EXTNSN_JAVA: &str = "java";
pub const EXTNSN_PYTHON: &str = "py";
pub const EXTNSN_TXT: &str = "txt";

pub const LANGUAGE_LIST: [&str; 4] = [LANG_CPP, LANG_RUST, LANG_JAVA, LANG_PYTHON];

pub fn extension_lang_map(language: &String) -> &str {
    match language.to_ascii_lowercase().as_str() {
        LANG_CPP => EXTNSN_CPP,
        LANG_RUST => EXTNSN_RUST,
        LANG_JAVA => EXTNSN_JAVA,
        LANG_PYTHON => EXTNSN_PYTHON,
        _ => EXTNSN_TXT,
    }
}

pub fn language_list_string() -> String {
    let mut list_string: String = format!("{:?}", LANGUAGE_LIST);
    list_string = list_string[1..list_string.len() - 1].to_string();

    list_string
}

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
    // Ordered list of input variables and their types
    input_var_types: Vec<(String, String)>,
    input_var_values: HashMap<String, String>,
    output_type: String,
    output_value: String,
}

impl Example {
    pub fn from(
        input_var_types: Vec<(String, String)>,
        input_var_values: HashMap<String, String>,
        output_type: String,
        output_value: String,
    ) -> Example {
        Example {
            input_var_types,
            input_var_values,
            output_type,
            output_value,
        }
    }

    pub fn to_string(&self, example_number: String) -> String {
        let mut example_string: String = String::new();
        for i in 0..self.input_var_types.len() {
            let (input_var_name, input_var_type) = &self.input_var_types[i]; 
            example_string += input_var_type.as_str();
            example_string += " ";
            example_string += input_var_name.as_str();
            example_string += "_";
            example_string += example_number.as_str();
            example_string += " = ";
            example_string += self.input_var_values[input_var_name].as_str();
            example_string += ";\n";

            example_string += self.output_type.as_str();
            example_string += " res";
            example_string += "_";
            example_string += example_number.as_str();
            example_string += " = ";
            example_string += self.output_value.as_str();
            example_string += ";\n\n";
        }

        example_string
    }
}
