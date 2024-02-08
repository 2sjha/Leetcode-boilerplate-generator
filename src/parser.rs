use std::fmt::format;

use serde_json::Value;

use crate::utils::CustomError;

pub fn parse_problem_number(question_info: &String) -> Result<String, Box<dyn std::error::Error>> {
    let parsed_json: Value = serde_json::from_str(question_info)?;
    if parsed_json["data"].as_str().is_none()
        || parsed_json["data"]["question"].as_str().is_none()
        || parsed_json["data"]["question"]["questionFrontendId"]
            .as_str()
            .is_none()
    {
        Err(CustomError::from(
            "Couldn't read response JSON data.".to_string(),
        ))?
    } else {
        Ok(parsed_json["data"]["question"]["questionFrontendId"]
            .as_str()
            .unwrap()
            .to_string())
    }
}

pub fn parse_description(
    problem_url: &String,
    extension: &String,
    question_content: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok("".to_string())
}

pub fn parse_examples(description: &String) -> Result<String, Box<dyn std::error::Error>> {
    Ok("".to_string())
}

pub fn parse_starter_code(
    question_editor_data: &String,
    extension: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok("".to_string())
}
