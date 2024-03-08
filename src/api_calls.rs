use crate::utils::CustomError;
use log::debug;
use serde_json::{json, Value};

const LC_GQL_URL: &str = "https://leetcode.com/graphql";

pub fn get_question_content(
    client: &surf::Client,
    title_slug: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = r#"
        query questionContent($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                content
            }
        }
    "#;
    let operation_name: String = String::from("questionContent");
    let request_body = json!({
        "query": query,
        "variables": gql_req_variables(title_slug),
        "operationName": operation_name
    });

    send_gql_request(client, request_body, operation_name)
}

pub fn get_question_editor_data(
    client: &surf::Client,
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
            }
        }
    "#;
    let operation_name: String = String::from("questionEditorData");
    let request_body: Value = json!({
        "query": query,
        "variables": gql_req_variables(title_slug),
        "operationName": operation_name
    });

    send_gql_request(client, request_body, operation_name)
}

pub fn get_examples_data(
    client: &surf::Client,
    title_slug: &String,
) -> Result<String, Box<dyn std::error::Error>> {
    let query = r#"
        query consolePanelConfig($titleSlug: String!) {
            question(titleSlug: $titleSlug) {
                questionId
                questionFrontendId
                questionTitle
                exampleTestcaseList
                metaData
            }
        }
    "#;
    let operation_name: String = String::from("consolePanelConfig");
    let request_body = json!({
        "query": query,
        "variables": gql_req_variables(title_slug),
        "operationName": operation_name
    });

    send_gql_request(client, request_body, operation_name)
}

#[tokio::main]
async fn send_gql_request(
    client: &surf::Client,
    request_body: Value,
    operation_name: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut response = client
        .post(LC_GQL_URL)
        .header("Accept-Encoding", "gzip, deflate")
        .header("content-type", "application/json")
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:123.0) Gecko/20100101 Firefox/123.0",
        )
        .body_json(&request_body)?
        .await?;
    if response.status().is_success() {
        let body: String = response.body_string().await?;
        debug!("{}", body);
        Ok(body)
    } else {
        debug!("{:?}", response);
        Err(CustomError::from(format!(
            "{} GraphQL request failed with status code: {}",
            operation_name,
            response.status()
        )))?
    }
}

fn gql_req_variables(title_slug: &String) -> Value {
    return json!({
        "titleSlug": title_slug
    });
}
