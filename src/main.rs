mod api_calls;
mod generator;
mod genrtr_cpp;
mod genrtr_rust;
mod parser;
mod utils;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;
use surf::Client;
use surf_cookie_middleware::CookieMiddleware;
use surf_retry::{ExponentialBackoff, RetryMiddleware};

fn main() {
    env_logger::init();

    let args: Vec<String> = env::args().collect();
    let mut problem_url: String;
    let mut language: String;

    // Get Problem URL and Language from system args or console input
    if args.len() < 3 {
        print!("Leetcode Problem URL: ");
        let _ = io::stdout().flush();
        problem_url = String::new();
        let _input_result = io::stdin().read_line(&mut problem_url);
        problem_url = problem_url.trim().to_string();

        print!(
            "Language (Supported Languages are {}): ",
            utils::language_list_string()
        );
        let _ = io::stdout().flush();
        language = String::new();
        let _input_result = io::stdin().read_line(&mut language);
        language = language.trim().to_string();
    } else {
        problem_url = String::from(&args[1]);
        language = String::from(&args[2]);
    }

    let res = parse_and_generate(problem_url, language);
    if res.is_err() {
        eprintln!("Error: {}", res.unwrap_err().to_string());
    } else {
        println!(
            "\nFile with driver code generated successfully. You may want to format it according to your preferrence."
        );
    }
}

fn parse_and_generate(
    mut problem_url: String,
    language: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch problem title slug from the URL
    let problem_title: String = parser::get_title_slug(&problem_url)?;
    problem_url = parser::trim_problem_url(&problem_url);
    parser::validate_language(&language)?;

    // Make API Calls to fetch question content and editor code
    let retry_mw = RetryMiddleware::new(
        3,
        ExponentialBackoff::builder().build_with_max_retries(3),
        1,
    );
    let cookies_mw = CookieMiddleware::new();
    let client = Client::new().with(cookies_mw).with(retry_mw);
    let question_content: String = api_calls::get_question_content(&client, &problem_title)?;
    let question_editor_data: String =
        api_calls::get_question_editor_data(&client, &problem_title)?;
    let examples_data: String = api_calls::get_examples_data(&client, &problem_title)?;

    // Parse the JSON responses
    let mut description: String = parser::parse_question_content(&question_content)?;
    let output_values: Vec<String> = parser::parse_output_values_from_description(&description);
    let (starter_code, problem_number) =
        parser::parse_starter_code(&question_editor_data, &language)?;
    let examples: Vec<utils::Example> = parser::get_examples(&examples_data, output_values)?;

    // Generate full driver code and save it in a file
    description =
        generator::generate_description_as_comment(&mut problem_url, &description, &language);
    let driver_code: String = generator::generate_driver_code(examples, &starter_code, &language);
    let filename: String = format!(
        "{}-{}.{}",
        problem_number,
        problem_title,
        utils::extension_lang_map(&language)
    );
    let mut file: File = File::create(&filename)?;
    let _ = file.write_all(description.as_bytes());
    let _ = file.write_all(driver_code.as_bytes());

    Ok(())
}
