use async_trait::async_trait;
use chrono::*;
use hoyo_api::{
  game_record::get_daily_note,
  models::{
    genshin::game_record::GenshinDailyNote, login_cookie::LoginCookie,
    starrail::game_record::StarrailDailyNote, zzz::game_record::ZzzDailyNote,
  },
};
use scheduler::Scheduled;

use crate::{api::pixoo_api::send_text, config::CONFIG, consts::*, errors::AppError};

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
    let time_to_midnight = TimeDelta::days(1) - (now - NaiveTime::from_hms_opt(0, 0, 0).unwrap());
    let to = TimeDelta::milliseconds(
      time_to_midnight.num_milliseconds() % CONFIG.hoyolab_reflesh_interval.num_milliseconds(),
    );
    if to < interval {
      tokio::time::sleep(to.to_std().unwrap()).await;
      let cookie = LoginCookie::new(CONFIG.hoyolab_ltoken.clone(), CONFIG.hoyolab_ltuid.clone());
      let future_genshin = get_daily_note::<GenshinDailyNote>(
        CONFIG.genshin_id.clone(),
        cookie.clone(),
        HOYOLAB_LANG.to_string(),
      );
      let future_starrail = get_daily_note::<StarrailDailyNote>(
        CONFIG.starrail_id.clone(),
        cookie.clone(),
        HOYOLAB_LANG.to_string(),
      );
      let future_zzz = get_daily_note::<ZzzDailyNote>(
        CONFIG.zzz_id.clone(),
        cookie.clone(),
        HOYOLAB_LANG.to_string(),
      );
      match tokio::try_join!(future_genshin, future_starrail, future_zzz) {
        Ok((genshin, starail, zzz)) => {
          if let (Some(genshin), Some(starrail), Some(zzz)) = (genshin.data, starail.data, zzz.data)
          {
            let future_resin = send_text(
              genshin.current_resin.to_string(),
              RESIN_X,
              RESIN_Y,
              RESIN_W,
              RESIN_H,
              RESIN_FONT,
              COLOR02.to_string(),
              RESIN_ID,
            );
            let future_stamina = send_text(
              starrail.current_stamina.to_string(),
              STAMINA_X,
              STAMINA_Y,
              STAMINA_W,
              STAMINA_H,
              STAMINA_FONT,
              COLOR02.to_string(),
              STAMINA_ID,
            );
            let future_battery = send_text(
              zzz.vitality.current.to_string(),
              BATTERY_X,
              BATTERY_Y,
              BATTERY_W,
              BATTERY_H,
              BATTERY_FONT,
              COLOR02.to_string(),
              BATTERY_ID,
            );
            if let Err(e) = tokio::try_join!(future_resin, future_stamina, future_battery) {
              (self.on_error)(e);
            }
          } else {
            (self.on_error)(AppError::ParseJsonError);
          }
        }
        Err(e) => {
          (self.on_error)(AppError::HoyoApiError(e));
        }
      }
    }
  }
}
