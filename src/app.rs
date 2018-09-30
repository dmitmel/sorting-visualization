use graphics;
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::input::*;

use rand::{thread_rng, Rng};

use std::sync::{Arc, Condvar, Mutex};
use std::thread;

use algorithms::Algorithm;
use array::Array;

use state::{AnimationState, SharedState, State};

const BACKGROUND_COLOR: Color = graphics::color::BLACK;
const VALUE_COLOR: Color = graphics::color::WHITE;
const ACCESSSED_VALUE_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

const ACCESSED_VALUE_TIMEOUT: f64 = 0.25;

#[derive(Debug)]
pub struct App(SharedState);

impl App {
  pub fn new<A>(algorithm: A, max_value: u32) -> Self
  where
    A: Algorithm + Send + 'static,
  {
    let mut array: Vec<u32> = (1..=max_value).collect();
    thread_rng().shuffle(&mut array);

    let colors = vec![graphics::color::TRANSPARENT; array.len()];

    let state = Arc::new(State {
      animation: Mutex::new(AnimationState {
        time: 0.0,
        speed: 1.0,
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
      }).unwrap();

    App(state)
  }

  pub fn render(&mut self, gl: &mut GlGraphics, args: RenderArgs) {
    let window_w = f64::from(args.width);
    let window_h = f64::from(args.height);

    gl.draw(args.viewport(), |ctx, gl| {
      graphics::clear(BACKGROUND_COLOR, gl);

      let anim = self.0.animation();

      let array_len = anim.array.len();
      let max_value = anim.array.iter().max().unwrap_or(&0);

      let mut draw_value = |index: usize, color: Color| {
        let value = anim.array[index];

        let w = window_w / array_len as f64;
        let h = (value as f64) * window_h / (*max_value as f64);
        let x = (index as f64) * w;
        let y = window_h - h;
        graphics::rectangle(color, [x, y, w, h], ctx.transform, gl);
      };

      for index in 0..anim.array.len() {
        draw_value(index, VALUE_COLOR);
      }

      for access in &anim.array_accesses {
        let mut color = ACCESSSED_VALUE_COLOR;
        color[3] =
          (1.0 - (anim.time - access.time) / ACCESSED_VALUE_TIMEOUT) as f32;
        draw_value(access.index, color);
      }

      for (index, color) in anim.colors.iter().enumerate() {
        draw_value(index, *color);
      }
    });
  }

  pub fn update(&mut self, args: UpdateArgs) {
    let mut anim = self.0.animation();

    if !anim.paused {
      anim.time += args.dt * anim.speed;
    }

    let time = anim.time;
    anim
      .array_accesses
      .retain(|access| time - access.time < ACCESSED_VALUE_TIMEOUT);
  }

  pub fn button(&mut self, args: ButtonArgs) {
    let mut anim = self.0.animation();

    if let Button::Keyboard(key) = args.button {
      match (key, args.state) {
        (Key::Space, ButtonState::Press) => {
          anim.paused = !anim.paused;
          self.0.pause_notifier.notify_all();
        }
        (Key::Up, ButtonState::Press) => anim.speed *= 2.0,
        (Key::Down, ButtonState::Press) => anim.speed /= 2.0,
        _ => {}
      }
    }
  }
}
