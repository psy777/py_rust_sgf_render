# Python-Rust SGF Renderer

A high-performance SGF (Smart Game Format) renderer that converts Go/Baduk game files into PNG images. The project combines Python's ease of use with Rust's performance, leveraging the Skia graphics library for high-quality rendering of Go game positions.

## Overview

This library provides a powerful tool for converting SGF files into clear, high-quality PNG images. It's particularly useful for:
- Documenting Go games and positions
- Creating teaching materials
- Sharing game positions on websites or in documents
- Analyzing historical games

## Features

- Fast, accurate SGF to PNG conversion using Rust's performance
- High-quality board and stone rendering with anti-aliasing
- Support for both square and rectangular boards (2x2 to 25x25)
- Full 19x19 board support with proper star points (hoshi)
- Multiple themes: dark, light, and paper
- Kifu mode for displaying move numbers
- Move number filtering to view game state at any point
- Accurate stone placement based on SGF coordinates
- Clean, minimal output focused on clarity

## Included Example Game

The repository includes a famous game from Go history: Game 4 of the historic 2016 match between AlphaGo and Lee Sedol. This game is often referred to as "Humanity's Last Stand" as it represents Lee Sedol's sole victory against the AI in their five-game match. This remarkable game (game_4.sgf) showcases:

- Lee Sedol's brilliant "Hand of God" move (move 78)
- A crucial victory for human ingenuity against artificial intelligence
- Complex fighting and strategic depth that challenged AlphaGo's understanding
- A historic moment in both Go and AI history

This game serves as both a test case for the renderer and a piece of Go history, demonstrating the library's ability to handle complex game records.

## Requirements

- Python 3.6+
- Rust (2021 edition)
- maturin (for building the Rust extension)

## Installation

1. Clone the repository
2. Build the Rust extension:
```bash
cd rust_sgf_renderer
maturin develop
```

## Usage

```python
from rust_sgf_renderer import render_sgf

# Read your SGF file
with open("game.sgf", "r", encoding="utf-8") as file:
    sgf_content = file.read()

# Basic rendering
render_sgf(sgf_content, "output.png")

# With theme and kifu mode (showing move numbers)
render_sgf(sgf_content, "output_kifu.png", theme="dark", kifu=True)

# View game state after specific move
render_sgf(sgf_content, "move_20.png", move_number=20)

# All options combined
render_sgf(sgf_content, "custom.png", theme="light", kifu=True, move_number=20)
```

## Technical Details

The renderer is implemented as a hybrid Python/Rust application, combining the best of both worlds:

- **Rust Core (`rust_sgf_renderer`):**
  - High-performance SGF parsing and rendering
  - PyO3 integration for seamless Python bindings
  - Skia graphics library for professional-quality output
  - Optimized stone placement and board drawing algorithms
  - Memory-efficient handling of game trees

- **Python Interface:**
  - Simple, intuitive API for file handling
  - Streamlined SGF input and PNG output
  - Easy integration with existing Python projects
  - Flexible file path handling

The board is rendered at 800x800 pixels with carefully calculated spacing and anti-aliasing for optimal visibility. The rendering includes:
- Precise grid lines with proper spacing
- Correctly positioned star points (hoshi) for 19x19 boards
- Anti-aliased stones with subtle shadows
- Clear black and white stone differentiation
- Automatic board scaling for different sizes
- Centered rendering for rectangular boards
- Optional move numbers in kifu mode

### Themes
- **dark**: Dark wooden board with glass stones (default)
- **light**: Light wooden board with glass stones
- **paper**: Simple black and white style for documents

### Board Sizes
- Supports square boards from 2x2 to 25x25
- Supports rectangular boards (e.g., 15:10)
- Automatically centers and scales the board

## Project Structure

```
.
├── rust_sgf_renderer/     # Rust library
│   ├── src/
│   │   └── lib.rs        # Core rendering logic
│   ├── Cargo.toml        # Rust dependencies
│   └── .gitignore
├── game_4.sgf           # Historic AlphaGo vs Lee Sedol Game 4
├── test_renderer.py     # Python usage example
└── README.md           # Documentation
```

## License

This project is available under the MIT License.

## Contributing

Contributions are welcome! Whether it's improving the rendering quality, adding new features, or fixing bugs, please feel free to submit pull requests.
