//! The [`Array`](crate::array::Array) struct.

use graphics::color::TRANSPARENT;
use graphics::types::Color;

use crate::state::{ArrayAccess, SharedState};

use rlua::prelude::*;

/// A convenient wrapper around [`SharedState`] for
/// [algorithms](crate::algorithms) that handles concurrency and all that stuff.
/// **All methods in this struct lock the [state](crate::state::State) for as
/// short as possible** so that the rendering thread can lock it when it wants.
#[derive(Debug, Clone)]
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
    self.set_color(index, TRANSPARENT);
  }

  /// Sets color of the value at a given index.
  ///
  /// _See_ [`State.colors`](crate::state::State::colors)
  pub fn set_color(&self, index: usize, color: Color) {
    let mut state = self.0.get();
    state.colors[index] = color;
  }
}

/// Lua API. For more information see the
/// [lua algorithm](crate::algorithms::lua) module.
impl LuaUserData for Array {
  fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
    #[rustfmt::skip]
    macro_rules! method {
      ($name:ident, $method:expr ) => { methods.add_method(stringify!($name), $method) };
      ($name:ident, $method:expr,) => { method!($name, $method) };
    }

    #[rustfmt::skip]
    macro_rules! meta_method {
      ($meta:expr, $method:expr ) => { methods.add_meta_method($meta, $method) };
      ($meta:expr, $method:expr,) => { meta_method!($meta, $method) };
    }

    method!(wait, |_, array, ms: u64| {
      array.wait(ms);
      Ok(())
    });

    use self::LuaMetaMethod::{Index, Len, NewIndex};
    meta_method!(Len, |_, array, ()| Ok(array.len()));
    meta_method!(Index, |_, array, index: usize| Ok(array.get(index)));
    meta_method!(NewIndex, |_, array, (index, value): (usize, u32)| {
      array.set(index, value);
      Ok(())
    });

    method!(swap, |_, array, (a, b): (usize, usize)| {
      array.swap(a, b);
      Ok(())
    });

    method!(reset_color, |_, array, index: usize| {
      array.reset_color(index);
      Ok(())
    });

    #[rustfmt::skip]
    method!(
      set_color,
      |_, array, (index, r, g, b, a): (usize, f32, f32, f32, f32)| {
        array.set_color(index, [r, g, b, a]);
        Ok(())
      },
    );
  }
}
