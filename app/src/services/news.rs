use crate::{
  api::{news_api::get_news, pixoo_api::send_text},
  config::*,
  consts::*,
  errors::AppError,
};
use async_trait::async_trait;
use chrono::*;
use scheduler::Scheduled;

pub struct News<T: Fn(AppError) + Send + Sync> {
  on_error: T,
}

impl<T: Fn(AppError) + Send + Sync> News<T> {
  pub fn new(on_error: T) -> Self {
    News { on_error }
  }
}

#[async_trait]
impl<T: Fn(AppError) + Send + Sync> Scheduled for News<T> {
  async fn process(&self, interval: TimeDelta) {
    let now = Local::now().naive_local().time();
    for &reflesh_time in &CONFIG.news_reflesh_times {
      let to = reflesh_time - now
        + if reflesh_time < now {
          TimeDelta::days(1)
        } else {
          TimeDelta::zero()
        };
      if to < interval {
        tokio::time::sleep(to.to_std().unwrap()).await;
        let news = get_news().await;
        match news {
          Ok(news) => {
            match send_text(
              news,
              NEWS_X,
              NEWS_Y,
              NEWS_W,
              NEWS_H,
              NEWS_FONT,
              COLOR03.to_string(),
              NEWS_ID,
            )
            .await
            {
              Ok(_) => (),
              Err(e) => {
                (self.on_error)(e);
              }
            }
          }
          Err(e) => {
            (self.on_error)(e);
          }
        }
      }
    }
  }
}
