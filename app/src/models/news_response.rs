use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase", tag = "status")]
pub enum NewsResponse {
  Ok {
    total_results: u32,
    articles: Vec<NewsArticle>,
  },
  Error {
    code: String,
    message: String,
  },
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NewsArticle {
  pub source: NewsSource,
  pub author: String,
  pub title: String,
  pub description: String,
  pub url: String,
  pub url_to_image: String,
  pub published_at: String,
  pub content: String,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NewsSource {
  pub id: String,
  pub name: String,
}
