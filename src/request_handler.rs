use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, Responder, Result};
use std::env;

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
  println!("Query Recieved {}", &query);
  // Step 1: Use bing and yahoo search to fetch search results
  let mut bing_records: Vec<NewsItem> = match bing_search::fetch(&query).await {
      Ok(l) => l,
      Err(e) => {
        eprint!("Bing Search Error: {}", e);
        Vec::new() // Continue with an empty vector
      },
  };
  let yahoo_records: Vec<NewsItem> = match yahoo_search::fetch(&query).await {
      Ok(l) => l,
      Err(e) => {
        eprint!("Yahoo Search Error: {}", e);
        Vec::new() // Continue with an empty vector
      },
  };
  // Take the Top 5 Bing links by timestamp, and Top 3 from Yahoo!
  let mut combined = Vec::new();
  // Add up to 5 elements from bing_records
  let bing_slice = if bing_records.len() >= 5 {
    &bing_records[0..5]
  } else {
    &bing_records[..]
  };
  combined.extend_from_slice(bing_slice);
  // Add up to 3 elements from yahoo_records
  let yahoo_slice = if yahoo_records.len() >= 3 {
    &yahoo_records[0..3]
  } else {
    &yahoo_records[..]
  };
  combined.extend_from_slice(yahoo_slice);
  // Step 2: Crawl text from the 8 search results.
  let mut pages: Vec<String> = vec![];
  if !combined.is_empty() {
    for entry in &combined {
      println!("{}, {}, {}", &entry.title, &entry.source, &entry.url);
      
      if entry.url.is_empty() {
        eprintln!("Skipping entry with empty URL");
        continue;
      }
      
      let text_content = match web_crawler::crawl_page(&entry.url).await {
        Ok(content) => content,
        Err(e) => {
          eprintln!("Error in web_crawler: {}", e);
          continue;
        }
      };
      pages.push(format!(
        " - Title: {}\n - Publisher: {}\n - content: {}",
        entry.title, entry.source, text_content
      ));
    }
  }
  // Step 3: Prepare the prompt for LLM
  let mut prompt = vec![
    (format!("NOTE: DO NOT CRAWL LINKS. Based on the following information, and the news article's metadata {}", query), String::from("user")),
  ];
  if !pages.is_empty() {
    for pg in &pages {
      let length = 1500;
      let pg_cut = if pg.len() > length { &pg.chars().take(length).collect() } else { pg };
      prompt.push((pg_cut.to_string(), String::from("user")));
    }
  } else {
    let no_context_message = "no context found - just answer based on your knowledge";
    prompt.push((no_context_message.to_string(), String::from("user")));
  }
  
  // Step 4: Send request to LLM
  let api_key = match env::var("OPENAI_API_KEY") {
    Ok(value) => { value }
    Err(e) => { "".to_string() }
  };
  println!("LLM Request Submitted");
  let result = match llm_request::generate_chat_completion(&api_key, prompt).await {
    Ok(result) => result,
    Err(e) => return HttpResponse::InternalServerError().json(format!("Error in LLM request: {}", e)),
  };
  HttpResponse::Ok().json(result)
}

pub async fn invalid_route() -> Result<HttpResponse> {
  Ok(HttpResponse::NotFound().body("404 Not Found"))
}

pub async fn invalid_method() -> Result<HttpResponse> {
  Ok(HttpResponse::MethodNotAllowed().body("405 Method Not Allowed"))
}
