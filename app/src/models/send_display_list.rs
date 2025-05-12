use serde::{Serialize, Serializer};
use serde_repr::Serialize_repr;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendDisplayList {
  #[serde(serialize_with = "command_serializer")]
  pub command: (),
  pub item_list: Vec<SendDisplayListItem>,
}
#[derive(Debug, Serialize)]
pub struct SendDisplayListItem {
  #[serde(rename = "TextId")]
  pub text_id: u8,
  pub r#type: DisplayType,
  pub x: u8,
  pub y: u8,
  pub dir: ScrollDir,
  pub font: u16,
  #[serde(rename = "TextWidth")]
  pub text_width: u8,
  #[serde(rename = "Textheight")]
  pub text_height: u8,
  #[serde(rename = "TextString")]
  pub text_string: String,
  pub speed: u32,
  pub color: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub update_time: Option<u32>,
  pub align: Align,
}
#[derive(Debug, Serialize_repr)]
#[repr(u8)]
pub enum DisplayType {
  Second = 1,
  Min = 2,
  Hour = 3,
  TimeAmPm = 4,
  HourMin = 5,
  HourMinSec = 6,
  Year = 7,
  Day = 8,
  Mon = 9,
  MonYear = 10,
  EngMonthDotDay = 11,
  DateWeekYear = 12,
  EngWeek = 13,
  EngWeekThree = 14,
  EngWeekAll = 15,
  EngMon = 16,
  TempDigit = 17,
  TodayMaxTemp = 18,
  TodayMinTemp = 19,
  WeatherWord = 20,
  NoiseDigit = 21,
  TextMessage = 22,
  NetTextMessage = 23,
}
#[derive(Debug, Serialize_repr)]
#[repr(u8)]
pub enum ScrollDir {
  Left = 0,
  Right = 1,
}
#[derive(Debug, Serialize_repr)]
#[repr(u8)]
pub enum Align {
  Left = 1,
  Middle = 2,
  Right = 3,
}

fn command_serializer<S: Serializer>(_: &(), serializer: S) -> Result<S::Ok, S::Error> {
  serializer.serialize_str("Draw/SendHttpItemList")
}
