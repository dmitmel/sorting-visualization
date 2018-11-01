//! Structs and type definitions that describe the app state.

use std::sync::{Arc, Condvar, Mutex, MutexGuard};

use graphics::types::Color;

/// A wrapper around [`State`] that can be [safely shared](Send) between threads.
pub type SharedState = Arc<State>;

/// Contains the state of the whole [app](crate::app::App). It also has
/// [synchronization](Sync).
#[derive(Debug)]
pub struct State {
  /// A synchronized [animation state](AnimationState). A mutex is used here
  /// because the animation state is accessed by both the main and the algorithm
  /// thread.
  pub animation: Mutex<AnimationState>,
  /// A condition variable which is used to block the
  /// [algorithm](crate::algorithms::Algorithm) thread when the animation is
  /// [paused](AnimationState::paused).
  ///
  /// _See_ [`Array.wait`](crate::array::Array::wait)
  pub pause_notifier: Condvar,
}

impl State {
  /// Safe getter for the [animation state](State::animation). It should be used
  /// instead of directly locking the mutex!
  pub fn animation(&self) -> MutexGuard<'_, AnimationState> {
    self.animation.lock().unwrap()
  }
}

/// The state of animation of the [algorithm](crate::algorithms::Algorithm)
/// visualization.
#[derive(Debug)]
pub struct AnimationState {
  /// Current time in seconds. [Updated](crate::app::App::update) if the
  /// animation is not [paused](AnimationState::paused).
  pub time: f64,
  /// Speed factor (e.g. 1.0 - normal, 2.0 - 2x faster, 0.5 - 2x slower).
  /// Affects the animation [time](AnimationState::time) and
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
  /// The length of this vector is equal to the [array](AnimationState::array)
  /// length, so every color in this vector matches a value with the exact same
  /// index.
  pub colors: Vec<Color>,
  /// Contains indexes and timestamps of recent array accesses which are drawn
  /// as [colored overlays](AnimationState::colors). When an
  /// [algorithm](crate::algorithms::Algorithm)
  /// [reads](crate::array::Array::get) a value from the array a new
  /// [`ArrayAccess`] with index and the current [time](AnimationState::time) is
  /// pushed to this vector.
  pub array_accesses: Vec<ArrayAccess>,
}

#[derive(Debug)]
pub struct ArrayAccess {
  pub time: f64,
  pub index: usize,
}
