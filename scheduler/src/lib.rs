use std::sync::Arc;

use async_trait::async_trait;
use chrono::TimeDelta;
use futures::FutureExt;
use tokio_util::sync::CancellationToken;

#[async_trait]
pub trait Scheduled {
  async fn process(&self, interval: TimeDelta);
}

pub struct Scheduler {
  scheduleds: Vec<Arc<Box<dyn Scheduled + Send + Sync>>>,
  cancel_token: Arc<CancellationToken>,
}

impl Scheduler {
  pub fn from_scheduleds(scheduleds: Vec<Box<dyn Scheduled + Send + Sync>>) -> Self {
    let scheduleds = scheduleds.into_iter().map(Arc::new).collect();
    let cancel_token = Arc::new(CancellationToken::new());

    Scheduler {
      scheduleds,
      cancel_token,
    }
  }

  pub fn run(&self, interval: TimeDelta) {
    let scheduleds = self.scheduleds.clone();
    let cancel_token = self.cancel_token.clone();
    tokio::spawn(async move {
      loop {
        for s in scheduleds.clone().into_iter() {
          tokio::spawn(async move {
            s.process(interval).await;
          });
        }

        futures::select! {
          _ = tokio::time::sleep(interval.to_std().expect("interval is larger than the maximum value supported for std::time::Duration")).fuse() =>(),
          _ = cancel_token.cancelled().fuse() => return,
        };
      }
    });
  }
}

impl Drop for Scheduler {
  fn drop(&mut self) {
    self.cancel_token.cancel();
  }
}

#[cfg(test)]
mod tests {}
