use anyhow::{anyhow, Result};
use reqwest;
use select::{document::Document, predicate::Name};
use url::Url;

pub async fn fetch(query: &str) -> Result<Vec<String>> {
    let url = format!("https://www.google.com/search?q={}&tbm=nws", &urlencoding::encode(query));

    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;

    let document = Document::from(body.as_str());

    println!("{}", body);

    let mut links: Vec<String> = Vec::new();

    for node in document.find(Name("a")) {
        if let Some(href) = node.attr("href") {
            if href.starts_with("/url") && 
               !href.contains("google") &&
               !href.contains("search") {
                let link = extract_page_link(href)?;
                links.push(link);
            }
        }
    }

    Ok(links)
}

fn extract_page_link(input: &str) -> Result<String> {
    let full_url = format!("http://example.com{}", input);
    let parsed_url  = Url::parse(&full_url)?;
    let query_params = parsed_url.query_pairs();
    let mut query: String = String::from(""); 

    for (key, value) in query_params {
      if key == "q" { query = value.into_owned(); }
    }
    
    Ok(query)
}
