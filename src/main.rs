use opengl_graphics::*;
use piston::event_loop::*;
use piston::input::*;
use piston::window::WindowSettings;
use sdl2_window::Sdl2Window as Window;

mod algorithms;
mod app;
mod array;
mod cli;
mod state;

use crate::app::App;
use crate::cli::{Options, Order};

/// Required version of OpenGL.
///
/// _Note:_ change this to [`OpenGL::V2_1`] if it doesn't work.
const OPENGL_VERSION: OpenGL = OpenGL::V3_2;

/// Title of the main window.
const WINDOW_TITLE: &str = clap::crate_name!();
/// Initial size of the main window.
const WINDOW_SIZE: (u32, u32) = (640, 480);

fn main() {
  let Options {
    algorithm,
    length,
    order,
    speed,
  } = cli::parse_options();

  let window_title = format!("{} - {}", WINDOW_TITLE, algorithm.name());
  let mut window: Window = WindowSettings::new(window_title, WINDOW_SIZE)
    .opengl(OPENGL_VERSION)
    .exit_on_esc(true)
    .vsync(true)
    .build()
    .expect("couldn't create window");
  let mut gl = GlGraphics::new(OPENGL_VERSION);

  let mut array: Vec<u32> = (1..=length).collect();

  match order {
    Order::Sorted => {}
    Order::Reversed => array.reverse(),
    Order::Shuffled => {
      use rand::{thread_rng, Rng};
      thread_rng().shuffle(&mut array);
    }
  }

  // load font for the status text
  let font = include_bytes!("../assets/Menlo-Regular.ttf");
  let mut glyphs = GlyphCache::from_bytes(font, (), TextureSettings::new())
    .expect("couldn't load font");
  // preload printable ASCII chars for faster rendering
  glyphs
    .preload_printable_ascii(app::STATUS_TEXT_FONT_SIZE)
    .expect("couldn't preload printable ASCII chars");

  let mut app = App::init(algorithm, array, speed);

  println!("Press [Space] to pause/resume the animation");
  println!("Press [Up]    to speed up the animation");
  println!("Press [Down]  to slow down the animation");
  println!();

  let mut events = Events::new(EventSettings::new());
  while let Some(event) = events.next(&mut window) {
    match event {
      Event::Loop(Loop::Render(args)) => app.render(args, &mut gl, &mut glyphs),
      Event::Loop(Loop::Update(args)) => app.update(args),
      Event::Input(Input::Button(args)) => app.button(args),
      _ => {}
    }
  }
}
