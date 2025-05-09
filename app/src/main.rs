use attos_pixoo64::{
  errors::AppError,
  services::{animation::Animation, clock::Clock, hoyolab::Hoyolab, news::News},
};
use chrono::TimeDelta;
use log::{error, info, warn};
use scheduler::Scheduler;

fn on_error(e: AppError) {
  error!("Error: {}", e);
}

#[tokio::main]
async fn main() {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("warn"));
  info!("Starting AttO's Pixoo64...");

  let scheduler = Scheduler::from_scheduleds(vec![
    Box::new(Animation::new(on_error)),
    Box::new(Clock::new(on_error)),
    Box::new(Hoyolab::new(on_error)),
    Box::new(News::new(on_error)),
  ]);
  info!("Starting scheduler...");
  scheduler.run(TimeDelta::minutes(1));

  #[cfg(unix)]
  {
    use tokio::signal::unix as signal;

    let [mut s1, mut s2, mut s3] = [
      signal::signal(signal::SignalKind::hangup()).unwrap(),
      signal::signal(signal::SignalKind::interrupt()).unwrap(),
      signal::signal(signal::SignalKind::terminate()).unwrap(),
    ];

    tokio::select!(
        v = s1.recv() => v.unwrap(),
        v = s2.recv() => v.unwrap(),
        v = s3.recv() => v.unwrap(),
    );
  }
  #[cfg(windows)]
  {
    let (mut s1, mut s2) = (
      tokio::signal::windows::ctrl_c().unwrap(),
      tokio::signal::windows::ctrl_break().unwrap(),
    );

    tokio::select!(
        v = s1.recv() => v.unwrap(),
        v = s2.recv() => v.unwrap(),
    );
  }
  warn!("Recieved control C and shutting down.");
}
