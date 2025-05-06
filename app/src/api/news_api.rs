use itertools::Itertools;
use reqwest::Client;

use crate::{config::CONFIG, consts::*, errors::AppError, models::news_response::NewsResponse};

pub async fn get_news() -> Result<String, AppError> {
  let client = Client::builder().user_agent(USER_AGENT).build()?;
  let response = client
    .get(NEWS_API_URL)
    .query(&[
      ("country", "us"),
      ("pageSize", "5"),
      ("apiKey", &CONFIG.news_api_key),
    ])
    .send()
    .await?;
  let json = response.json::<NewsResponse>().await.map_err(|e| {
    println!("error: {}", e);
    AppError::ParseJsonError
  })?;
  match json {
    NewsResponse::Ok {
      total_results: _,
      articles,
    } => Ok(articles.into_iter().map(|a| a.title).join("    ")),
    NewsResponse::Error {
      code: _,
      message: _,
    } => Err(AppError::ApiError),
  }
}
