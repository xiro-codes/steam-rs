use anyhow::Result;
use serde_derive::{Serialize, Deserialize};
use serde_json::Value;

use crate::Steam;

#[derive(Serialize, Deserialize, Debug)]
pub struct NewsItem {
    gid: String,
    title: String,
    url: String,
    is_external_url: bool,
    author: String,
    contents: String,
    feedlabel: String,
    date: u32,
    feedname: String,
    feed_type: u8,
    appid: u32,
    tags: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AppNews {
    appid: u32,
    newsitems: Vec<NewsItem>,
    count: u32
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    appnews: AppNews
}

impl Steam {
    pub async fn get_news_for_app(&self, appid: u32, count: u32, max_length: u32) -> Result<AppNews> {
        let url = format!("https://api.steampowered.com/ISteamNews/GetNewsForApp/v0002/?appid={}&count={}&maxlength={}",
            appid,
            count,
            max_length,
        );

        let json: Value = reqwest::get(url).await.unwrap().json().await.unwrap();

        let response: Response = serde_json::from_value(json).unwrap();

        Ok(response.appnews)
    }
}