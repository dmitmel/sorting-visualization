//! The [`App`] struct and some constants.

use graphics;
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::input::*;

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use algorithms::Algorithm;
use array::Array;

use state::*;

/// Color that is used to clear the window before [drawing](App::render).
const BACKGROUND_COLOR: Color = graphics::color::BLACK;

/// Color of rectangles that represent the array values.
const VALUE_COLOR: Color = graphics::color::WHITE;

/// Color of the values that were recently accessed.
///
/// _See_ [`AnimationState.array_accesses`](AnimationState::array_accesses)
const ACCESSSED_VALUE_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

/// Time in seconds after which array accesses get removed.
///
/// _See_ [`AnimationState.array_accesses`](AnimationState::array_accesses)
const ACCESSED_VALUE_TIMEOUT: f64 = 0.25;

/// This struct contains all [rendering](App::render), [updating](App::update)
/// and [input handling](App::button) logic.
#[derive(Debug)]
pub struct App(SharedState);

impl App {
  /// Creates a new app (with a state constructed from the given `array`) and
  /// starts an algorithm thread. This function is called `init` instead of
  /// `new` because it has side effects.
  pub fn init(
    algorithm: Box<dyn Algorithm + Send>,
    array: Vec<u32>,
    speed: f64,
  ) -> Self {
    let colors = vec![graphics::color::TRANSPARENT; array.len()];

    let state = Arc::new(State {
      animation: Mutex::new(AnimationState {
        time: 0.0,
        speed,
        paused: true,
        array,
        colors,
        array_accesses: Vec::with_capacity(1024),
      }),
      pause_notifier: Condvar::new(),
    });

    let algorithm_state = state.clone();

    thread::Builder::new()
      .name("algorithm".to_string())
      .spawn(move || {
        let array = Array::new(algorithm_state);
        array.wait(500);
        algorithm.sort(array);
      })
      .unwrap();

    App(state)
  }

  /// Draws the current [animation state](AnimationState).
  pub fn render(&mut self, gl: &mut GlGraphics, args: RenderArgs) {
    let window_w = f64::from(args.width);
    let window_h = f64::from(args.height);

    gl.draw(args.viewport(), |ctx, gl| {
      graphics::clear(BACKGROUND_COLOR, gl);

      // lock the animation state for the whole rendering cycle so that
      // algorithm thread doesn't change something while rendering
      let anim = self.0.animation();

      let len = anim.array.len();
      let max_value = *anim.array.iter().max().unwrap_or(&0);

      // draws a rectangle with a given color which represents a value at a
      // specified index
      let mut draw_value = |index: usize, color: Color| {
        let value = anim.array[index];

        let w = window_w / (len as f64);
        let h = f64::from(value) * window_h / f64::from(max_value);
        let x = (index as f64) * w;
        let y = window_h - h;

        graphics::rectangle(color, [x, y, w, h], ctx.transform, gl);
      };

      // draw all values
      for index in 0..anim.array.len() {
        draw_value(index, VALUE_COLOR);
      }

      // draw array accesses
      for access in &anim.array_accesses {
        let mut color = ACCESSSED_VALUE_COLOR;
        // map age of this access to the [1.0, 0.0] interval (alpha component)
        // so that new accesses are opaque and old ones are transparent
        color[3] =
          (1.0 - (anim.time - access.time) / ACCESSED_VALUE_TIMEOUT) as f32;

        draw_value(access.index, color);
      }

      // draw colored overlays (marks) for some values
      for (index, color) in anim.colors.iter().enumerate() {
        draw_value(index, *color);
      }
    });
  }

  /// Advances the [animation state](AnimationState) by a given amount
  /// of [time](UpdateArgs::dt).
  pub fn update(&mut self, args: UpdateArgs) {
    let mut anim = self.0.animation();

    if !anim.paused {
      anim.time += args.dt * anim.speed;
    }

    // time is copied (f64 implements the Copy trait) to a variable because it
    // can't be accessed when array_accesses is borrowed mutably
    let time = anim.time;
    let accesses = &mut anim.array_accesses;
    accesses.retain(|access| time - access.time < ACCESSED_VALUE_TIMEOUT);
  }

  /// Handles user input and updates the [animation state](AnimationState). It
  /// also prints the important bits of the state if it has actually changed.
  ///
  /// # Controls
  ///
  /// | Key                    | Action       |
  /// | ---------------------- | ------------ |
  /// | <kbd>Space</kbd>       | pause/resume |
  /// | <kbd>&uparrow;</kbd>   | 2x faster    |
  /// | <kbd>&downarrow;</kbd> | 2x slower    |
  pub fn button(&mut self, args: ButtonArgs) {
    let mut anim = self.0.animation();

    // import commonly used enum values in the current scope
    use Button::Keyboard;
    use ButtonState::Press;

    let state_was_updated = match (args.button, args.state) {
      (Keyboard(Key::Space), Press) => {
        anim.paused = !anim.paused;
        // tell the algorithm thread that the state has been updated
        self.0.pause_notifier.notify_all();
        true
      }

      (Keyboard(Key::Up), Press) => {
        anim.speed *= 2.0;
        true
      }

      (Keyboard(Key::Down), Press) => {
        anim.speed /= 2.0;
        true
      }

      _ => false,
    };

    if state_was_updated {
      println!("paused = {}, speed = {}", anim.paused, anim.speed);
    }
  }
}
