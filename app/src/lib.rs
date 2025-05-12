pub mod api;
pub mod models;
pub mod services;

pub mod config;
pub mod consts;
pub mod errors;
pub mod types;

#[cfg(test)]
mod tests {
  use chrono::NaiveDateTime;
  use hoyo_api::models::{json_wrapper::JsonWrapper, zzz::game_record::ZzzDailyNote};

  use crate::{
    api::{pixoo_api::clear_text, weather_api::get_weather},
    consts::HOUR_FONT,
    models::send_display_list::{
      Align, DisplayType, ScrollDir, SendDisplayList, SendDisplayListItem,
    },
    services::text::Text,
  };

  #[tokio::test]
  async fn pixoo_api_test() {
    println!("{:?}", clear_text().await);
    println!(
      "{:?}",
      Text::run(NaiveDateTime::parse_from_str("2015-09-05 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap())
        .await
    );
    // println!(
    //   "{:?}",
    //   send_text(
    //     "Hoge".to_string(),
    //     0,
    //     0,
    //     64,
    //     16,
    //     4,
    //     "#FFFFFF".to_string(),
    //     2,
    //   )
    //   .await
    // );
  }

  #[tokio::test]
  async fn pixoo_serialize_test() {
    let data = SendDisplayList {
      command: (),
      item_list: vec![SendDisplayListItem {
        text_id: 0,
        r#type: DisplayType::TextMessage,
        x: 0,
        y: 0,
        dir: ScrollDir::Left,
        font: HOUR_FONT,
        text_width: 64,
        text_height: 16,
        text_string: "TEST".to_string(),
        speed: 2,
        color: "#FFFFFF".to_string(),
        update_time: None,
        align: Align::Middle,
      }],
    };
    println!("{:?}", serde_json::to_string(&data));
  }

  #[tokio::test]
  async fn weather_api_test() {
    let res = get_weather().await;
    println!("{:?}", res);
    assert!(res.is_ok());
  }

  #[tokio::test]
  async fn parse_json_test_02() {
    let json = r#"{
  "retcode": 0,
  "message": "OK",
  "data": {
    "energy": {
      "progress": {
        "max": 240,
        "current": 190
      },
      "restore": 17955,
      "day_type": 2,
      "hour": 1,
      "minute": 32
    },
    "vitality": {
      "max": 400,
      "current": 400
    },
    "vhs_sale": {
      "sale_state": "SaleStateDoing"
    },
    "card_sign": "CardSignDone",
    "bounty_commission": {
      "num": 0,
      "total": 4,
      "refresh_time": 26785
    },
    "survey_points": null,
    "abyss_refresh": 26785,
    "coffee": null,
    "weekly_task": {
      "refresh_time": 26785,
      "cur_point": 800,
      "max_point": 1300
    },
    "member_card": {
      "is_open": false,
      "member_card_state": "MemberCardStateNo",
      "exp_time": "0"
    },
    "is_sub": false,
    "is_other_sub": false
  }
}"#;
    let json = serde_json::from_str::<JsonWrapper<ZzzDailyNote>>(&json);
    println!("{:?}", json);
  }
}
