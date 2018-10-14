//! The [`Array`] struct.

use graphics::color;
use graphics::types::Color;

use state::{ArrayAccess, SharedState};

/// A convenient wrapper around [`SharedState`] for [algorithms](::algorithms)
/// that handles concurrency and all that stuff. **All methods in this struct
/// lock the [state](`::state::AnimationState`) for as short as possible** so
/// that the rendering thread can lock it when it wants to.
#[derive(Debug)]
pub struct Array(SharedState);

impl Array {
  /// Creates a new array from a copy of [`SharedState`].
  pub fn new(state: SharedState) -> Self {
    Array(state)
  }

  /// Puts the current thread to sleep for the specified amount of time and
  /// blocks it if the animation is [paused](`::state::AnimationState::paused`).
  pub fn wait(&self, ms: u64) {
    use std::thread;
    use std::time::Duration;

    thread::sleep(Duration::from_micros({
      // animation state should be locked for as short as possible so we lock it
      // here, get the delay, then unlock it and finally lock it after
      // `thread::sleep` finishes
      let anim = self.0.animation();
      (ms as f64 * 1000.0 / anim.speed) as u64
    }));

    // wait while the animation is paused using a Condvar (API is a bit weird)
    let mut anim = self.0.animation();
    while anim.paused {
      anim = self.0.pause_notifier.wait(anim).unwrap();
    }
  }

  /// Returns the length of the underlying
  /// [vector](::state::AnimationState::array).
  pub fn len(&self) -> usize {
    let anim = self.0.animation();
    anim.array.len()
  }

  /// Returns a value at a given index.
  pub fn get(&self, index: usize) -> u32 {
    let mut anim = self.0.animation();
    let value = anim.array[index];

    let time = anim.time;
    anim.array_accesses.push(ArrayAccess { time, index });

    value
  }

  /// Sets a value of the at a given index.
  pub fn set(&self, index: usize, value: u32) {
    let mut anim = self.0.animation();
    anim.array[index] = value;
  }

  /// Swaps two values at given indices.
  pub fn swap(&self, a: usize, b: usize) {
    let mut anim = self.0.animation();
    anim.array.swap(a, b);
  }

  /// Resets color of the value at a given index (sets it to the transparent
  /// color).
  ///
  /// _See_ [`AnimationState.colors`](::state::AnimationState::colors)
  pub fn reset_color(&self, index: usize) {
    self.set_color(index, color::TRANSPARENT);
  }

  /// Sets color of the value at a given index.
  ///
  /// _See_ [`AnimationState.colors`](::state::AnimationState::colors)
  pub fn set_color(&self, index: usize, color: Color) {
    let mut anim = self.0.animation();
    anim.colors[index] = color;
  }
}
