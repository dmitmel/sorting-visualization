use graphics::color;
use graphics::types::Color;

use state::SharedState;

#[derive(Debug)]
pub struct Array(SharedState);

impl Array {
  pub fn new(state: SharedState) -> Self {
    Array(state)
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

  pub fn reset_all_colors(&self) {
    let mut anim = self.0.animation();
    for color in anim.colors.iter_mut() {
      *color = color::TRANSPARENT;
    }
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
