use std::{collections::HashMap, io::Read};

use gif::Decoder;
use reqwest::Client;
use tokio::task::JoinSet;

use crate::{
  config::CONFIG,
  errors::AppError,
  models::{
    send_animation::SendAnimation,
    send_display_list::{SendDisplayList, SendDisplayListItem},
  },
};

pub async fn send_animation<R: Read>(decoder: Decoder<R>) -> Result<(), AppError> {
  if decoder.width() != 64 || decoder.height() != 64 {
    return Err(AppError::GifFormatError);
  }
  let client = Client::new();
  client
    .post(format!("http://{}/post", CONFIG.pixoo_ip))
    .json(
      &[("Command", "Draw/ResetHttpGifId")]
        .into_iter()
        .collect::<HashMap<_, _>>(),
    )
    .send()
    .await?;
  let mut join_set = JoinSet::new();
  let frames = decoder.into_iter().collect::<Result<Vec<_>, _>>()?;
  let frame_num = frames.len();
  for (i, frame) in frames.into_iter().enumerate() {
    let client = Client::new();
    let data = SendAnimation {
      command: (),
      pic_num: frame_num as u8,
      pic_width: 64,
      pic_offset: i as u8,
      pic_id: 1,
      pic_speed: frame.delay as u16 * 10,
      pic_data: frame
        .buffer
        .chunks(4)
        .flat_map(|chunk| {
          if chunk.len() >= 4 {
            // Take first 3 elements, skip the 4th
            chunk[0..3].to_vec()
          } else {
            // If we have less than 4 elements, take what we have
            chunk.to_vec()
          }
        })
        .collect(),
    };
    join_set.spawn(async move {
      client
        .post(format!("http://{}/post", CONFIG.pixoo_ip))
        .json(&data)
        .send()
        .await
    });
  }
  while let Some(res) = join_set.join_next().await {
    match res.unwrap() {
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

pub async fn send_text(items: Vec<SendDisplayListItem>) -> Result<(), AppError> {
  let client = Client::new();
  let data = SendDisplayList {
    command: (),
    item_list: items,
  };
  let res = client
    .post(format!("http://{}/post", CONFIG.pixoo_ip))
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

pub async fn clear_text() -> Result<(), AppError> {
  let client = Client::new();
  let res = client
    .post(format!("http://{}/post", CONFIG.pixoo_ip))
    .json(
      &[("Command", "Draw/ClearHttpText")]
        .into_iter()
        .collect::<HashMap<_, _>>(),
    )
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
