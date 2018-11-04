//! The [`App`] struct and some constants.

use graphics::color::{BLACK, TRANSPARENT, WHITE};
use graphics::types::Color;
use opengl_graphics::{GlGraphics, GlyphCache};
use piston::input::*;

use std::thread;

use crate::algorithms::Algorithm;
use crate::array::Array;

use crate::state::*;

/// Color that is used to clear the window before [drawing](App::render).
pub const BACKGROUND_COLOR: Color = BLACK;

/// Color of rectangles that represent the array values.
pub const VALUE_COLOR: Color = WHITE;

/// Color of the values that were recently accessed.
///
/// _See_ [`State.array_accesses`](State::array_accesses)
pub const ACCESSSED_VALUE_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

/// Time in seconds after which array accesses get removed.
///
/// _See_ [`State.array_accesses`](State::array_accesses)
pub const ACCESSED_VALUE_TIMEOUT: f64 = 0.25;

/// Font size of the status text in pixels.
pub const STATUS_TEXT_FONT_SIZE: u32 = 16;
/// Margins between the status text and window borders.
pub const STATUS_TEXT_MARGIN: f64 = 8.0;

/// This struct contains all [rendering](App::render), [updating](App::update)
/// and [input handling](App::button) logic.
#[derive(Debug)]
pub struct App {
  state: SharedState,
  algorithm_thread: thread::JoinHandle<()>,
}

impl App {
  /// Creates a new app (with a state constructed from the given `array`) and
  /// starts an algorithm thread. This function is called `init` instead of
  /// `new` because it has side effects.
  pub fn init(
    algorithm: Box<dyn Algorithm + Send>,
    array: Vec<u32>,
    speed: f64,
  ) -> Self {
    let colors = vec![TRANSPARENT; array.len()];

    let state = SharedState::new(State {
      time: 0.0,
      speed,
      paused: true,
      array,
      colors,
      array_accesses: Vec::with_capacity(1024),
    });

    let algorithm_state = state.clone();

    let algorithm_thread = thread::Builder::new()
      .name("algorithm".to_string())
      .spawn(move || {
        let array = Array::new(algorithm_state);
        array.wait(500);
        algorithm.sort(array);
      })
      .unwrap();

    Self {
      state,
      algorithm_thread,
    }
  }

  /// Draws the current [state](State).
  pub fn render(
    &mut self,
    args: RenderArgs,
    gl: &mut GlGraphics,
    glyphs: &mut GlyphCache<'_>,
  ) {
    gl.draw(args.viewport(), |ctx, gl| {
      use graphics::*;

      clear(BACKGROUND_COLOR, gl);

      // lock the state for the whole rendering cycle so that the algorithm
      // thread doesn't change something while the main thread is doing
      // rendering
      let state = self.state.get();

      // transform of the bottom left point of the status text
      let status_text_transform = ctx.transform.trans(
        STATUS_TEXT_MARGIN,
        STATUS_TEXT_MARGIN + f64::from(STATUS_TEXT_FONT_SIZE),
      );

      let status_text = format!(
        "paused = {}, speed = {}%",
        state.paused,
        state.speed * 100.0
      );

      // draw the status text
      text::Text::new_color(WHITE, STATUS_TEXT_FONT_SIZE)
        .draw(
          &status_text,
          glyphs,
          &ctx.draw_state,
          status_text_transform,
          gl,
        )
        .unwrap();

      let len = state.array.len();
      let max_value = *state.array.iter().max().unwrap_or(&0);

      // draws a rectangle with a given color which represents a value at a
      // specified index
      let mut draw_value = |index: usize, color: Color| {
        let value = state.array[index];

        let window_w = f64::from(args.width);
        let window_h = f64::from(args.height);

        let array_y =
          STATUS_TEXT_MARGIN * 2.0 + f64::from(STATUS_TEXT_FONT_SIZE);

        let array_h = window_h - array_y;

        let w = window_w / (len as f64);
        let h = f64::from(value) * array_h / f64::from(max_value);
        let x = (index as f64) * w;
        let y = window_h - h;

        rectangle(color, [x, y, w, h], ctx.transform, gl);
      };

      // draw all values
      for index in 0..state.array.len() {
        draw_value(index, VALUE_COLOR);
      }

      // draw array accesses
      for access in &state.array_accesses {
        let mut color = ACCESSSED_VALUE_COLOR;
        // map age of this access to the [1.0, 0.0] interval (alpha component)
        // so that new accesses are opaque and old ones are transparent
        color[3] =
          (1.0 - (state.time - access.time) / ACCESSED_VALUE_TIMEOUT) as f32;

        draw_value(access.index, color);
      }

      // draw colored overlays (marks) for some values
      for (index, color) in state.colors.iter().enumerate() {
        draw_value(index, *color);
      }
    });
  }

  /// Advances the [state](State) by a given amount of [time](UpdateArgs::dt).
  pub fn update(&mut self, args: UpdateArgs) {
    let mut state = self.state.get();

    if !state.paused {
      state.time += args.dt * state.speed;
    }

    // time is copied (f64 implements the Copy trait) to a variable because it
    // can't be accessed when array_accesses is borrowed mutably
    let time = state.time;
    let accesses = &mut state.array_accesses;
    accesses.retain(|access| time - access.time < ACCESSED_VALUE_TIMEOUT);
  }

  /// Handles user input and updates the [state](State).
  ///
  /// # Controls
  ///
  /// | Key                    | Action       |
  /// | ---------------------- | ------------ |
  /// | <kbd>Space</kbd>       | pause/resume |
  /// | <kbd>&uparrow;</kbd>   | 2x faster    |
  /// | <kbd>&downarrow;</kbd> | 2x slower    |
  pub fn button(&mut self, args: ButtonArgs) {
    let mut state = self.state.get();

    // import commonly used enum values in the current scope
    use self::Button::Keyboard;
    use self::ButtonState::Press;

    match (args.button, args.state) {
      (Keyboard(Key::Space), Press) => {
        state.paused = !state.paused;
        self.algorithm_thread.thread().unpark();
      }

      (Keyboard(Key::Up), Press) => state.speed *= 2.0,
      (Keyboard(Key::Down), Press) => state.speed /= 2.0,

      _ => {}
    };
  }
}
