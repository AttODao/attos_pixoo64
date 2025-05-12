use async_trait::async_trait;
use chrono::*;
use hoyo_api::{
  game_record::get_daily_note,
  models::{
    genshin::game_record::GenshinDailyNote, login_cookie::LoginCookie,
    starrail::game_record::StarrailDailyNote, zzz::game_record::ZzzDailyNote,
  },
};
use log::info;
use scheduler::Scheduled;

use crate::{
  api::{pixoo_api::send_text, weather_api::get_weather},
  config::CONFIG,
  consts::*,
  errors::AppError,
  models::send_display_list::{Align, DisplayType, ScrollDir, SendDisplayListItem},
  types::ErrorHandler,
};

pub struct Text {
  on_error: ErrorHandler,
}

impl Text {
  pub fn new(on_error: ErrorHandler) -> Self {
    Text { on_error }
  }

  pub async fn clock(now: NaiveDateTime) -> Result<Vec<SendDisplayListItem>, AppError> {
    info!("Task: clock({})", now);
    let mut items = vec![];

    items.push(SendDisplayListItem {
      text_id: HOUR_ID,
      r#type: DisplayType::TextMessage,
      x: HOUR_X,
      y: HOUR_Y,
      text_width: HOUR_W,
      text_height: HOUR_H,
      dir: ScrollDir::Left,
      font: HOUR_FONT,
      text_string: format!("{:02}", now.hour()),
      speed: 0,
      color: COLOR01.to_string(),
      update_time: None,
      align: Align::Middle,
    });
    items.push(SendDisplayListItem {
      text_id: MINUTE_ID,
      r#type: DisplayType::TextMessage,
      x: MINUTE_X,
      y: MINUTE_Y,
      text_width: MINUTE_W,
      text_height: MINUTE_H,
      dir: ScrollDir::Left,
      font: MINUTE_FONT,
      text_string: format!("{:02}", now.minute()),
      speed: 0,
      color: COLOR01.to_string(),
      update_time: None,
      align: Align::Middle,
    });
    items.push(SendDisplayListItem {
      text_id: MONTH_ID,
      r#type: DisplayType::TextMessage,
      x: MONTH_X,
      y: MONTH_Y,
      text_width: MONTH_W,
      text_height: MONTH_H,
      dir: ScrollDir::Left,
      font: MONTH_FONT,
      text_string: format!("{:02}", now.month()),
      speed: 0,
      color: COLOR02.to_string(),
      update_time: None,
      align: Align::Middle,
    });
    items.push(SendDisplayListItem {
      text_id: DAY_ID,
      r#type: DisplayType::TextMessage,
      x: DAY_X,
      y: DAY_Y,
      text_width: DAY_W,
      text_height: DAY_H,
      dir: ScrollDir::Left,
      font: DAY_FONT,
      text_string: format!("{:02}", now.day()),
      speed: 0,
      color: COLOR02.to_string(),
      update_time: None,
      align: Align::Middle,
    });
    items.push(SendDisplayListItem {
      text_id: WEEKDAY_ID,
      r#type: DisplayType::TextMessage,
      x: WEEKDAY_X,
      y: WEEKDAY_Y,
      text_width: WEEKDAY_W,
      text_height: WEEKDAY_H,
      dir: ScrollDir::Left,
      font: WEEKDAY_FONT,
      text_string: WEEKDAY_TEXT[now.weekday().num_days_from_monday() as usize].to_string(),
      speed: 0,
      color: COLOR02.to_string(),
      update_time: None,
      align: Align::Middle,
    });

    Ok(items)
  }

  pub async fn hoyolab() -> Result<Vec<SendDisplayListItem>, AppError> {
    info!("Task: hoyolab");
    let mut items = vec![];

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
    let (genshin, starrail, zzz) = tokio::try_join!(future_genshin, future_starrail, future_zzz)?;
    let (genshin, starrail, zzz) = (
      genshin.data.ok_or(AppError::ParseJsonError)?,
      starrail.data.ok_or(AppError::ParseJsonError)?,
      zzz.data.ok_or(AppError::ParseJsonError)?,
    );
    items.push(SendDisplayListItem {
      text_id: RESIN_ID,
      r#type: DisplayType::TextMessage,
      x: RESIN_X,
      y: RESIN_Y,
      text_width: RESIN_W,
      text_height: RESIN_H,
      dir: ScrollDir::Left,
      font: RESIN_FONT,
      text_string: format!("{:03}", genshin.current_resin),
      speed: 0,
      color: COLOR03.to_string(),
      update_time: None,
      align: Align::Middle,
    });
    items.push(SendDisplayListItem {
      text_id: STAMINA_ID,
      r#type: DisplayType::TextMessage,
      x: STAMINA_X,
      y: STAMINA_Y,
      text_width: STAMINA_W,
      text_height: STAMINA_H,
      dir: ScrollDir::Left,
      font: STAMINA_FONT,
      text_string: format!("{:03}", starrail.current_stamina),
      speed: 0,
      color: COLOR03.to_string(),
      update_time: None,
      align: Align::Middle,
    });
    items.push(SendDisplayListItem {
      text_id: BATTERY_ID,
      r#type: DisplayType::TextMessage,
      x: BATTERY_X,
      y: BATTERY_Y,
      text_width: BATTERY_W,
      text_height: BATTERY_H,
      dir: ScrollDir::Left,
      font: BATTERY_FONT,
      text_string: format!("{:03}", zzz.energy.progress.current),
      speed: 0,
      color: COLOR03.to_string(),
      update_time: None,
      align: Align::Middle,
    });

    Ok(items)
  }

  pub async fn weather() -> Result<Vec<SendDisplayListItem>, AppError> {
    info!("Task: weather");
    let mut items = vec![];

    let weather = get_weather().await?;
    items.push(SendDisplayListItem {
      text_id: WEATHER_ID,
      r#type: DisplayType::TextMessage,
      x: WEATHER_X,
      y: WEATHER_Y,
      dir: ScrollDir::Left,
      font: WEATHER_FONT,
      text_width: WEATHER_W,
      text_height: WEATHER_H,
      text_string: WEATHERS
        .get(
          weather
            .list
            .first()
            .unwrap()
            .weather
            .first()
            .unwrap()
            .icon
            .as_str(),
        )
        .unwrap()
        .to_string()
        .to_uppercase(),
      speed: 0,
      color: COLOR04.to_string(),
      update_time: None,
      align: Align::Middle,
    });
    items.push(SendDisplayListItem {
      text_id: MIN_TEMP_ID,
      r#type: DisplayType::TextMessage,
      x: MIN_TEMP_X,
      y: MIN_TEMP_Y,
      dir: ScrollDir::Left,
      font: MIN_TEMP_FONT,
      text_width: MIN_TEMP_W,
      text_height: MIN_TEMP_H,
      text_string: format!("{:02}", weather.list.first().unwrap().temp.min as i32),
      speed: 0,
      color: COLOR04.to_string(),
      update_time: None,
      align: Align::Middle,
    });
    items.push(SendDisplayListItem {
      text_id: MAX_TEMP_ID,
      r#type: DisplayType::TextMessage,
      x: MAX_TEMP_X,
      y: MAX_TEMP_Y,
      dir: ScrollDir::Left,
      font: MAX_TEMP_FONT,
      text_width: MAX_TEMP_W,
      text_height: MAX_TEMP_H,
      text_string: format!("{:02}", weather.list.first().unwrap().temp.max as i32),
      speed: 0,
      color: COLOR04.to_string(),
      update_time: None,
      align: Align::Middle,
    });

    Ok(items)
  }

  pub async fn run(now: NaiveDateTime) -> Result<(), AppError> {
    info!("Running: Text");
    let mut items = vec![];

    items.extend(Self::clock(now).await?);

    if (now.time() - NaiveTime::from_hms_opt(0, 0, 0).unwrap()).num_minutes()
      % CONFIG.hoyolab_reflesh_interval_mins as i64
      == 0
    {
      items.extend(Self::hoyolab().await?);
    }

    if CONFIG.weather_reflesh_times.contains(&now.time()) {
      items.extend(Self::weather().await?);
    }
    send_text(items).await?;

    Ok(())
  }
}

#[async_trait]
impl Scheduled for Text {
  async fn process(&self, interval: TimeDelta) {
    let now = Local::now().naive_local();
    let to = MY_EPOCH
      + interval * (((now - MY_EPOCH).num_seconds() + 1) / interval.num_seconds() + 1) as i32;
    let wait = to - now;
    tokio::time::sleep(wait.to_std().unwrap()).await;
    if let Err(e) = Self::run(to).await {
      (self.on_error)(e);
    }
  }
}
