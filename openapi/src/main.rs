/*
 */

 use reqwest::{header, Client};
 use serde_json::json;
 use std::env;
 
 #[tokio::main]
 async fn main() -> Result<(), reqwest::Error> {
     let client = Client::new();
 
     //use env variable OPENAI_API_KEY
     let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
     let url = "https://api.openai.com/v1/completions";
 
     let json = json!({
         "model": "text-davinci-003",
         "prompt": "ML Tutor: I am a ML/AI language model tutor\nYou: What is a language model?\nML Tutor: A language model is a statistical model that describes the probability of a word given the previous words.\nYou: What is a statistical model?",
         "temperature": 0.7,
         "max_tokens": 256,
         "top_p": 1,
         "frequency_penalty": 0,
         "presence_penalty": 0
     });
 
     let response = client
         .post(url)
         .header(header::AUTHORIZATION, format!("Bearer {api_key}"))
         .header(header::CONTENT_TYPE, "application/json")
         .json(&json)
         .send()
         .await?;
 
     let body = response.text().await?;
     println!("{body}");
 
     Ok(())
 }