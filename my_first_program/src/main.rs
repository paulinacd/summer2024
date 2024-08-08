use dotenv::dotenv;
use std::env;
use serde::{Deserialize, Serialize};
use ureq;
use std::io::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    role: String,
    content: String,
}

#[derive(Serialize)]
struct RequestPayload {
    messages: Vec<Message>,
    temperature: f32,
    top_p: f32,
    max_tokens: u32,
}

#[derive(Deserialize, Debug)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: u32,
}

#[derive(Deserialize, Debug)]
struct ResponsePayload {
    id: String,
    object: String,
    created: u64,
    model: String,
    choices: Vec<Choice>,
    usage: Usage,
}

#[derive(Deserialize, Debug)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

fn get_user_input() -> String {
    "How to write a stack?".to_string()
}

fn generate_prompt(user_question:&str, language:&str)-> String(){
    format!("You are master of code completion, look into user question {}, try to help him as much as possible, he prefers the answer in language {}", user_question, language)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve the API endpoint and API key from environment variables
    let api_endpoint = env::var("API_ENDPOINT")
        .expect("API_ENDPOINT not set in .env file");
    let api_key = env::var("API_KEY")
        .expect("API_KEY not set in .env file");

    // Construct the request payload
    let payload = RequestPayload {
        messages: vec![
            Message {
                role: "user".to_string(),
                content: generate_prompt(get_user_input().as_str(), "Python"),
            },
        ],
        temperature: 0.7,
        top_p: 0.95,
        max_tokens: 800,
    };

    // Send the request again to parse the JSON
    let response = ureq::post(&api_endpoint)
        .set("Content-Type", "application/json")
        .set("api-key", &api_key)
        .send_json(&payload)?;

    // Handle the response
let response_from_server = match response.into_json::<ResponsePayload>() {
        Ok(response_payload) => {
            if let Some(choice) = response_payload.choices.first() {
                println!("Generated text: {}", choice.message.content);
            } else {
                println!("No response generated.");
            }
            println!("Full response: {:?}", response_payload);
        }
        Err(e) => {
            eprintln!("Failed to parse response: {}", e);
        }
    };

    let text = match response_from_server {
        Ok(r) => r.choices[0].message.content.clone(),
        Err(_) => panic!(),
    };

    println!("{}", text);

    Ok(())
}