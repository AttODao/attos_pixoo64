use async_trait::async_trait;
use chrono::*;
use scheduler::Scheduled;

use crate::{api::pixoo_api::send_text, consts::*, errors::AppError};

pub struct Clock<T: Fn(AppError) + Send + Sync> {
  on_error: T,
}

impl<T: Fn(AppError) + Send + Sync> Clock<T> {
  pub fn new(on_error: T) -> Self {
    Clock { on_error }
  }
}

#[async_trait]
impl<T: Fn(AppError) + Send + Sync> Scheduled for Clock<T> {
  async fn process(&self, interval: TimeDelta) {
    let now = Local::now().naive_local().time();
    let time_to_midnight = TimeDelta::days(1) - (now - NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    let to = TimeDelta::milliseconds(
      time_to_midnight.num_milliseconds() % TimeDelta::minutes(1).num_milliseconds(),
    );
    if to < interval {
      tokio::time::sleep(to.to_std().unwrap()).await;
      let clock = Local::now().naive_local();
      let hour = send_text(
        clock.hour().to_string(),
        HOUR_X,
        HOUR_Y,
        HOUR_W,
        HOUR_H,
        HOUR_FONT,
        COLOR01.to_string(),
        HOUR_ID,
      );
      let minute = send_text(
        clock.minute().to_string(),
        MINUTE_X,
        MINUTE_Y,
        MINUTE_W,
        MINUTE_H,
        MINUTE_FONT,
        COLOR01.to_string(),
        MINUTE_ID,
      );
      let month = send_text(
        clock.month().to_string(),
        MONTH_X,
        MONTH_Y,
        MONTH_W,
        MONTH_H,
        MONTH_FONT,
        COLOR01.to_string(),
        MONTH_ID,
      );
      let day = send_text(
        clock.day().to_string(),
        DAY_X,
        DAY_Y,
        DAY_W,
        DAY_H,
        DAY_FONT,
        COLOR01.to_string(),
        DAY_ID,
      );
      let weekday = send_text(
        WEEKDAY_TEXT[clock.weekday().num_days_from_monday() as usize].to_string(),
        WEEKDAY_X,
        WEEKDAY_Y,
        WEEKDAY_W,
        WEEKDAY_H,
        WEEKDAY_FONT,
        COLOR01.to_string(),
        WEEKDAY_ID,
      );
      if let Err(e) = tokio::try_join!(hour, minute, month, day, weekday) {
        (self.on_error)(e);
      }
    }
  }
}
