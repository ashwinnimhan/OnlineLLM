use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, Responder, Result};

use crate::helpers::google_search;
use crate::helpers::bing_search;
use crate::helpers::yahoo_search;
use crate::helpers::web_crawler;
use crate::helpers::llm_request;

use crate::helpers::common::{ChatCompletionRequest, Message, ChatCompletionResponse, Choice, Usage, NewsItem};

pub async fn chat_completion(req: web::Json<ChatCompletionRequest>) -> impl Responder {
  let mut query: String = String::from("");
  if req.messages[0].role == "user".to_string() {
      query = req.messages[0].content.clone();
  }
  println!("Query: {}", query);

  // Step 1: Use bing and yahoo search to fetch search results
  let mut bing_records: Vec<NewsItem> = match bing_search::fetch(&query).await {
    Ok(l) => l,
    Err(e) => return HttpResponse::InternalServerError().json(format!("Bing Search Error: {}", e)),
  };
  let yahoo_records: Vec<NewsItem> = match yahoo_search::fetch(&query).await {
    Ok(l) => l,
    Err(e) => return HttpResponse::InternalServerError().json(format!("Yahoo Search Error: {}", e)),
  };

  // Take the Top 5 Bing links by timestamp, and Top 3 from Yahoo!
  let mut combined = bing_records[0..5].to_vec();
  combined.extend_from_slice(&yahoo_records[0..3]); 

  // Step 2: Crawl text from the 8 search results.
  let mut pages: Vec<String> = vec![];
  for entry in &combined {
    let text_content = match web_crawler::crawl_page(&entry.url).await {
      Ok(content) => content,
      Err(e) => return HttpResponse::InternalServerError().json(format!("Error in web_crawler: {}", e)),
    };
    pages.push(format!("- Title: {}\n - Publisher: {}\n - url: {}\n - content: {}", entry.title, entry.source, entry.url, text_content));
  }

  // Step 3: Prepare the prompt for LLM
  let mut prompt = vec![
    (format!("Based on the following information, {}", query), String::from("user")),
  ];
  for pg in &pages {
    prompt.push((pg.to_string(), String::from("user")));
  }

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