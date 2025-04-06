use dotenv::dotenv;
use reqwest;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct CryptoInfo {
    pub name: String,
    pub symbol: String,
    pub price: f64,
    pub market_cap: f64,
    pub volume_24h: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewsArticle {
    pub title: String,
    pub source: String,
    pub published_at: String,
    pub description: String,
    pub url: String,
}

pub async fn fetch_crypto_data() -> Result<Vec<CryptoInfo>, reqwest::Error> {
    dotenv().ok();
    let api_key = env::var("COINMARKETCAP_API_KEY").expect("COINMARKETCAP_API_KEY not set in .env");

    let api_url = "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest";

    let client = reqwest::Client::new();
    let response = client
        .get(api_url)
        .header("Accept", "application/json")
        .header("X-CMC_PRO_API_KEY", api_key)
        .send()
        .await?;

    let data: serde_json::Value = response.json().await?;

    let mut crypto_list = Vec::new();
    if let Some(coins) = data["data"].as_array() {
        for coin in coins {
            let crypto = CryptoInfo {
                name: coin["name"].as_str().unwrap_or("Unknown").to_string(),
                symbol: coin["symbol"].as_str().unwrap_or("Unknown").to_string(),
                price: coin["quote"]["USD"]["price"].as_f64().unwrap_or(0.0),
                market_cap: coin["quote"]["USD"]["market_cap"].as_f64().unwrap_or(0.0),
                volume_24h: coin["quote"]["USD"]["volume_24h"].as_f64().unwrap_or(0.0),
            };
            crypto_list.push(crypto);
        }
    }

    Ok(crypto_list)
}

pub async fn fetch_news_data(crypto_name: &str) -> Result<Vec<NewsArticle>, reqwest::Error> {
    dotenv().ok();
    let gnews_api_key = env::var("GNEWS_API_KEY").expect("GNEWS_API_KEY not set in .env");

    let url = format!(
        "https://gnews.io/api/v4/search?q={}&lang=en&max=5&token={}",
        crypto_name, gnews_api_key
    );

    let client = reqwest::Client::new();
    let response = client.get(&url).send().await?;
    let data: serde_json::Value = response.json().await?;

    let mut news_list = Vec::new();
    if let Some(articles) = data["articles"].as_array() {
        for article in articles {
            let news = NewsArticle {
                title: article["title"].as_str().unwrap_or("No title").to_string(),
                source: article["source"]["name"]
                    .as_str()
                    .unwrap_or("Unknown")
                    .to_string(),
                published_at: article["publishedAt"]
                    .as_str()
                    .unwrap_or("Unknown")
                    .to_string(),
                description: article["description"]
                    .as_str()
                    .unwrap_or("No description")
                    .to_string(),
                url: article["url"].as_str().unwrap_or("").to_string(),
            };
            news_list.push(news);
        }
    }

    Ok(news_list)
}
