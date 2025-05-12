use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error("api error")]
  ApiError,
  #[error("file open error")]
  FileOpenError(#[from] std::io::Error),
  #[error("gif decoding error({0})")]
  GifDecodingError(#[from] gif::DecodingError),
  #[error("gif format error")]
  GifFormatError,
  #[error("http request error({0})")]
  HttpRequestError(#[from] reqwest::Error),
  #[error("hoyo api error({0})")]
  HoyoApiError(#[from] hoyo_api::errors::NetworkError),
  #[error("parse json error")]
  ParseJsonError,
}
