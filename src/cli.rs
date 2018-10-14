use clap::{AppSettings, Arg};

use algorithms;
use algorithms::Algorithm;

const RANGE_START_OPT: &str = "RANGE_START";
const RANGE_END_OPT: &str = "RANGE_END";
const ORDER_OPT: &str = "ORDER";

const ALGORITHM_ARG: &str = "ALGORITHM";

pub struct Options {
  pub algorithm: Box<dyn Algorithm + Send>,
  pub range_start: u32,
  pub range_end: u32,
  pub order: Order,
}

pub enum Order {
  Sorted,
  Reversed,
  Shuffled,
}

pub fn get_options() -> Options {
  let parser = app_from_crate!()
    .setting(AppSettings::NextLineHelp)
    .setting(AppSettings::ColoredHelp)
    .arg(
      Arg::with_name(RANGE_START_OPT)
        .short("s")
        .long("range-start")
        .help("min value in the array")
        .default_value("1"),
    ).arg(
      Arg::with_name(RANGE_END_OPT)
        .short("e")
        .long("range-end")
        .help("max value in the array")
        .default_value("100"),
    ).arg(
      Arg::with_name(ORDER_OPT)
        .short("o")
        .long("order")
        .help("order of the array")
        .possible_values(&["sorted", "reversed", "shuffled"])
        .case_insensitive(true)
        .default_value("shuffled"),
    ).arg(
      Arg::with_name(ALGORITHM_ARG)
        .help("sorting algorithm")
        .possible_values(&[
          "bubble",
          "cycle",
          "gnome",
          "insertion",
          "quicksort",
          "selection",
        ]).case_insensitive(true)
        .required(true)
        .index(1),
    );
  let matches = parser.get_matches();

  Options {
    algorithm: match matches.value_of(ALGORITHM_ARG).unwrap() {
      "bubble" => Box::new(algorithms::BubbleSort),
      "cycle" => Box::new(algorithms::CycleSort),
      "gnome" => Box::new(algorithms::GnomeSort),
      "insertion" => Box::new(algorithms::InsertionSort),
      "quicksort" => Box::new(algorithms::Quicksort),
      "selection" => Box::new(algorithms::SelectionSort),
      _ => unreachable!(),
    },

    range_start: value_t_or_exit!(matches, RANGE_START_OPT, u32),
    range_end: value_t_or_exit!(matches, RANGE_END_OPT, u32),

    order: match matches.value_of(ORDER_OPT).unwrap() {
      "sorted" => Order::Sorted,
      "reversed" => Order::Reversed,
      "shuffled" => Order::Shuffled,
      _ => unreachable!(),
    },
  }
}
