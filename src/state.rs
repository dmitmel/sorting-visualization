//! Structs and type definitions that describe the app state.

use std::sync::{Arc, Mutex, MutexGuard};

use graphics::types::Color;

/// A wrapper around [`State`] that can be [safely shared](std::sync) between
/// threads.
#[derive(Debug, Clone)]
pub struct SharedState(Arc<Mutex<State>>);

impl SharedState {
  pub fn new(state: State) -> Self {
    SharedState(Arc::new(Mutex::new(state)))
  }

  pub fn get(&self) -> MutexGuard<'_, State> {
    self.0.lock().unwrap()
  }
}

/// Contains the state of the whole [app](crate::app::App).
#[derive(Debug)]
pub struct State {
  /// Current time in seconds. [Updated](crate::app::App::update) if the
  /// animation is not [paused](State::paused).
  pub time: f64,
  /// Speed factor (e.g. 1.0 - normal, 2.0 - 2x faster, 0.5 - 2x slower, etc).
  /// Affects the animation [time](State::time) and
  /// [delays](crate::array::Array::wait) in
  /// [algorithms](crate::algorithms::Algorithm).
  pub speed: f64,
  /// Is the animation paused?
  pub paused: bool,
  /// An array which is being sorted.
  pub array: Vec<u32>,
  /// Colored **overlays** of each value. **Overlay** means that these colors
  /// are _not_ used to [draw](crate::app::App::render) the values, instead
  /// they're drawn **over** the values. These colors can be used by an
  /// [algorithm](crate::algorithms::Algorithm) to highlight important array
  /// elements.
  ///
  /// The length of this vector is equal to the [array](State::array)
  /// length, so every color in this vector corresponds to a value with the
  /// exact same index.
  pub colors: Vec<Color>,
  /// Contains timestamps of the most recent array accesses of every element,
  /// which are drawn like [colored overlays](State::colors). The length of this
  /// vector is equal to the [array](State::array) length, so for every element
  /// in the [array](State::array) _only the most recent_ timestamp is stored
  /// here (not a big deal), so we can avoid memory allocations while program is
  /// running, and this is a pretty simple optimization. To optimize things even
  /// further I've done this: if an element hasn't been accessed yet or it has
  /// been accessed [long ago](crate::app::ACCESSED_VALUE_TIMEOUT), then instead
  /// of using an `Option<f64>` type for every element, I use negative values
  /// because they don't make any sense in the context of time (this saves
  /// [8 bytes](https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0e335d0d6109850c7df766fd131c8916)
  /// for every element). When an [algorithm](crate::algorithms::Algorithm)
  /// [reads](crate::array::Array::get) or [writes](crate::array::Array::set) a
  /// value at a certain index from/to the array, the current
  /// [time](State::time) is stored at exactly the same index in this vector.
  pub array_accesses: Vec<f64>,
}

/// A constant that means "there's no array access here". _See_ documentation of
/// [`State.array_accesses`](State::array_accesses) to understand why this
/// constant is a negative number.
pub const NO_ARRAY_ACCESS: f64 = -1.0;
