use crate::utils::{self, Example};

pub fn generate_driver_code(
    examples: Vec<Example>,
    starter_code: &String,
    language: &String,
) -> String {
    match language.to_ascii_lowercase().as_str() {
        utils::LANG_CPP => cpp_driver_code(examples, starter_code),
        utils::LANG_RUST => rust_driver_code(examples, starter_code),
        utils::LANG_JAVA => java_driver_code(examples, starter_code),
        utils::LANG_PYTHON => python_driver_code(examples, starter_code),
        _ => "".to_string(), // Won't happen
    }
}

pub fn generate_description_as_comment(
    problem_url: &String,
    description: &String,
    language: &String,
) -> String {
    match language.to_ascii_lowercase().as_str() {
        utils::LANG_CPP => generic_description_comment(problem_url, description),
        utils::LANG_RUST => generic_description_comment(problem_url, description),
        utils::LANG_JAVA => generic_description_comment(problem_url, description),
        utils::LANG_PYTHON => python_description_comment(problem_url, description),
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

fn rust_driver_code(examples: Vec<Example>, starter_code: &String) -> String {
    let mut driver_code: String = String::new();
    driver_code += "struct Solution;\n";
    driver_code += starter_code;
    driver_code += "\n\n";
    driver_code += "fn main() {\n\n";

    // Cant parse 1st example or create code for it
    // Then problem code/type might be too specific
    // Thus write them as comment.
    let code_example_1 = rust_code_for_example(&examples[0], 1, starter_code);
    if code_example_1.is_none() {
        eprintln!("Couldn't understand example. Writing examples as comment. You're on your own for this one.");
        driver_code += examples_as_comment(examples).as_str();
    } else {
        driver_code += code_example_1.unwrap().as_str();
        if examples.len() >= 2 {
            for i in 1..examples.len() {
                driver_code += rust_code_for_example(&examples[i], i + 1, starter_code)
                    .unwrap()
                    .as_str();
            }
        }
    }

    driver_code += "}\n";
    driver_code
}

fn rust_code_for_example(
    example: &Example,
    example_number: usize,
    starter_code: &String,
) -> Option<String> {
    Some(example.to_string(example_number.to_string()))
}

fn cpp_driver_code(examples: Vec<Example>, starter_code: &String) -> String {
    let mut driver_code: String = String::new();
    driver_code += starter_code;
    driver_code += "\n\n";
    let mut i: usize = 0;
    while i < examples.len() {
        let example: &Example = &examples[i];
        driver_code += example.to_string((i + 1).to_string()).as_str();
        i += 1;
    }

    driver_code
}

fn cpp_code_for_example(example: &Example) -> Option<String> {
    None
}

fn python_driver_code(examples: Vec<Example>, starter_code: &String) -> String {
    cpp_driver_code(examples, starter_code)
}

fn python_code_for_example(example: &Example) -> Option<String> {
    None
}

fn java_driver_code(examples: Vec<Example>, starter_code: &String) -> String {
    cpp_driver_code(examples, starter_code)
}

fn java_code_for_example(example: &Example) -> Option<String> {
    None
}

fn examples_as_comment(examples: Vec<Example>) -> String {
    let mut examples_comment: String = String::new();
    examples_comment += "/*\n";
    let mut i: usize = 0;
    let mut example: &Example;
    while i < examples.len() - 1 {
        example = &examples[i];
        examples_comment += example.to_string((i + 1).to_string()).as_str();
        examples_comment += "\n\n";
        i += 1;
    }

    example = &examples[i];
    examples_comment += example.to_string((i + 1).to_string()).as_str();
    examples_comment += "\n*/\n\n";

    examples_comment
}
