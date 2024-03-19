use std::{collections::HashMap, fmt};

pub const LANG_CPP: &str = "cpp";
pub const LANG_RUST: &str = "rust";

const EXTNSN_CPP: &str = "cpp";
const EXTNSN_RUST: &str = "rs";
const EXTNSN_TXT: &str = "txt";

pub const LANGUAGE_LIST: [&str; 2] = [LANG_CPP, LANG_RUST];

pub const IN_INT: &str = "integer";
pub const IN_STRING: &str = "string";
pub const IN_LIST_INT: &str = "integer[]";
pub const IN_LIST_CHAR: &str = "character[]";
pub const IN_LIST_STRING: &str = "string[]";
pub const IN_LIST_STRING2: &str = "list<string>";
pub const IN_MATRIX_INT: &str = "integer[][]";
pub const IN_MATRIX_CHAR: &str = "character[][]";

pub const OUT_VOID: &str = "void";
pub const OUT_INT: &str = "integer";
pub const OUT_BOOL: &str = "boolean";
pub const OUT_STRING: &str = "string";
pub const OUT_LIST_INT: &str = "list<integer>";
pub const OUT_LIST_INT2: &str = "integer[]";
pub const OUT_LIST_STRING: &str = "list<string>";
pub const OUT_MATRIX_INT: &str = "integer[][]";
pub const OUT_MATRIX_INT2: &str = "list<list<integer>>";

pub fn extension_lang_map(language: &String) -> &str {
    match language.to_ascii_lowercase().as_str() {
        LANG_CPP => EXTNSN_CPP,
        LANG_RUST => EXTNSN_RUST,
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
    // Map of input variables and values
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

    pub fn get_input_var_types(&self) -> Vec<(String, String)> {
        self.input_var_types.clone()
    }

    pub fn get_input_var_values(&self) -> HashMap<String, String> {
        self.input_var_values.clone()
    }

    pub fn get_output_type(&self) -> String {
        self.output_type.clone()
    }

    pub fn get_output_value(&self) -> String {
        self.output_value.clone()
    }

    pub fn to_string(&self, example_number: usize) -> String {
        let mut example_string: String = String::new();

        // input_type_1 input_var_i_1 = input_val_i_1;
        // ..
        // input_type_n input_var_i_n = input_val_i_n;
        for i in 0..self.input_var_types.len() {
            let (input_var_name, input_var_type) = &self.input_var_types[i];
            example_string += format!(
                "\t{} {}_{} = {};\n",
                input_var_type,
                input_var_name,
                example_number,
                self.input_var_values[input_var_name].as_str()
            )
            .as_str();
        }

        // return_type output_i = output_val_i;
        example_string += format!(
            "\t{} expected_{} = {};\n",
            self.output_type, example_number, self.output_value
        )
        .as_str();

        // return_type output_i = func_name(input_var_i_1, input_var_i_2, .. input_var_i_n);
        example_string +=
            format!("\t{} output_{} = func(", self.output_type, example_number,).as_str();
        let mut i: usize = 0;
        while i < self.input_var_types.len() - 1 {
            let input_var_name_type = &self.input_var_types[i];
            example_string += format!("{}_{}, ", input_var_name_type.0, example_number).as_str();
            i += 1;
        }
        example_string += format!("{}_{});\n", &self.input_var_types[i].0, example_number).as_str();

        // assert(output_i == expected_i);
        example_string += format!(
            "\tassert(output_{} == expected_{});\n\n",
            example_number, example_number
        )
        .as_str();

        example_string
    }
}
