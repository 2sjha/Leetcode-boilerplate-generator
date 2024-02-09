use crate::{utils::Example, utils::CustomError};

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

pub fn generate_driver_code(
    examples: Vec<Example>,
    starter_code: &String,
    language: &String,
) -> String {
    match language.to_ascii_lowercase().as_str() {
        LANG_CPP => cpp_driver_code(examples, starter_code),
        LANG_RUST => rust_driver_code(examples, starter_code),
        LANG_JAVA => java_driver_code(examples, starter_code),
        LANG_PYTHON => python_driver_code(examples, starter_code),
        _ => "".to_string(), // Won't happen
    }
}

pub fn generate_description_as_comment(
    problem_url: &String,
    description: &String,
    language: &String,
) -> String {
    match language.to_ascii_lowercase().as_str() {
        LANG_CPP => generic_description_comment(problem_url, description),
        LANG_RUST => generic_description_comment(problem_url, description),
        LANG_JAVA => generic_description_comment(problem_url, description),
        LANG_PYTHON => python_description_comment(problem_url, description),
        _ => "".to_string(), // Won't happen
    }
}

fn generic_description_comment(problem_url: &String, description: &String) -> String {
    let mut desc_comment: String = String::new();
    desc_comment += "/*\n";
    desc_comment += problem_url;
    desc_comment += "\n\n";
    desc_comment += description;
    desc_comment += "*/\n\n";

    desc_comment
}

fn python_description_comment(problem_url: &String, description: &String) -> String {
    let mut desc_comment: String = String::new();
    desc_comment += "\"\"\"\n";
    desc_comment += problem_url;
    desc_comment += "\n\n";
    desc_comment += description;
    desc_comment += "\"\"\"\n\n";

    desc_comment
}

fn cpp_driver_code(examples: Vec<Example>, starter_code: &String) -> String {
    let mut driver_code: String = String::new();
    driver_code += starter_code;
    driver_code += "\n\n";
    for example in examples {
        driver_code += example.get_input().as_str();
        driver_code += "\n";
        driver_code += example.get_output().as_str();
    }

    driver_code
}

fn rust_driver_code(examples: Vec<Example>, starter_code: &String) -> String {
    let mut driver_code: String = String::new();
    driver_code += "struct Solution;\n";
    driver_code += starter_code;
    driver_code += "\n\n";
    driver_code += "fn main() {\n\n";

    if analyze_example(&examples[0]).is_err() {
        driver_code += examples_as_comment(examples).as_str();
    } else {
        // driver_code += examples_as_rust_code(examples, starter_code).as_str();
    }

    driver_code += "}\n";
    driver_code
}

fn python_driver_code(examples: Vec<Example>, starter_code: &String) -> String {
    cpp_driver_code(examples, starter_code)
}

fn java_driver_code(examples: Vec<Example>, starter_code: &String) -> String {
    cpp_driver_code(examples, starter_code)
}

fn analyze_example(example: &Example) -> Result<(), Box<dyn std::error::Error>> {
    Err(CustomError::from_str(
        "Couldn't understand example. Generating incomplete driver code.",
    ))?
}



fn examples_as_comment(examples: Vec<Example>) -> String {
    let mut examples_comment: String = String::new();
    examples_comment += "/*\n";
    let mut i: usize = 0;
    let mut example: &Example;
    while i < examples.len() - 1 {
        example = &examples[i];
        examples_comment += example.get_input().as_str();
        examples_comment += "\n";
        examples_comment += example.get_output().as_str();
        examples_comment += "\n\n";
        i += 1;
    }

    example = &examples[i];
    examples_comment += example.get_input().as_str();
    examples_comment += "\n";
    examples_comment += example.get_output().as_str();
    examples_comment += "\n*/\n\n";

    examples_comment
}
