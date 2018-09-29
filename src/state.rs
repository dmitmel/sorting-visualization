use std::sync::{Arc, Mutex, MutexGuard};

use array::ArrayAccess;
use graphics::types::Color;

pub type SharedState = Arc<State>;

#[derive(Debug)]
pub struct State {
  pub animation: Mutex<AnimationState>,
}

impl State {
  pub fn animation(&self) -> MutexGuard<'_, AnimationState> {
    self.animation.lock().unwrap()
  }
}

#[derive(Debug)]
pub struct AnimationState {
  pub time: f64,
  pub array: Vec<u32>,
  pub colors: Vec<Color>,
  pub array_accesses: Vec<ArrayAccess>,
}
