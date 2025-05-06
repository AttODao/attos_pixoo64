use std::{collections::HashMap, io::Read};

use gif::Decoder;
use reqwest::Client;
use tokio::task::JoinSet;

use crate::{
  config::CONFIG,
  consts::*,
  errors::AppError,
  models::{
    send_animation::SendAnimation,
    send_display_list::{Align, DisplayType, ScrollDir, SendDisplayList, SendDisplayListItem},
  },
};

pub async fn send_animation<R: Read>(
  decoder: Decoder<R>,
  animation_id: u32,
) -> Result<(), AppError> {
  if decoder.width() != 64 || decoder.height() != 64 || decoder.buffer_size() != 64 * 64 * 3 {
    return Err(AppError::GifFormatError);
  }
  let client = Client::builder().user_agent(USER_AGENT).build()?;
  let mut join_set = JoinSet::new();
  let frames = decoder.into_iter().collect::<Result<Vec<_>, _>>()?;
  let frame_num = frames.len();
  for (i, frame) in frames.into_iter().enumerate() {
    let data = SendAnimation {
      command: (),
      pic_num: frame_num as u8,
      pic_width: 64,
      pic_offset: i as u8,
      pic_id: animation_id,
      pic_speed: frame.delay as u16,
      pic_data: frame.buffer,
    };
    join_set.spawn(
      client
        .post(format!("http://{}", CONFIG.pixoo_ip))
        .json(&data)
        .send(),
    );
  }
  for res in join_set.join_all().await {
    match res {
      Ok(res) => {
        let json = res
          .json::<HashMap<String, u32>>()
          .await
          .map_err(|_| AppError::ParseJsonError)?;
        if json.get("error_code") != Some(&0) {
          return Err(AppError::ApiError);
        }
      }
      Err(e) => return Err(AppError::HttpRequestError(e)),
    }
  }
  Ok(())
}

pub async fn send_text(
  text: String,
  x: u8,
  y: u8,
  w: u8,
  h: u8,
  font: u16,
  color: String,
  text_id: u8,
) -> Result<(), AppError> {
  let client = Client::builder().user_agent(USER_AGENT).build()?;
  let data = SendDisplayList {
    command: (),
    item_list: vec![SendDisplayListItem {
      text_id,
      r#type: DisplayType::TextMessage,
      x,
      y,
      dir: ScrollDir::Left,
      font,
      text_width: w,
      text_height: h,
      text_string: text,
      speed: SCROLL_SPEED,
      color,
      update_time: None,
      align: Align::Left,
    }],
  };
  let res = client
    .post(format!("http://{}", CONFIG.pixoo_ip))
    .json(&data)
    .send()
    .await;
  match res {
    Ok(res) => {
      let json = res
        .json::<HashMap<String, u32>>()
        .await
        .map_err(|_| AppError::ParseJsonError)?;
      if json.get("error_code") != Some(&0) {
        return Err(AppError::ApiError);
      }
    }
    Err(e) => return Err(AppError::HttpRequestError(e)),
  }
  Ok(())
}
