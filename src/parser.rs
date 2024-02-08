use crate::utils::CustomError;
use serde_json::Value;

const LC_PROBLEM_URL_PREFIX: &str = "https://leetcode.com/problems/";

pub fn get_title_slug(problem_url: &String) -> Result<String, CustomError> {
    let url_parts: Vec<&str> = problem_url.split('/').collect();

    // Example URL: https://leetcode.com/problems/remove-duplicates-from-sorted-array/?envType=study-plan-v2&envId=top-interview-150
    // ["https:", "", "leetcode.com", "problems", "remove-duplicates-from-sorted-array", "?envType=study-plan-v2&envId=top-interview-150\n"]
    if !problem_url.starts_with(LC_PROBLEM_URL_PREFIX) || url_parts.len() < 5 {
        return Err(CustomError::from(
            "Invalid URL. This doesn't look like a Leetcode Problem URL.".to_string(),
        ));
    }

    Ok(url_parts[4].to_string())
}

fn clean_description(question_content: &String) -> String {
    // let mut clean_question_content: String;

    question_content.to_string()
}

pub fn get_examples(description: &String) -> String {
    "".to_string()
}

pub fn parse_question_content(
    question_content: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let parsed_json: Value = serde_json::from_str(question_content)?;

    if parsed_json.get("data").is_none()
        || parsed_json.get("data").unwrap().get("question").is_none()
        || parsed_json
            .get("data")
            .unwrap()
            .get("question")
            .unwrap()
            .get("content")
            .is_none()
    {
        Err(CustomError::from(
            "Couldn't read response JSON data.".to_string(),
        ))?
    } else {
        let q_content = parsed_json["data"]["question"]["content"]
            .as_str()
            .unwrap()
            .to_string();

        Ok(clean_description(&q_content))
    }
}

pub fn parse_starter_code(
    question_editor_data: &String,
    language: &String,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let parsed_json: Value = serde_json::from_str(question_editor_data)?;

    let problem_number: String;
    let mut starter_code: String = String::new();

    if parsed_json.get("data").is_none()
        || parsed_json.get("data").unwrap().get("question").is_none()
        || parsed_json
            .get("data")
            .unwrap()
            .get("question")
            .unwrap()
            .get("questionFrontendId")
            .is_none()
    {
        return Err(CustomError::from(
            "Couldn't read response JSON data.".to_string(),
        ))?;
    } else {
        problem_number = parsed_json["data"]["question"]["questionFrontendId"]
            .as_str()
            .unwrap()
            .to_string();
    }

    if parsed_json
        .get("data")
        .unwrap()
        .get("question")
        .unwrap()
        .get("codeSnippets")
        .is_none()
    {
        return Err(CustomError::from(
            "Couldn't read response JSON data.".to_string(),
        ))?;
    }

    if !parsed_json["data"]["question"]["codeSnippets"].is_array() {
        return Err(CustomError::from(
            "Couldn't read response JSON data.".to_string(),
        ))?;
    }

    let code_snippets: &Vec<Value> = parsed_json["data"]["question"]["codeSnippets"]
        .as_array()
        .unwrap();
    let mut extnsn_found: bool = false;
    for c_snippet in code_snippets {
        if c_snippet.get("langSlug").is_some()
            && c_snippet["langSlug"].as_str().unwrap().to_ascii_lowercase()
                == language.to_ascii_lowercase()
            && c_snippet.get("code").is_some()
        {
            extnsn_found = true;
            starter_code = c_snippet["code"].as_str().unwrap().to_string();
        }
    }

    if !extnsn_found {
        return Err(CustomError::from("Language not supported yet".to_string()))?;
    }

    Ok((starter_code, problem_number))
}
