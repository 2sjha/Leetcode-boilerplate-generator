use crate::utils::{self, CustomError, Example};
use html2text::from_read;
use serde_json::Value;
use std::collections::HashMap;

const LC_PROBLEM_URL_PREFIX: &str = "https://leetcode.com/problems/";

pub fn validate_language(language: &String) -> Result<(), CustomError> {
    if utils::LANGUAGE_LIST.contains(&language.as_str()) {
        Ok(())
    } else {
        Err(CustomError::from_str("Language not supported yet"))
    }
}

pub fn get_title_slug(problem_url: &String) -> Result<String, CustomError> {
    let url_parts: Vec<&str> = problem_url.split('/').collect();

    // Example URL: https://leetcode.com/problems/remove-duplicates-from-sorted-array/?envType=study-plan-v2&envId=top-interview-150
    // ["https:", "", "leetcode.com", "problems", "remove-duplicates-from-sorted-array", "?envType=study-plan-v2&envId=top-interview-150\n"]
    if !problem_url.starts_with(LC_PROBLEM_URL_PREFIX) || url_parts.len() < 5 {
        Err(CustomError::from_str(
            "Invalid URL. This doesn't look like a Leetcode Problem URL.",
        ))
    } else {
        Ok(url_parts[4].to_string())
    }
}

fn get_text_from_html(question_content_as_html: &String) -> String {
    // TODO: Maybe make width configurable with language
    // Currently width=120 to avoid examples being cut from the middle
    // Generated description & code may be formatted anyway
    from_read(question_content_as_html.as_bytes(), 120)
}

pub fn parse_output_values_from_description(description: &String) -> Vec<String> {
    let mut output_values: Vec<String> = Vec::new();
    const OUTPUT_PREFIX: &str = "**Output:**";

    let lines = description.lines();
    for line in lines {
        if line.starts_with(OUTPUT_PREFIX) {
            let output: &str = &line[(OUTPUT_PREFIX.len() + 1)..];
            output_values.push(String::from(output));
        }
    }

    output_values
}

pub fn get_examples(
    examples_data: &String,
    output_values: Vec<String>,
) -> Result<Vec<Example>, Box<dyn std::error::Error>> {
    let parsed_json: Value = serde_json::from_str(examples_data)?;

    if parsed_json.get("data").is_none()
        || parsed_json.get("data").unwrap().get("question").is_none()
        || parsed_json
            .get("data")
            .unwrap()
            .get("question")
            .unwrap()
            .get("exampleTestcaseList")
            .is_none()
        || parsed_json
            .get("data")
            .unwrap()
            .get("question")
            .unwrap()
            .get("metaData")
            .is_none()
    {
        Err(CustomError::from_str("Couldn't read response JSON data."))?
    }

    let metadata_str = parsed_json["data"]["question"]["metaData"]
        .as_str()
        .unwrap();

    let metadata_json: Value = serde_json::from_str(metadata_str)?;
    if metadata_json.get("params").is_none()
        || !metadata_json["params"].is_array()
        || metadata_json.get("return").is_none()
        || metadata_json.get("name").is_none()
    {
        Err(CustomError::from_str("Couldn't read response JSON data."))?
    }

    if !parsed_json["data"]["question"]["exampleTestcaseList"].is_array() {
        Err(CustomError::from_str("Couldn't read response JSON data."))?
    }

    let mut input_var_types: Vec<(String, String)> = Vec::new();
    let mut input_var_values: HashMap<String, String> = HashMap::new();
    let output_type: String = String::from(metadata_json["return"]["type"].as_str().unwrap());
    let func_name: String = String::from(metadata_json["name"].as_str().unwrap());

    for param in metadata_json["params"].as_array().unwrap() {
        input_var_types.push((
            String::from(param["name"].as_str().unwrap()),
            String::from(param["type"].as_str().unwrap()),
        ));
    }

    let mut examples: Vec<Example> = Vec::new();
    let mut i: usize = 0;
    for example_testcase in parsed_json["data"]["question"]["exampleTestcaseList"]
        .as_array()
        .unwrap()
    {
        let example_testcase: &str = example_testcase.as_str().unwrap();
        let testcase_values: Vec<&str> = example_testcase.split("\n").collect();
        for i in 0..testcase_values.len() {
            input_var_values.insert(
                String::from(&input_var_types[i].0),
                String::from(testcase_values[i]),
            );
        }
        examples.push(Example::from(
            input_var_types.clone(),
            input_var_values.clone(),
            output_type.clone(),
            output_values[i].clone(),
            func_name.clone(),
        ));
        i += 1;
        input_var_values.clear();
    }

    Ok(examples)
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
        Err(CustomError::from_str("Couldn't read response JSON data."))?
    } else {
        let question_content_html = parsed_json["data"]["question"]["content"]
            .as_str()
            .unwrap()
            .to_string();

        Ok(get_text_from_html(&question_content_html))
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
        return Err(CustomError::from_str("Couldn't read response JSON data."))?;
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
        return Err(CustomError::from_str("Couldn't read response JSON data."))?;
    }

    if !parsed_json["data"]["question"]["codeSnippets"].is_array() {
        return Err(CustomError::from_str("Couldn't read response JSON data."))?;
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
        Err(CustomError::from_str("Language not supported yet"))?
    } else {
        Ok((starter_code, problem_number))
    }
}
