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
  /// Contains indexes and timestamps of recent array accesses which are drawn
  /// as [colored overlays](State::colors). When an
  /// [algorithm](crate::algorithms::Algorithm)
  /// [reads](crate::array::Array::get) a value from the array a new
  /// [`ArrayAccess`] with index and the current [time](State::time) is
  /// pushed to this vector.
  pub array_accesses: Vec<ArrayAccess>,
}

#[derive(Debug)]
pub struct ArrayAccess {
  pub time: f64,
  pub index: usize,
}
