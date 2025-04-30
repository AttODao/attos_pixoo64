use std::borrow::Cow;

use base64::{engine::general_purpose, Engine as _};
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct SendAnimation<'a> {
  #[serde(serialize_with = "command_serializer")]
  pub command: (),
  pub pic_num: u8,
  pub pic_width: u8,
  pub pic_offset: u8,
  pub pic_id: u32,
  pub pic_speed: u16,
  #[serde(serialize_with = "pic_data_serializer")]
  pub pic_data: Cow<'a, [u8]>,
}

fn command_serializer<S: Serializer>(_: &(), serializer: S) -> Result<S::Ok, S::Error> {
  serializer.serialize_str("Draw/SendHttpGif")
}

fn pic_data_serializer<S: Serializer>(
  pic_data: &Cow<[u8]>,
  serializer: S,
) -> Result<S::Ok, S::Error> {
  let base64_data = general_purpose::STANDARD.encode(pic_data);
  serializer.serialize_str(&base64_data)
}
