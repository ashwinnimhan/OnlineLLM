// In common.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionRequest {
  pub model: String,
  pub messages: Vec<Message>,
  pub max_tokens: u32,
  pub temperature: f32,
  pub top_p: f32,
  pub n: u32,
  pub stream: bool,
  pub logprobs: Option<u32>,
  pub stop: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct Message {
  pub role: String,
  pub content: String,
}

impl Message {
  pub fn new(role: &str, content: &str) -> Self {
    Message {
      role: role.to_string(),
      content: content.to_string(),
    }
  }
}

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionResponse {
  pub id: String,
  pub object: String,
  pub created: u64,
  pub model: String,
  pub choices: Vec<Choice>,
  pub usage: Usage,
}

#[derive(Serialize, Deserialize)]
pub struct Choice {
  pub message: Message,
  pub index: u32,
  pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Usage {
  pub prompt_tokens: u32,
  pub completion_tokens: u32,
  pub total_tokens: u32,
}

#[derive(Clone)]
pub struct NewsItem {
  pub title: String,
  pub url: String,
  pub source: String,
  pub snippet: String,
  pub time: i64
}