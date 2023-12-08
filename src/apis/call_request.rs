use crate::models::general::llm::{Message, ChatCompletion, APIMessage, APIChoice, APIResponse};
use dotenv::dotenv;
use std::env;
use std::fmt::format;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue};

// Call large language model API GPT4
pub async fn call_gpt(messages: Vec<Message>) -> Result<String, Box<dyn std::error::Error + Send>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Extract API key and organization from environment variables
    let api_key = env::var("OPEN_AI_KEY")
        .expect("OPEN_AI_KEY not found in .env file");
    let api_org = env::var("OPEN_AI_ORG")
        .expect("OPEN_AI_ORG not found in .env file");

    // Define the API endpoint
    let url = "https://api.openai.com/v1/chat/completions";

    // Initialize an empty HeaderMap
    let mut headers = HeaderMap::new();

    // Insert the Authorization header
    headers.insert(
        "Authorization", 
        HeaderValue::from_str(&format!("Bearer {}", api_key))
            .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)})?,
    );

    // Insert the OpenAI-Organization header
    headers.insert(
        "OpenAI-Organization", 
        HeaderValue::from_str(api_org.as_str())
            .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)})?,
    );

    // Build the HTTP client with the default headers
    let client = Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)})?;

    // Create chat completion request
    let chat_completion = ChatCompletion {
        model: "gpt-3.5-turbo".to_string(),
        messages: messages,
        temperature: 0.1,
    };

    // Extract api response
    let res: APIResponse = client
        .post(url)
        .json(&chat_completion)
        .send()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)})?
        .json()
        .await
        .map_err(|e| -> Box<dyn std::error::Error + Send> {Box::new(e)})?;

    Ok(res.choices[0].message.content.clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_call_to_openai() {
        // call gpt to se if it responds
        let messages = Message {
            role: "user".to_string(),
            content: "Hello, this is a test, give me a short response".to_string(),
        };
        // make call and dbg! response
        let res = call_gpt(vec![messages]).await;
        match res {
            Ok(res) => {
                dbg!(res);
                assert!(true)
            },
            Err(e) => {
                dbg!(e);
                assert!(false)
            }
            
        }
    }
}