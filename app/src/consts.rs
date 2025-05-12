use chrono::NaiveDateTime;

pub const MY_EPOCH: NaiveDateTime = NaiveDateTime::new(
  chrono::NaiveDate::from_ymd_opt(2020, 1, 1).unwrap(),
  chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
);

pub const HOYOLAB_LANG: &str = "ja-jp";

pub const OPEN_WEATHER_API_URL: &str = "https://api.openweathermap.org/data/2.5/forecast/daily";
pub const WEATHER_CNT: &str = "1";
pub const WEATHER_UNITS: &str = "metric";
pub const WEATHER_LANG: &str = "en";
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub static WEATHERS: phf::Map<&str, &str> = phf::phf_map!(
  "01d" => "Clear",
  "01n" => "Clear",
  "02d" => "Sunny",
  "02n" => "Sunny",
  "03d" => "Cloud",
  "03n" => "Cloud",
  "04d" => "Cloud",
  "04n" => "Cloud",
  "09d" => "Rain",
  "09n" => "Rain",
  "10d" => "Rain",
  "10n" => "Rain",
  "11d" => "Thund",
  "11n" => "Thund",
  "13d" => "Snow",
  "13n" => "Snow",
  "50d" => "Fog",
  "50n" => "Fog"
);

pub const HOUR_ID: u8 = 1;
pub const MINUTE_ID: u8 = 2;
pub const MONTH_ID: u8 = 3;
pub const DAY_ID: u8 = 4;
pub const WEEKDAY_ID: u8 = 5;
pub const RESIN_ID: u8 = 6;
pub const STAMINA_ID: u8 = 7;
pub const BATTERY_ID: u8 = 8;
pub const WEATHER_ID: u8 = 9;
pub const MIN_TEMP_ID: u8 = 10;
pub const MAX_TEMP_ID: u8 = 11;

pub const WEEKDAY_TEXT: [&str; 7] = ["MO", "TU", "WE", "TH", "FR", "SA", "SU"];

pub const COLOR01: &str = "#48AC9B";
pub const COLOR02: &str = "#4D0082";
pub const COLOR03: &str = "#DC143C";
pub const COLOR04: &str = "#FF6A00";

pub const SCROLL_SPEED: u32 = 100;

pub const HOUR_X: u8 = 7;
pub const HOUR_Y: u8 = 43;
pub const HOUR_W: u8 = 15;
pub const HOUR_H: u8 = 13;

pub const MINUTE_X: u8 = 26;
pub const MINUTE_Y: u8 = 43;
pub const MINUTE_W: u8 = 15;
pub const MINUTE_H: u8 = 13;

pub const MONTH_X: u8 = 50;
pub const MONTH_Y: u8 = 43;
pub const MONTH_W: u8 = 11;
pub const MONTH_H: u8 = 7;

pub const DAY_X: u8 = 53;
pub const DAY_Y: u8 = 51;
pub const DAY_W: u8 = 11;
pub const DAY_H: u8 = 7;

pub const WEEKDAY_X: u8 = 50;
pub const WEEKDAY_Y: u8 = 59;
pub const WEEKDAY_W: u8 = 14;
pub const WEEKDAY_H: u8 = 5;

pub const RESIN_X: u8 = 51;
pub const RESIN_Y: u8 = 4;
pub const RESIN_W: u8 = 11;
pub const RESIN_H: u8 = 5;

pub const STAMINA_X: u8 = 51;
pub const STAMINA_Y: u8 = 17;
pub const STAMINA_W: u8 = 11;
pub const STAMINA_H: u8 = 5;

pub const BATTERY_X: u8 = 51;
pub const BATTERY_Y: u8 = 30;
pub const BATTERY_W: u8 = 11;
pub const BATTERY_H: u8 = 5;

pub const WEATHER_X: u8 = 3;
pub const WEATHER_Y: u8 = 59;
pub const WEATHER_W: u8 = 22;
pub const WEATHER_H: u8 = 5;

pub const MIN_TEMP_X: u8 = 27;
pub const MIN_TEMP_Y: u8 = 59;
pub const MIN_TEMP_W: u8 = 7;
pub const MIN_TEMP_H: u8 = 5;

pub const MAX_TEMP_X: u8 = 38;
pub const MAX_TEMP_Y: u8 = 59;
pub const MAX_TEMP_W: u8 = 7;
pub const MAX_TEMP_H: u8 = 5;

pub const HOUR_FONT: u16 = 96;
pub const MINUTE_FONT: u16 = 96;
pub const MONTH_FONT: u16 = 80;
pub const DAY_FONT: u16 = 80;
pub const WEEKDAY_FONT: u16 = 114;
pub const RESIN_FONT: u16 = 82;
pub const STAMINA_FONT: u16 = 82;
pub const BATTERY_FONT: u16 = 82;
pub const WEATHER_FONT: u16 = 198;
pub const MIN_TEMP_FONT: u16 = 82;
pub const MAX_TEMP_FONT: u16 = 82;
