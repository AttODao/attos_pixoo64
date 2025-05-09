pub const HOYOLAB_LANG: &str = "ja-jp";

pub const NEWS_API_URL: &str = "https://newsapi.org/v2/top-headlines";
pub const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub const HOUR_ID: u8 = 0;
pub const MINUTE_ID: u8 = 1;
pub const MONTH_ID: u8 = 2;
pub const DAY_ID: u8 = 3;
pub const WEEKDAY_ID: u8 = 4;
pub const RESIN_ID: u8 = 5;
pub const STAMINA_ID: u8 = 6;
pub const BATTERY_ID: u8 = 7;
pub const NEWS_ID: u8 = 8;

pub const WEEKDAY_TEXT: [&str; 7] = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"];

pub const COLOR01: &str = "#48AC9B";
pub const COLOR02: &str = "#97B93C";
pub const COLOR03: &str = "#A33CD0";

pub const SCROLL_SPEED: u32 = 100;

pub const HOUR_X: u8 = 3;
pub const HOUR_Y: u8 = 37;
pub const HOUR_W: u8 = 17;
pub const HOUR_H: u8 = 12;

pub const MINUTE_X: u8 = 26;
pub const MINUTE_Y: u8 = 37;
pub const MINUTE_W: u8 = 17;
pub const MINUTE_H: u8 = 12;

pub const MONTH_X: u8 = 45;
pub const MONTH_Y: u8 = 37;
pub const MONTH_W: u8 = 7;
pub const MONTH_H: u8 = 5;

pub const DAY_X: u8 = 56;
pub const DAY_Y: u8 = 39;
pub const DAY_W: u8 = 7;
pub const DAY_H: u8 = 5;

pub const WEEKDAY_X: u8 = 46;
pub const WEEKDAY_Y: u8 = 45;
pub const WEEKDAY_W: u8 = 17;
pub const WEEKDAY_H: u8 = 5;

pub const RESIN_X: u8 = 50;
pub const RESIN_Y: u8 = 3;
pub const RESIN_W: u8 = 11;
pub const RESIN_H: u8 = 5;

pub const STAMINA_X: u8 = 50;
pub const STAMINA_Y: u8 = 15;
pub const STAMINA_W: u8 = 11;
pub const STAMINA_H: u8 = 5;

pub const BATTERY_X: u8 = 50;
pub const BATTERY_Y: u8 = 19;
pub const BATTERY_W: u8 = 11;
pub const BATTERY_H: u8 = 5;

pub const NEWS_X: u8 = 2;
pub const NEWS_Y: u8 = 51;
pub const NEWS_W: u8 = 60;
pub const NEWS_H: u8 = 13;

pub const HOUR_FONT: u16 = 376;
pub const MINUTE_FONT: u16 = 376;
pub const MONTH_FONT: u16 = 82;
pub const DAY_FONT: u16 = 82;
pub const WEEKDAY_FONT: u16 = 18;
pub const RESIN_FONT: u16 = 82;
pub const STAMINA_FONT: u16 = 82;
pub const BATTERY_FONT: u16 = 82;
pub const NEWS_FONT: u16 = 52;
