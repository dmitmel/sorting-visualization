use graphics::color;
use graphics::types::Color;

use state::SharedState;

#[derive(Debug)]
pub struct Array(SharedState);

impl Array {
  pub fn new(state: SharedState) -> Self {
    Array(state)
  }

  pub fn wait(&self, ms: u64) {
    use std::thread;
    use std::time::Duration;

    thread::sleep(Duration::from_millis({
      let anim = self.0.animation();
      (ms as f64 / anim.speed) as u64
    }));

    let mut anim = self.0.animation();
    while anim.paused {
      anim = self.0.pause_notifier.wait(anim).unwrap();
    }
  }

  pub fn len(&self) -> usize {
    let anim = self.0.animation();
    anim.array.len()
  }

  pub fn get(&self, index: usize) -> u32 {
    let mut anim = self.0.animation();
    let value = anim.array[index];

    let time = anim.time;
    anim.array_accesses.push(ArrayAccess { time, index });

    value
  }

  pub fn set(&self, index: usize, value: u32) {
    let mut anim = self.0.animation();
    anim.array[index] = value;
  }

  pub fn swap(&self, a: usize, b: usize) {
    let mut anim = self.0.animation();
    anim.array.swap(a, b);
  }

  pub fn reset_color(&self, index: usize) {
    self.set_color(index, color::TRANSPARENT);
  }

  pub fn set_color(&self, index: usize, color: Color) {
    let mut anim = self.0.animation();
    anim.colors[index] = color;
  }
}

#[derive(Debug)]
pub struct ArrayAccess {
  pub time: f64,
  pub index: usize,
}
