# Minesweeper

Console based minesweeper built in Rust using [crossterm](https://crates.io/crates/crossterm)
## Table of Contents

- [Images](#images)
- [Installation](#installation)
- [Usage](#usage)

## Images

![an image of a large minesweeper board](images/board1.PNG)

![an image of a small minesweeper board](images/board2.PNG)

## Installation

### Executable

Download the [latest release](https://github.com/2C0BB/minesweeper/releases/latest)

### Repository

Clone to your project directory and build with cargo

```sh
git clone https://github.com/2C0BB/minesweeper.git
cargo build --release
```

## Usage

Run the executable, board size and amount of mines can be changed with command line args

```sh
./minesweeper <board size> <num of mines>
```
