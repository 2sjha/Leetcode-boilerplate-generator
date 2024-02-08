mod api_calls;
mod generator;
mod parser;
mod utils;
use std::env;
use std::fs::File;
use std::io;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

        print!("\nLanguage (Supported Languages are {}): ", generator::language_list());
        let _ = io::stdout().flush();
        language = String::new();
        let _input_result = io::stdin().read_line(&mut language);
        language = language.trim().to_string();
    } else {
        problem_url = String::from(&args[1]);
        language = String::from(&args[2]);
    }

    // Fetch problem title slug from the URL
    let problem_title: String = parser::get_title_slug(&problem_url)?;

    // Make API Calls to fetch question content and editor code
    let question_content: String = api_calls::get_question_content(&problem_title)?;
    let question_editor_data: String = api_calls::get_question_editor_data(&problem_title)?;

    // Parse the JSON responses
    let mut description: String = parser::parse_question_content(&question_content)?;
    let (starter_code, problem_number) =
        parser::parse_starter_code(&question_editor_data, &language)?;
    let examples: String = parser::get_examples(&description);

    // Generate full driver code and save it in a file
    description = generator::generate_description_as_comment(&problem_url, &description, &language);
    let driver_code: String = generator::generate_driver_code(&examples, &starter_code, &language);
    let filename: String = format!(
        "{}-{}.{}",
        problem_number,
        problem_title,
        generator::extension_lang_map(&language)
    );
    let mut file: File = File::create(&filename)?;
    let _ = file.write_all(description.as_bytes());
    let _ = file.write_all(driver_code.as_bytes());

    Ok(())
}
