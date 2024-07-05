use reqwest::Error;
use crate::helpers::common::{ChatCompletionRequest, Message, ChatCompletionResponse, Choice, Usage};

pub async fn generate_chat_completion(
    api_key: &str,
    messages: Vec<(String, String)>,
) -> Result<ChatCompletionResponse, Error> {

  let chat_history: Vec<Message> = messages
  .iter()
  .map(|(content, role)| Message::new(role, content))
  .collect();

  let client = reqwest::Client::new();
  let request = ChatCompletionRequest {
    model: String::from("gpt-3.5-turbo"),
    messages: chat_history,
    max_tokens: 250,
    temperature: 0.7,
    top_p: 1.0,
    n: 1,
    stream: false,
    logprobs: None,
    stop: None,
  };

  let response = client
    .post("https://api.openai.com/v1/chat/completions")
    .header("Content-Type", "application/json")
    .header("Authorization", format!("Bearer {}", api_key))
    .json(&request)
    .send()
    .await?
    .json::<ChatCompletionResponse>()
    .await?;

  println!("LLM Response\n\n");
  Ok(response)
}