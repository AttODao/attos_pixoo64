use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SendAnimation {
  pub command: (),
}
