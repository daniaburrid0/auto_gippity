use crate::models::general::llm::Message;
use dotenv::dotenv;
use std::env;
use reqwest::Client;

// Call large language model API GPT4
pub async fn call_gpt(messages: Vec<Message>) {
    dotenv().ok();
    
    //  Extract api key from .env file
    let api_key = env::var("OPEN_AI_KEY").expect("OPEN_AI_KEY not found in .env file");
    let api_org = env::var("OPEN_AI_ORG").expect("OPEN_AI_ORG not found in .env file");

    // confirm endpoint
    let url = "https://api.openai.com/v1/chat/completions";
}