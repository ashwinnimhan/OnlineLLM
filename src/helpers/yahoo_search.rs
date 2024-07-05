use anyhow::{anyhow, Result};
use reqwest;
use select::{document::Document, predicate::{Name, Class}};
use url::Url;
use chrono::{Duration, Local};
use regex::Regex;

use crate::helpers::common::NewsItem;

fn calculate_unix_timestamp(token: &str) -> Result<i64> {
  // Define regular expression to match the pattern
  let re = Regex::new(r"^· (\d+) (minute|hour|day|month|year)s? ago$").unwrap();
  
  // Extract minutes, hours, days, months, or years from the pattern
  let captures = re.captures(token).ok_or(anyhow!("Invalid time pattern"))?;
  let value: i64 = captures[1].parse()?;
  let unit = &captures[2];

  // Calculate the timestamp
  let now = Local::now();
  let timestamp = match unit {
      "minute" => now - Duration::minutes(value),
      "hour" => now - Duration::hours(value),
      "day" => now - Duration::days(value),
      "month" => now - Duration::days(value * 30), // Approximation for months
      "year" => now - Duration::days(value * 365),
      _ => return Err(anyhow!("Unsupported unit").into()),
  };

  Ok(timestamp.timestamp())
}

pub async fn fetch(query: &str) -> Result<Vec<NewsItem>> {
    let url = format!("https://news.search.yahoo.com/search?p={}", &urlencoding::encode(query));
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    let body = response.text().await?;
    let document = Document::from(body.as_str());
    let news_nodes = document.find(Class("NewsArticle"));

    let mut news_items: Vec<NewsItem> = Vec::new();
    for node in  news_nodes {
      let source = node.find(Class("s-source")).next().map(|elem| elem.text()).unwrap_or_default();

      let link = node.find(Class("thmb")).next();
      let url = link.map_or("", |l| l.attr("href").unwrap_or_default());
      let title = link.map_or("", |l| l.attr("title").unwrap_or_default());

      let snippet = node.find(Class("s-desc")).next().map(|elem| elem.text()).unwrap_or_default();
      let time = node.find(Class("s-time")).next().map(|elem| elem.text()).unwrap_or_default();

      news_items.push(NewsItem {
        title: title.to_string(),
        url: url.to_string(),
        source,
        snippet,
        time: calculate_unix_timestamp(&time).unwrap_or_default()
      });
    }

    news_items.sort_by(|a, b| a.time.cmp(&b.time));

    println!("Yahoo Fetched");
    Ok(news_items)
}
