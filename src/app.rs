use graphics;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

const BACKGROUND_COLOR: [f32; 4] = graphics::color::BLACK;

pub struct App {}

impl App {
  pub fn new() -> Self {
    App {}
  }

  pub fn render(&mut self, gl: &mut GlGraphics, args: RenderArgs) {
    gl.draw(args.viewport(), |ctx, gl| {
      graphics::clear(BACKGROUND_COLOR, gl);
    });
  }

  pub fn update(&mut self, args: UpdateArgs) {}
}
