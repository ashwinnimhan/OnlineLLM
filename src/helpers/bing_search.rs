use anyhow::{anyhow, Result};
use reqwest;
use select::{document::Document, predicate::{Name, Class}};
use url::Url;
use chrono::{Duration, Local};
use regex::Regex;

use crate::helpers::common::NewsItem;

fn calculate_unix_timestamp(token: &str) -> Result<i64> {
    // Define regular expression to match the pattern
    let re = Regex::new(r"^(\d+)\s*(m|h|d|mon|y)$").unwrap();
    
    // Extract value and unit from the pattern
    let captures = re.captures(token).ok_or(anyhow!("Invalid time pattern"))?;
    let value: i64 = captures[1].parse()?;
    let unit = &captures[2];

    // Calculate the timestamp
    let now = Local::now();
    let timestamp = match unit {
        "m" => now - Duration::minutes(value),
        "h" => now - Duration::hours(value),
        "d" => now - Duration::days(value),
        "mon" => now - Duration::days(value * 30), // Approximation for months
        "y" => now - Duration::days(value * 365),
        _ => return Err(anyhow!("Unsupported unit").into()),
    };

    Ok(timestamp.timestamp())
}

pub async fn fetch(query: &str) -> Result<Vec<NewsItem>> {
    let url = format!("https://www.bing.com/news/search?q={}", &urlencoding::encode(query));
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    let document = Document::from(body.as_str());
    let news_nodes = document.find(Class("newsitem"));

    let mut news_items: Vec<NewsItem> = Vec::new();
    for node in  news_nodes {
      let title = node.attr("data-title").unwrap_or_default();
      let url = node.attr("data-url").unwrap_or_default();
      let source = node.attr("data-author").unwrap_or_default();
      let snippet: String = node.find(Class("snippet")).next().map(|n| n.text()).unwrap_or_default();
      let time: String = node.find(Class("source"))
                             .flat_map(|node| {
                                node.find(Name("span"))
                                    .filter(|n| n.attr("aria-label").is_some())
                                    .next()
                                    .map(|n| n.text())
                              })
                             .next()
                             .unwrap_or_default();

      calculate_unix_timestamp(&time);

      news_items.push(NewsItem {
        title: title.to_string(),
        url: url.to_string(),
        source: source.to_string(),
        snippet,
        time: calculate_unix_timestamp(&time).unwrap_or_default()
      });
    }

    news_items.sort_by(|a, b| a.time.cmp(&b.time));

    println!("Bing Fetched");
    Ok(news_items)
}
