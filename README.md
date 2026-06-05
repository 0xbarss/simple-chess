# Chess Rust

A chess game implemented in Rust using the Macroquad game library. This project features a graphical user interface, move validation, and a structured board representation.

## Features

- Graphical chess board and piece rendering using Macroquad.
- Legal move generation and validation.
- Support for basic chess rules and turn-based gameplay.
- Interactive mouse-based controls for selecting and moving pieces.
- Visual highlights for selected pieces and valid moves.

## Prerequisites

To build and run this project, you need to have Rust and Cargo installed on your system. You can install them from [rustup.rs](https://rustup.rs/).

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/0xbarss/simple-chess.git
   cd simple-chess
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

## Usage

Run the application using Cargo:

```bash
cargo run
```

### Controls
- Left-click on a piece to select it. Valid moves will be highlighted.
- Left-click on a highlighted square to move the selected piece.
- Left-click elsewhere to deselect a piece.

## Project Structure

- `src/main.rs`: The entry point of the application, handling the game loop and input processing.
- `src/board.rs`: Logic for representing the chess board and its state.
- `src/pieces.rs`: Definitions for chess pieces and squares.
- `src/moves.rs`: Implementation of chess move logic.
- `src/rules.rs`: Chess rules and legal move generation.
- `src/render/`: Graphics and rendering logic.
    - `mod.rs`: Renderer initialization.
    - `board.rs`: Board rendering constants and functions.
    - `pieces.rs`: Logic for drawing pieces on the board.
    - `input.rs`: Visual feedback for input states and highlights.
- `assets/`: Image assets for pieces and tiles.

## License

This project is licensed under the MIT License - see the LICENSE file for details (if applicable).
