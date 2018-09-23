use graphics;
use graphics::types::Color;
use opengl_graphics::GlGraphics;
use piston::input::*;

use rand::{thread_rng, Rng};

use std::sync::{Arc, Mutex};
use std::thread;

use algorithms::Algorithm;
use array::{Array, ArrayAccess};

const BACKGROUND_COLOR: Color = graphics::color::BLACK;
const RECTANGLE_COLOR: Color = graphics::color::WHITE;
const CHANGED_RECTANGLE_COLOR: Color = [1.0, 0.0, 0.0, 1.0];

const MESSAGE_TIMEOUT: f64 = 0.25;

#[derive(Debug)]
pub struct State {
  pub time: f64,
  pub array: Vec<u32>,
  pub colors: Vec<Color>,
  pub array_accesses: Vec<ArrayAccess>,
}

#[derive(Debug)]
pub struct App(Arc<Mutex<State>>);

impl App {
  pub fn new<A>(algorithm: A, max_value: u32) -> Self
  where
    A: Algorithm + Send + 'static,
  {
    let mut array: Vec<u32> = (1..=max_value).collect();
    thread_rng().shuffle(&mut array);

    let colors = vec![graphics::color::TRANSPARENT; array.len()];

    let state = Arc::new(Mutex::new(State {
      time: 0.0,
      array,
      colors,
      array_accesses: Vec::with_capacity(1024),
    }));

    let algorithm_state = state.clone();

    thread::Builder::new()
      .name("algorithm".to_string())
      .spawn(move || {
        use utils::delay;
        delay(500);

        algorithm.sort(Array::new(algorithm_state));
      }).unwrap();

    App(state)
  }

  pub fn render(&mut self, gl: &mut GlGraphics, args: RenderArgs) {
    let window_w = f64::from(args.width);
    let window_h = f64::from(args.height);

    gl.draw(args.viewport(), |ctx, gl| {
      graphics::clear(BACKGROUND_COLOR, gl);

      let state = self.0.lock().unwrap();

      let array_len = state.array.len();
      let max_value = state.array.iter().max().unwrap_or(&0);

      let mut draw_value = |index: usize, color: Color| {
        let value = state.array[index];

        let w = window_w / array_len as f64;
        let h = (value as f64) * window_h / (*max_value as f64);
        let x = (index as f64) * w;
        let y = window_h - h;
        graphics::rectangle(color, [x, y, w, h], ctx.transform, gl);
      };

      for index in 0..state.array.len() {
        draw_value(index, RECTANGLE_COLOR);
      }

      for access in &state.array_accesses {
        let mut color = CHANGED_RECTANGLE_COLOR;
        color[3] = (1.0 - (state.time - access.time) / MESSAGE_TIMEOUT) as f32;
        draw_value(access.index, color);
      }

      for (index, color) in state.colors.iter().enumerate() {
        draw_value(index, *color);
      }
    });
  }

  pub fn update(&mut self, args: UpdateArgs) {
    let mut state = self.0.lock().unwrap();

    state.time += args.dt;

    let time = state.time;
    state
      .array_accesses
      .retain(|access| time - access.time < MESSAGE_TIMEOUT);
  }

  pub fn button(&mut self, args: ButtonArgs) {
    println!("{:?}", args);
  }
}
