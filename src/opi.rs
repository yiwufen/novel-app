use reqwest::{Proxy, Client};
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use std::env;
use serde_derive::{Deserialize,Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Serialize, Deserialize,  Debug)]
pub struct OAIbody {
    pub model: String,
    pub messages: Vec<Message>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct OAIChoice {
    pub index: u8,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
struct OAIResponse {
    choices: Vec<OAIChoice>
}

pub async fn sentmessages(oaibody: & OAIbody) -> Result<Message, Box<dyn std::error::Error + Send + Sync>> {
    let uri = "https://api.openai.com/v1/chat/completions";

    let oai_token: String = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY not set in env");
    let auth_header_val = format!("Bearer {}", oai_token);

    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth_header_val).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

    let proxy = "http://127.0.0.1:7890";

    let client = Client::builder()
        .proxy(Proxy::all(proxy)?)
        .build()?;

    let  resp = client
        .post(uri)
        .headers(headers.clone())
        .json(& oaibody)
        .send()
        .await?;
    
    let resp = resp.json::<OAIResponse>().await.expect("Failed to parse response");

    let choices = resp.choices;
    // println!("{:?}", choices);
    let message =  choices[0].message.clone();
    // println!("{:?}", message);
    Ok(message)
}