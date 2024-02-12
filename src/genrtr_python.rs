use crate::generator;
use crate::utils::Example;

pub fn python_driver_code(examples: Vec<Example>, starter_code: &String) -> String {
    starter_code.clone()
}

fn python_code_for_example(example: &Example) -> Option<String> {
    None
}

pub fn python_description_comment(problem_url: &String, description: &String) -> String {
    format!("\"\"\"\n{}\n\n{}\"\"\"\n\n", problem_url, description)
}

pub fn examples_as_python_comment(examples: Vec<Example>) -> String {
    let mut examples_comment: String = format!("\"\"\"\n");
    let mut i: usize = 0;
    let mut example: &Example;
    while i < examples.len() - 1 {
        example = &examples[i];
        examples_comment += format!("{}\n\n", example.to_string(i + 1)).as_str();
        i += 1;
    }

    example = &examples[i];
    examples_comment += format!("{}\n\"\"\"\n\n", example.to_string(i + 1)).as_str();

    examples_comment
}