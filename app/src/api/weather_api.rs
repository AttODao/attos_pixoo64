use reqwest::ClientBuilder;

use crate::{
  config::CONFIG, consts::*, errors::AppError, models::weather_response::WeatherResponse,
};

pub async fn get_weather() -> Result<WeatherResponse, AppError> {
  let client = ClientBuilder::new().user_agent(USER_AGENT).build()?;
  let res = client
    .get(OPEN_WEATHER_API_URL)
    .query(&[
      ("lat", CONFIG.weather_lat.as_str()),
      ("lon", CONFIG.weather_lon.as_str()),
      ("appid", CONFIG.open_weather_api_key.as_str()),
      ("cnt", WEATHER_CNT),
      ("units", WEATHER_UNITS),
      ("lang", WEATHER_LANG),
    ])
    .send()
    .await?;
  res
    .json::<WeatherResponse>()
    .await
    .map_err(|_| AppError::ParseJsonError)
}
