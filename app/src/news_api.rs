use itertools::Itertools;

use crate::{errors::AppError, models::news_response::NewsResponse};

pub async fn get_news() -> Result<String, AppError> {
  let url = format!(
    "https://newsapi.org/v2/top-headlines?country=us&apiKey={}",
    crate::config::CONFIG.news_api_key
  );
  let response = reqwest::get(&url).await?;
  let json = response
    .json::<NewsResponse>()
    .await
    .map_err(|_| AppError::ParseJsonError)?;
  match json {
    NewsResponse::Ok {
      total_results: _,
      articles,
    } => Ok(articles.into_iter().map(|a| a.title).join("\t")),
    NewsResponse::Error {
      code: _,
      message: _,
    } => Err(AppError::ApiError),
  }
}
