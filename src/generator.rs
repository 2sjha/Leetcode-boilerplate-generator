use crate::{genrtr_cpp, genrtr_python, genrtr_java, genrtr_rust, utils::{self, Example}};

pub fn generate_driver_code(
    examples: Vec<Example>,
    starter_code: &String,
    language: &String,
) -> String {
    match language.to_ascii_lowercase().as_str() {
        utils::LANG_CPP => genrtr_cpp::cpp_driver_code(examples, starter_code),
        utils::LANG_RUST => genrtr_rust::rust_driver_code(examples, starter_code),
        utils::LANG_JAVA => genrtr_java::java_driver_code(examples, starter_code),
        utils::LANG_PYTHON => genrtr_python::python_driver_code(examples, starter_code),
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
        utils::LANG_PYTHON => genrtr_python::python_description_comment(problem_url, description),
        _ => "".to_string(), // Won't happen
    }
}

fn generic_description_comment(problem_url: &String, description: &String) -> String {
    format!("/*\n{}\n\n{}*/\n\n", problem_url, description)
}

pub fn examples_as_comment(examples: Vec<Example>) -> String {
    let mut examples_comment: String = format!("/*\n");
    let mut i: usize = 0;
    let mut example: &Example;
    while i < examples.len() - 1 {
        example = &examples[i];
        examples_comment += format!("{}\n\n", example.to_string(i + 1)).as_str();
        i += 1;
    }

    example = &examples[i];
    examples_comment += format!("{}\n*/\n\n", example.to_string(i + 1)).as_str();

    examples_comment
}
