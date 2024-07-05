use anyhow::{Context, Result};
use reqwest::Error;
use select::document::Document;
use select::predicate::Name;
use std::time::Duration;
use url::Url;

pub async fn fetch(urls: Vec<String>) -> Result<Vec<String>> {
    let mut text_content = Vec::new();
    for url in urls {
        match Url::parse(&url) {
            Ok(_) => match crawl_page(&url).await {
                Ok(text) => text_content.push(text),
                Err(err) => {
                    eprintln!("Error crawling {}: {}", url, err);
                    continue;
                }
            },
            Err(_) => {
                eprintln!("Invalid URL: {}", url);
                continue;
            }
        }
    }
    Ok(text_content)
}

pub async fn crawl_page(url: &str) -> Result<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::limited(3))
        .build()
        .context("Failed to create HTTP client")?;

    let response = client
        .get(url)
        .send()
        .await
        .with_context(|| format!("Failed to fetch {}", url))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "HTTP error: {} for {}",
            response.status(),
            url
        ));
    }

    let body = response
        .text()
        .await
        .with_context(|| format!("Failed to read response body for {}", url))?;

    if body.is_empty() {
        return Err(anyhow::anyhow!("Empty response body for {}", url));
    }

    let document = Document::from(body.as_str());
    let body_node = document
        .find(Name("body"))
        .next()
        .ok_or_else(|| anyhow::anyhow!("No <body> tag found in {}", url))?;

    let mut plain_text = String::new();

    for node in body_node.descendants() {
        if Some(node).is_some() {
            if node.name().is_some() {
                if node.name().unwrap() == String::from("p") {
                    plain_text.push_str(node.text().as_str());
                }
            }
        }
    }

    let cleaned_text = plain_text
        .replace("\t", "")
        .replace("\n", " ")
        .replace("  ", " ")
        .trim()
        .to_string();

    Ok(cleaned_text)
}
