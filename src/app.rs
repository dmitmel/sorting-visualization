use graphics;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

use rand::{thread_rng, Rng};

const BACKGROUND_COLOR: [f32; 4] = graphics::color::BLACK;
const RECTANGLE_COLOR: [f32; 4] = graphics::color::WHITE;

pub struct App {
  values: Vec<u32>,
}

impl App {
  pub fn new() -> Self {
    let mut values: Vec<u32> = (0..100).collect();
    thread_rng().shuffle(&mut values);
    App { values }
  }

  pub fn render(&mut self, gl: &mut GlGraphics, args: RenderArgs) {
    let window_width = f64::from(args.width);
    let window_height = f64::from(args.height);

    gl.draw(args.viewport(), |ctx, gl| {
      graphics::clear(BACKGROUND_COLOR, gl);

      let values_len = self.values.len();
      let max_val = self.values.iter().max().unwrap_or(&0);

      for (index, val) in self.values.iter().enumerate() {
        let width = window_width / values_len as f64;
        let height = f64::from(*val) * window_height / f64::from(*max_val);

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
