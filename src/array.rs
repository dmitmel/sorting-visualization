//! The [`Array`](crate::array::Array) struct.

use graphics::color;
use graphics::types::Color;

use crate::state::{ArrayAccess, SharedState};

/// A convenient wrapper around [`SharedState`] for
/// [algorithms](crate::algorithms) that handles concurrency and all that stuff.
/// **All methods in this struct lock the [state](crate::state::State) for as
/// short as possible** so that the rendering thread can lock it when it wants.
#[derive(Debug)]
pub struct Array(SharedState);

impl Array {
  /// Creates a new array from a copy of [`SharedState`].
  pub fn new(state: SharedState) -> Self {
    Array(state)
  }

  /// Puts the current thread to sleep for the specified amount of time and
  /// blocks it if the animation is [paused](crate::state::State::paused).
  pub fn wait(&self, ms: u64) {
    use std::thread;
    use std::time::Duration;

    // state must be locked for as short as possible so we shouldn't keep it
    // locked while sleeping (`thread::sleep` and `thread::park`)

    thread::sleep(Duration::from_micros({
      let state = self.0.get();
      (ms as f64 * 1000.0 / state.speed) as u64
    }));

    let paused = {
      let state = self.0.get();
      state.paused
    };

    if paused {
      thread::park();
    }
  }

  /// Returns the length of the underlying [vector](crate::state::State::array).
  pub fn len(&self) -> usize {
    let state = self.0.get();
    state.array.len()
  }

  /// Returns a value at a given index.
  pub fn get(&self, index: usize) -> u32 {
    let mut state = self.0.get();
    let value = state.array[index];

    let time = state.time;
    state.array_accesses.push(ArrayAccess { time, index });

    value
  }

  /// Sets a value of the at a given index.
  pub fn set(&self, index: usize, value: u32) {
    let mut state = self.0.get();
    state.array[index] = value;
  }

  /// Swaps two values at given indices.
  pub fn swap(&self, a: usize, b: usize) {
    let mut state = self.0.get();
    state.array.swap(a, b);
  }

  /// Resets color of the value at a given index (sets it to the transparent
  /// color).
  ///
  /// _See_ [`State.colors`](crate::state::State::colors)
  pub fn reset_color(&self, index: usize) {
    self.set_color(index, color::TRANSPARENT);
  }

  /// Sets color of the value at a given index.
  ///
  /// _See_ [`State.colors`](crate::state::State::colors)
  pub fn set_color(&self, index: usize, color: Color) {
    let mut state = self.0.get();
    state.colors[index] = color;
  }
}
