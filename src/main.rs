mod utils;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::fs::File;
use std::io;
use std::io::Write;
use utils::CustomError;

fn parse_url(problem_url: &String) -> Result<(String, String, String), CustomError> {
    let response_res = get(problem_url);
    if response_res.is_err() {
        return Err(CustomError::from(format!(
            "{}: {}",
            "Error Occured. Couldn't reach URL", problem_url
        )));
    }
    let response = response_res.unwrap();
    if !response.status().is_success() {
        return Err(CustomError::from(format!(
            "{}: {:?}",
            "Error Occured. HTTP Error Status",
            response.status()
        )));
    }

    let html_res = response.text();
    if html_res.is_err() {
        return Err(CustomError::from(format!(
            "{}: {}",
            "Error Occured. Couldn't read HTML from URL", problem_url
        )));
    }

    let html_body = html_res.unwrap();
    let document = Html::parse_document(&html_body);

    // PROBLEM: leetcode first renders a Dynamic Layout prompt. So cant directly fetch problem page.


    // Define a selector to select all <a> tags
    let a_selector = Selector::parse("a").unwrap();
    for a in document.select(&a_selector) {
        let href = a.value().attr("href").unwrap_or("");
        println!("Found link: {}", href);
    }

    Ok(("0".to_string(), "".to_string(), "".to_string()))
}

fn get_problem_name(problem_url: &String) -> Result<String, CustomError> {
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

fn main() -> Result<(), CustomError> {
    print!("Leetcode Problem URL: ");
    let _ = io::stdout().flush();
    let mut problem_url: String = String::new();
    let _input_result = io::stdin().read_line(&mut problem_url);

    let problem_name: String = get_problem_name(&problem_url)?;
    let (problem_number, description, starter_code) = parse_url(&problem_url)?;

    // TODO: Support more extensions [will need significant work]
    let extension = "cpp";
    let filename = format!("{}-{}.{}", problem_number, problem_name, extension);
    let file_res = File::create(&filename);
    if file_res.is_err() {
        return Err(CustomError::from(format!(
            "{}: {}",
            "Error occurred. Couldn't create file: ", filename
        )));
    }
    let mut file = file_res.unwrap();
    let _ = file.write_all(description.as_bytes());
    let _ = file.write_all(starter_code.as_bytes());

    Ok(())
}
