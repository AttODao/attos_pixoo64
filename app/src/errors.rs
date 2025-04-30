use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("api error")]
  ApiError,
  #[error("http request error({0})")]
  HttpRequestError(#[from] reqwest::Error),
  #[error("parse json error")]
  ParseJsonError,
}
