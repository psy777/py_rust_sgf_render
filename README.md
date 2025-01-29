# Python-Rust SGF Renderer

A high-performance SGF (Smart Game Format) renderer that converts Go/Baduk game files into PNG images. The project combines Python's ease of use with Rust's performance, using the Skia graphics library for high-quality rendering..

## Features

- Renders SGF files to PNG images
- High-performance Rust core with Python interface
- Full 19x19 board support with proper star points (hoshi)
- Black and white stone rendering with anti-aliasing
- Correct stone placement based on SGF coordinates

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

# Render to PNG
render_sgf(sgf_content, "output.png")
```

## Technical Details

The renderer is implemented as a hybrid Python/Rust application:

- **Rust Core (`rust_sgf_renderer`):**
  - Uses PyO3 for Python bindings
  - Implements SGF parsing and board rendering
  - Uses Skia for high-quality graphics
  - Handles stone placement and board drawing

- **Python Interface:**
  - Provides a simple API for file handling
  - Manages SGF input and PNG output

The board is rendered at 800x800 pixels with proper spacing and anti-aliasing for optimal visibility.

## Project Structure

```
.
├── rust_sgf_renderer/     # Rust library
│   ├── src/
│   │   └── lib.rs        # Core rendering logic
│   ├── Cargo.toml        # Rust dependencies
│   └── .gitignore
├── test_renderer.py      # Python usage example
└── README.md            # This file
```

## License

This project is available under the MIT License.
