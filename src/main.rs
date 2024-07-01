use anyhow::Result;
use tokio;

mod helpers;
use helpers::google_search; 
use helpers::web_crawler;

#[tokio::main]
async fn main() -> Result<()> {
  let query = "Balimore Bridge Collapse";
  let links = google_search::fetch(query).await?;

  let text_content = web_crawler::fetch(links[3..5].to_vec()).await?;
  for (i, text) in text_content.iter().enumerate() {
    println!("Text content for link {}: {}", i, text);
  }
  Ok(())
}