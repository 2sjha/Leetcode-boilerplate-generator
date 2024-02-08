pub const LANG_CPP: &str = "cpp";
pub const LANG_RUST: &str = "rust";
pub const LANG_JAVA: &str = "java";
pub const LANG_PYTHON: &str = "python";

pub const EXTNSN_CPP: &str = "cpp";
pub const EXTNSN_RUST: &str = "rs";
pub const EXTNSN_JAVA: &str = "java";
pub const EXTNSN_PYTHON: &str = "py";
pub const EXTNSN_TXT: &str = "txt";

pub fn extension_lang_map(language: &String) -> &str {
    match language.to_ascii_lowercase().as_str() {
        LANG_CPP => EXTNSN_CPP,
        LANG_RUST => EXTNSN_RUST,
        LANG_JAVA => EXTNSN_JAVA,
        LANG_PYTHON => EXTNSN_PYTHON,
        _ => EXTNSN_TXT,
    }
}

pub fn language_list() -> String {
    let mut list: String = String::new();
    list += "\"";
    list += LANG_CPP;
    list += "\", ";
    list += "\"";
    list += LANG_RUST;
    list += "\", ";
    list += "\"";
    list += LANG_JAVA;
    list += "\", ";
    list += "\"";
    list += LANG_PYTHON;
    list += "\"";

    list
}

pub fn generate_driver_code(examples: &String, starter_code: &String, language: &String) -> String {
    starter_code.to_string()
}

pub fn generate_description_as_comment(
    problem_url: &String,
    description: &String,
    language: &String,
) -> String {
    match language.to_ascii_lowercase().as_str() {
        LANG_CPP => get_generic_description_comment(problem_url, description),
        LANG_RUST => get_generic_description_comment(problem_url, description),
        LANG_JAVA => get_generic_description_comment(problem_url, description),
        LANG_PYTHON => get_python_description_comment(problem_url, description),
        _ => {
            println!("Language not supported yet. Generating generic starter code.");
            get_generic_description_comment(problem_url, description)
        }
    }
}

fn get_generic_description_comment(problem_url: &String, description: &String) -> String {
    let mut desc_comment: String = String::new();
    desc_comment += "/*\n";
    desc_comment += problem_url;
    desc_comment += "\n\n";
    desc_comment += description;
    desc_comment += "*/\n\n";

    desc_comment
}

fn get_python_description_comment(problem_url: &String, description: &String) -> String {
    let mut desc_comment: String = String::new();
    desc_comment += "\"\"\"\n";
    desc_comment += problem_url;
    desc_comment += "\n\n";
    desc_comment += description;
    desc_comment += "\"\"\"\n\n";

    desc_comment
}
