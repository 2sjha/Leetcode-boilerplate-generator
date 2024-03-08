use crate::utils::CustomError;
use log::debug;
use serde_json::json;

const LC_GQL_URL: &str = "https://leetcode.com/graphql";

#[tokio::main]
pub async fn get_question_content(
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
    let variables = json!({
        "titleSlug": title_slug
    });

    let request_body = json!({
        "query": query,
        "variables": variables,
        "operationName": "questionContent"
    });

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
        return Ok(body);
    } else {
        println!("{:?}", response);
    }

    response = client
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
        return Ok(body);
    } else {
        println!("{:?}", response);
        return Err(CustomError::from(format!(
            "questionContent GraphQL request failed with status code: {}",
            response.status()
        )))?;
    }
}

#[tokio::main]
pub async fn get_question_editor_data(
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
    let variables = json!({
        "titleSlug": title_slug
    });

    let request_body = json!({
        "query": query,
        "variables": variables,
        "operationName": "questionEditorData"
    });

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
        return Ok(body);
    } else {
        println!("{:?}", response);
    }

    response = client
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
        return Ok(body);
    } else {
        return Err(CustomError::from(format!(
            "questionEditorData GraphQL request failed with status code: {}",
            response.status()
        )))?;
    }
}

#[tokio::main]
pub async fn get_examples_data(
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

    let variables = json!({
        "titleSlug": title_slug
    });
    let request_body = json!({
        "query": query,
        "variables": variables,
        "operationName": "consolePanelConfig"
    });

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
        return Ok(body);
    } else {
        println!("{:?}", response);
    }

    response = client
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
        return Ok(body);
    } else {
        return Err(CustomError::from(format!(
            "consolePanelConfig GraphQL request failed with status code: {}",
            response.status()
        )))?;
    }
}
