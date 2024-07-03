// In common.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChatCompletionRequest {
  #[serde(default)]
  pub model: String,
  #[serde(default)]
  pub messages: Vec<Message>,
  #[serde(default)]
  pub max_tokens: u32,
  #[serde(default)]
  pub temperature: f32,
  #[serde(default)]
  pub top_p: f32,
  #[serde(default)]
  pub n: u32,
  #[serde(default)]
  pub stream: bool,
  #[serde(default)]
  pub logprobs: Option<u32>,
  #[serde(default)]
  pub stop: Option<Vec<String>>,
}

impl Default for ChatCompletionRequest {
  fn default() -> Self {
    Self {
      model: "default_model".to_string(),
      messages: Vec::new(), 
      max_tokens: 100,
      temperature: 1.0,
      top_p: 0.9,
      n: 1,
      stream: false,
      logprobs: None,
      stop: None,
    }
  }
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