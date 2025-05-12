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
  pub genshin_id: String,
  pub starrail_id: String,
  pub zzz_id: String,
  pub open_weather_api_key: String,
  pub weather_lat: String,
  pub weather_lon: String,
  pub animation_reflesh_times: Vec<NaiveTime>,
  pub hoyolab_reflesh_interval_mins: u32,
  pub weather_reflesh_times: Vec<NaiveTime>,
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
