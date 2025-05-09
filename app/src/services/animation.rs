use std::fs::File;

use async_trait::async_trait;
use chrono::*;
use scheduler::Scheduled;

use crate::{api::pixoo_api::send_animation, config::CONFIG, errors::AppError};

pub struct Animation<T: Fn(AppError) + Send + Sync> {
  on_error: T,
}

impl<T: Fn(AppError) + Send + Sync> Animation<T> {
  pub fn new(on_error: T) -> Self {
    Animation { on_error }
  }
}

#[async_trait]
impl<T: Fn(AppError) + Send + Sync> Scheduled for Animation<T> {
  async fn process(&self, interval: TimeDelta) {
    let now = Local::now().naive_local().time();
    for &reflesh_time in &CONFIG.animation_reflesh_times {
      let to = reflesh_time - now
        + if reflesh_time < now {
          TimeDelta::days(1)
        } else {
          TimeDelta::zero()
        };
      if to < interval {
        tokio::time::sleep(to.to_std().unwrap()).await;
        if let Err(e) =
          send_animation(gif::Decoder::new(File::open(CONFIG.gif_path.clone()).unwrap()).unwrap())
            .await
        {
          (self.on_error)(e);
        }
      }
    }
  }
}
