# Bounding Box Finder

Calculates individual and overall bounding boxes for 2D strokes (lists of `(x, y)` coordinates) on a canvas. Useful in drawing apps for fast hit-testing and selection checks before performing expensive per-point calculations.

## Prerequisites
* [Rust & Cargo](https://rustup.rs/)

## Build & Usage

1. Clone the repository:
   ```bash
   git clone [https://github.com/UnnamedDavid/Week1-Bounding-Box.git](https://github.com/UnnamedDavid/Week1-Bounding-Box.git)
   cd Week1-Bounding-Box
   ```
2. Edit canvas.json in the root directory with your stroke coordinates:
    ```json
    {
    "strokes": [
        { "points": [[10.5, 100.2], [12.0, 102.5]] },
        { "points": [[30.0, 40.0], [40.0, 48.5]] }
    ]
    }
    ```
3. Build and run from source:
    ```bash
    cargo run --release
    ```
(Or pass a custom JSON file: ```cargo run --release -- my_canvas.json```)

To build a standalone binary for your platform without running it immediately, run ```cargo build --release```. The executable will be generated in ```target/release/```.