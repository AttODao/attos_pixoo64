use std::fs::File;

use async_trait::async_trait;
use chrono::*;
use log::info;
use scheduler::Scheduled;

use crate::{
  api::pixoo_api::send_animation, config::CONFIG, errors::AppError, types::ErrorHandler,
};

pub struct Animation {
  on_error: ErrorHandler,
}

impl Animation {
  pub fn new(on_error: ErrorHandler) -> Self {
    Animation { on_error }
  }

  pub async fn run() -> Result<(), AppError> {
    info!("Running: Animation");
    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);
    let file = File::open(&CONFIG.gif_path).unwrap();
    let decoder = decoder.read_info(file).unwrap();
    send_animation(decoder).await
  }
}

#[async_trait]
impl Scheduled for Animation {
  async fn process(&self, interval: TimeDelta) {
    let now = Local::now().naive_local().time();
    for &reflesh_time in &CONFIG.animation_reflesh_times {
      let wait = reflesh_time - now
        + if reflesh_time < now {
          TimeDelta::days(1)
        } else {
          TimeDelta::zero()
        };
      if wait < interval {
        tokio::time::sleep(wait.to_std().unwrap()).await;
        if let Err(e) = Self::run().await {
          (self.on_error)(e);
        }
      }
    }
  }
}
