use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("api error")]
  ApiError,
  #[error("gif decoding error({0})")]
  GifDecodingError(#[from] gif::DecodingError),
  #[error("gif format error")]
  GifFormatError,
  #[error("http request error({0})")]
  HttpRequestError(#[from] reqwest::Error),
  #[error("parse json error")]
  ParseJsonError,
}
