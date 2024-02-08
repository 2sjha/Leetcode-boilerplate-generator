use crate::utils::CustomError;
use log::debug;
use reqwest::{
    header::{self},
    Client,
};
use serde_json::json;

const LC_GQL_URL: &str = "https://leetcode.com/graphql/problems";

fn get_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("application/json"),
    );

    headers
}

#[tokio::main]
pub async fn get_question_content(
    title_slug: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = r#"
        query questionContent($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                content
                mysqlSchemas
                dataSchemas
            }
        }
    "#;

    let variables = json!({
        "titleSlug": title_slug
    });
    let request_body = json!({
        "query": query,
        "variables": variables
    });

    let client = Client::new();
    let response = client
        .post(format!("{}/{}", LC_GQL_URL, title_slug))
        .headers(get_headers())
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let body: String = response.text().await?;
        debug!("{}", body);
        return Ok(body);
    } else {
        return Err(CustomError::from(format!(
            "GraphQL request failed with status code: {}",
            response.status()
        )))?;
    }
}

#[tokio::main]
pub async fn get_question_editor_data(
    title_slug: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = r#"
        query questionEditorData($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                questionId
                questionFrontendId
                codeSnippets {
                    lang
                    langSlug
                    code
                }
                envInfo
                enableRunCode
                hasFrontendPreview
                frontendPreviews
            }
        }
    "#;

    let variables = json!({
        "titleSlug": title_slug
    });
    let request_body = json!({
        "query": query,
        "variables": variables
    });

    let client = Client::new();
    let response = client
        .post(format!("{}/{}", LC_GQL_URL, title_slug))
        .headers(get_headers())
        .json(&request_body)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        debug!("{}", body);
        return Ok(body);
    } else {
        return Err(CustomError::from(format!(
            "GraphQL request failed with status code: {}",
            response.status()
        )))?;
    }
}
