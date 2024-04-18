use crate::models::general::llm::{APIResponse, ChatCompletion, Message};
use dotenv::dotenv;
use reqwest::Client;
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};

// Call Large Language Model (i.e. GPT-4)
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    dotenv().ok();

    // Extract API Key Information
    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set");
    let api_org = env::var("OPENAI_API_ORG").expect("OPENAI_API_ORG not set");

    // Confirm endpoint
    let url = "https://api.openai.com/v1/chat/completions";

    // Create headers
    let mut headers = HeaderMap::new();

    // Create api key header
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create Open AI Org header
    headers.insert(
        "OpenAI-Organization",
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?,
    );

    // Create client
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // Create chat completion
    let chat_completion = ChatCompletion {
        model: "gpt-4-turbo".to_string(),
        messages,
        temperature: 0.1,
    };

    // // Troubleshooting
    // let res_raw = client
    //     .post(url)
    //     .json(&chat_completion)
    //     .send()
    //     .await
    //     .unwrap();

    // dbg!(res_raw.text().await.unwrap());

    // Extract API Response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> { Box::new(e) })?;

    // return response
    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use std::panic::resume_unwind;

    use super::*;

    #[tokio::test]
    async fn test_call_gpt() {
        let messages = vec![Message {
            role: "user".to_string(),
            content: "Hi there, this is a test. Give me a short response".to_string(),
        }];

        let response = call_gpt(messages).await;
        match response {
            Ok(res_str) => {
                dbg!(res_str);
            }
            Err(e) => {
                dbg!(e);
                assert!(false);
            }
        }
    }
}
