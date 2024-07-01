use anyhow::{Context, Result};
use select::document::Document;
use select::predicate::Name;

/** 
 * Accepts a list of urls and attempts crawl using the crawl_page function
 * Returns text from pages that were successfully crawled as Vec. */  
pub async fn fetch(urls: Vec<String>) -> Result<Vec<String>> {
  let mut text_content = Vec::new();
  for url in urls {
    match crawl_page(&url).await {
      Ok(text) => text_content.push(text),
      Err(err) => {
        eprintln!("Error crawling {}: {}", url, err);
        continue;
      }
    }
  }
  Ok(text_content)
}

/** 
 * Fetches page content and parses it to get plain text from body of page
 * Returns plain text from the crawled webpage */ 
async fn crawl_page(url: &str) -> Result<String> {
  let client = reqwest::Client::new();
  let response = client
      .get(url)
      .send()
      .await
      .with_context(|| format!("Failed to fetch {}", url))?;
  let body = response.text().await.with_context(|| format!("Failed to read response body for {}", url))?;

  let document = Document::from(body.as_str());
  let body_node = document.find(Name("body")).next();
  let mut plain_text = String::new();

  if let Some(body_node) = body_node {
    for node in body_node.descendants() {
      if Some(node).is_some() {
        if node.name().is_some() {
          if node.name().unwrap() == String::from("p") {
            plain_text.push_str(node.text().as_str());
          }
        }
      }
    }
  }

  let cleaned_text = plain_text.replace("\t", "")
  .replace("\n", " ")
  .replace("  ", " ")
  .trim()
  .to_string();

  Ok(cleaned_text)
}
