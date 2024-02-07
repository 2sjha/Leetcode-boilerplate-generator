mod api_calls;
mod generator;
mod parser;
mod utils;
use std::fs::File;
use std::io;
use std::io::Write;
use utils::CustomError;

fn get_title_slug(problem_url: &String) -> Result<String, CustomError> {
    let url_parts: Vec<&str> = problem_url.split('/').collect();

    // Exxample URL: https://leetcode.com/problems/remove-duplicates-from-sorted-array/?envType=study-plan-v2&envId=top-interview-150
    // ["https:", "", "leetcode.com", "problems", "remove-duplicates-from-sorted-array", "?envType=study-plan-v2&envId=top-interview-150\n"]
    if url_parts.len() < 5 || url_parts[0] != "https:" || url_parts[2] != "leetcode.com" {
        return Err(CustomError::from(
            "Invalid URL. This doesn't look like a Leetcode Problem URL.".to_string(),
        ));
    }

    Ok(url_parts[4].to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Leetcode Problem URL: ");
    let _ = io::stdout().flush();
    let mut problem_url: String = String::new();
    let _input_result = io::stdin().read_line(&mut problem_url);

    let problem_name: String = get_title_slug(&problem_url)?;
    let question_info: String = api_calls::get_question_info(&problem_name)?;
    let question_content: String = api_calls::get_question_content(&problem_name)?;
    let question_editor_data: String = api_calls::get_question_editor_data(&problem_name)?;

    let extension: String = "cpp".to_string();
    let problem_number: String = parser::parse_problem_number(&question_info);
    let description: String =
        parser::parse_description(&problem_url, &extension, &question_content);
    let examples: String = parser::parse_examples(&description);
    let starter_code = parser::parse_starter_code(&question_editor_data, &extension);
    let driver_code = generator::generate_driver_code(&examples, &extension);

    let filename = format!("{}-{}.{}", problem_number, problem_name, extension);
    let mut file = File::create(&filename)?;
    let _ = file.write_all(description.as_bytes());
    let _ = file.write_all(starter_code.as_bytes());
    let _ = file.write_all(driver_code.as_bytes());

    Ok(())
}
