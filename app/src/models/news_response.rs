use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(tag = "status")]
pub enum NewsResponse {
  #[serde(rename_all = "camelCase", rename = "ok")]
  Ok {
    total_results: u32,
    articles: Vec<NewsArticle>,
  },
  #[serde(rename_all = "camelCase", rename = "error")]
  Error { code: String, message: String },
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsArticle {
  pub source: NewsSource,
  pub author: Option<String>,
  pub title: String,
  pub description: Option<String>,
  pub url: String,
  pub url_to_image: Option<String>,
  pub published_at: String,
  pub content: Option<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewsSource {
  pub id: Option<String>,
  pub name: String,
}
