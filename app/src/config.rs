use chrono::naive::NaiveTime;
use config::ConfigError;
use once_cell::sync::Lazy;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
  pub pixoo_ip: String,
  pub gif_path: String,
  pub hoyolab_ltoken: String,
  pub hoyolab_ltuid: String,
  pub news_api_key: String,
  pub hoyolab_reflesh_time: NaiveTime,
  pub news_reflesh_time: NaiveTime,
}
impl Config {
  pub fn load() -> Result<Self, ConfigError> {
    let cfg = config::Config::builder()
      .add_source(config::File::with_name("config"))
      .build()?;
    cfg.try_deserialize()
  }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::load().expect("Failed to load config"));
