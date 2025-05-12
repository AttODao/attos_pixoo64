use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
  pub list: Vec<WeatherListItem>,
}
#[derive(Debug, Deserialize)]
pub struct WeatherListItem {
  pub temp: WeatherTemp,
  pub weather: Vec<WeatherWeather>,
}
#[derive(Debug, Deserialize)]
pub struct WeatherTemp {
  pub min: f64,
  pub max: f64,
}
#[derive(Debug, Deserialize)]
pub struct WeatherWeather {
  pub icon: String,
}
