# sorting-visualization

[![Travis CI](https://img.shields.io/travis/dmitmel/sorting-visualization.svg?style=flat-square)](https://travis-ci.org/dmitmel/sorting-visualization)
[![docs: GH pages](https://img.shields.io/badge/docs-GH%20pages-blue.svg?style=flat-square)](https://dmitmel.github.io/sorting-visualization)
[![contributors: welcome](https://img.shields.io/badge/contributors-welcome-brightgreen.svg?style=flat-square)](https://github.com/dmitmel/sorting-visualization/pulls)

A [Rust](https://www.rust-lang.org/) program for visualizing sorting algorithms which uses [Piston](http://www.piston.rs/) for graphics. Inspired by [**Hopson97/Sort-Algorithm-Visualiser**](https://github.com/Hopson97/Sort-Algorithm-Visualiser).

[![Demo](https://i.imgur.com/jyPDiWX.gif)](https://gist.github.com/dmitmel/f8664421b547577065912c3246f4c1e9)

## Setup

```bash
git clone https://github.com/dmitmel/sorting-visualization
cd sorting-visualization
cargo build --release
```

## Usage

```bash
# see 'Features' for the list of supported algorithms and their IDs
cargo run <algorithm>
# set length of the array
cargo run <algorithm> --length <number>
# set order of elements in the array
cargo run <algorithm> --order <sorted|reversed|shuffled>
```

## Features

- CLI
- Supports different algorithms (IDs are in brackets):
  - [Bubble sort](https://en.wikipedia.org/wiki/Bubble_sort) \[`bubble`\]
  - [Cycle sort](https://en.wikipedia.org/wiki/Cycle_sort) \[`cycle`\]
  - [Gnome sort](https://en.wikipedia.org/wiki/Gnome_sort) \[`gnome`\]
  - [Insertion sort](https://en.wikipedia.org/wiki/Insertion_sort) \[`insertion`\]
  - [Quicksort](https://en.wikipedia.org/wiki/Quicksort) \[`quicksort`\]
  - [Selection sort](https://en.wikipedia.org/wiki/Selection_sort) \[`selection`\]
- Animation controls:
  - <kbd>Space</kbd> - pause/resume
  - <kbd>&uparrow;</kbd> - 2x faster
  - <kbd>&downarrow;</kbd> - 2x slower
- Easy-to-use algorithm API
- Algorithms can highlight important array elements
- Code is well-documented

## Building docs

```bash
./docs.sh
```

## TODO

1. Ask someone to proof-read the code
2. Add a CLI option to list available algorithms
3. User-friendly GUI
4. More algorithms
5. Sound?

## Contributing

[**Documentation**](https://dmitmel.github.io/sorting-visualization)

PRs are appreciated!

## License

[MIT](https://github.com/dmitmel/sorting-visualization/blob/master/LICENSE) © [Dmytro Meleshko](https://github.com/dmitmel)
