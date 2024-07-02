use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, Responder, Result};

use crate::helpers::google_search;
use crate::helpers::web_crawler;
use crate::helpers::llm_request;

use crate::helpers::common::{ChatCompletionRequest, Message, ChatCompletionResponse, Choice, Usage};


pub async fn chat_completion(req: web::Json<ChatCompletionRequest>) -> impl Responder {
  let mut query: String = String::from("");
  if req.messages[0].role == "user".to_string() {
      query = req.messages[0].content.clone();
  }

  println!("Query: {}", query);

  // Step 1: Use ggls to fetch search results
  let links = match google_search::fetch(&query).await {
      Ok(links) => links,
      Err(e) => return HttpResponse::InternalServerError().json(format!("Error in ggls: {}", e)),
  };

  println!("Search results (links):");
  for link in &links {
      println!("- {}", link);
  }

  // Step 2: Use web_crawler to fetch content from the links
  let text_content = match web_crawler::fetch(links[0..5].to_vec()).await {
      Ok(content) => content,
      Err(e) => return HttpResponse::InternalServerError().json(format!("Error in web_crawler: {}", e)),
  };

  println!("Fetched content:");
  for content in &text_content {
      println!("{}", content);
  }

  // Step 3: Prepare the prompt for LLM
  let combined_text = text_content.join("\n\n");
  let prompt = vec![
      (format!("Based on the following information, {}", query), String::from("user")),
      (combined_text, String::from("user")),
  ];

  println!("Prompt:");
  println!("Role: {}", prompt[0].1);
  println!("{}", prompt[0].0);

  // Step 4: Send request to LLM
  let api_key = String::from("your-api-key-here"); // Replace with actual API key
  let result = match llm_request::generate_chat_completion(&api_key, prompt).await {
      Ok(result) => result,
      Err(e) => return HttpResponse::InternalServerError().json(format!("Error in LLM request: {}", e)),
  };

  println!("LLM response:");
  for choice in &result.choices {
      println!("Choice:");
      println!("Role: {}", choice.message.role);
      println!("{}", choice.message.content);
  }

  HttpResponse::Ok().json(result)
}

pub async fn invalid_route() -> Result<HttpResponse> {
  Ok(HttpResponse::NotFound().body("404 Not Found"))
}

pub async fn invalid_method() -> Result<HttpResponse> {
  Ok(HttpResponse::MethodNotAllowed().body("405 Method Not Allowed"))
}