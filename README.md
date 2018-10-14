# sort-visualization

A simple program to visualize sort algorithms written in [Rust](https://www.rust-lang.org/), using [Piston](http://www.piston.rs/) for graphics. Inspired by [**Hopson97/Sort-Algorithm-Visualiser**](https://github.com/Hopson97/Sort-Algorithm-Visualiser).

[![Demo](https://i.imgur.com/jyPDiWX.gif)](https://gist.github.com/dmitmel/f8664421b547577065912c3246f4c1e9)

## Installation

```bash
git clone https://github.com/dmitmel/sort-visualization
cd sort-visualization
cargo install
```

You can use `cargo run` if you don't want to install the binary system-wide.

**Note:** compilation takes a lot of time

## Usage

```bash
# see 'Features' for the list of supported algorithms and their IDs
sort-visualization <algorithm>
# set minimum and maximum values in the array
sort-visualization <algorithm> --min <number> --max <number>
# set order of elements in the array
sort-visualization <algorithm> --order <sorted|reversed|shuffled>
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
cargo doc --document-private-items --open
```

## Contribute

PRs accepted.

## License

[MIT](https://github.com/dmitmel/sort-visualization/blob/master/LICENSE) © [Dmytro Meleshko](https://github.com/dmitmel)
