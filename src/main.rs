extern crate rand;

extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate sdl2_window;

use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window as Window;

mod algorithms;
mod app;
mod array;
mod utils;

use algorithms::Algorithm;
use app::App;

const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

const WINDOW_NAME: &str = "Sort Visualization";
const WINDOW_SIZE: (u32, u32) = (640, 480);

fn main() {
  let algorithm = algorithms::BubbleSort;

  let window_name = format!("{} - {}", WINDOW_NAME, algorithm.name());
  let mut window: Window = WindowSettings::new(window_name, WINDOW_SIZE)
    .opengl(OPENGL_VERSION)
    .exit_on_esc(true)
    .vsync(true)
    .build()
    .expect("couldn't create window");
  let mut gl = GlGraphics::new(OPENGL_VERSION);

  let mut app = App::new(algorithm, 100);

  let mut events = Events::new(EventSettings::new());
  while let Some(event) = events.next(&mut window) {
    match event {
      Event::Loop(Loop::Render(args)) => app.render(&mut gl, args),
      Event::Loop(Loop::Update(args)) => app.update(args),
      Event::Input(Input::Button(args)) => app.button(args),
      _ => {}
    }
  }
}
