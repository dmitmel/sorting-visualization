use graphics;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use rand::{thread_rng, Rng};

use algorithms::Algorithm;
use array::{Array, Value};
use std::thread;

const BACKGROUND_COLOR: [f32; 4] = graphics::color::BLACK;
const RECTANGLE_COLOR: [f32; 4] = graphics::color::WHITE;

pub struct App {
  array: Array,
}

impl App {
  pub fn new<A>(algorithm: A, max_value: Value) -> Self
  where
    A: Algorithm + Send + 'static,
  {
    let mut values: Vec<Value> = (1..=max_value).collect();
    thread_rng().shuffle(&mut values);

    let array = Array::new(values);
    let algorithm_array = array.clone();

    thread::Builder::new()
      .name("algorithm".to_string())
      .spawn(move || algorithm.sort(algorithm_array))
      .unwrap();

    App { array }
  }

  pub fn render(&mut self, gl: &mut GlGraphics, args: RenderArgs) {
    let window_width = f64::from(args.width);
    let window_height = f64::from(args.height);

    gl.draw(args.viewport(), |ctx, gl| {
      graphics::clear(BACKGROUND_COLOR, gl);

      let values = self.array.lock();

      let values_len = values.len();
      let max_val = values.iter().max().unwrap_or(&0);

      for (index, val) in values.iter().enumerate() {
        let width = window_width / values_len as f64;
        let height = (*val as f64) * window_height / (*max_val as f64);

        let x = (index as f64) * width;
        let y = window_height - height;

        graphics::rectangle(
          RECTANGLE_COLOR,
          [x, y, width, height],
          ctx.transform,
          gl,
        );
      }
    });
  }

  pub fn update(&mut self, args: UpdateArgs) {}
}
