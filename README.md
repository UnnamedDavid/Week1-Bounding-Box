# Bounding Box Finder

Calculates individual and overall bounding boxes for 2D strokes (lists of `(x, y)` coordinates) on a canvas. Useful in drawing apps for fast hit-testing and selection checks before performing expensive per-point calculations.

## Prerequisites
* [Rust & Cargo](https://rustup.rs/)

## Build & Usage

Simply extract the zip file and keep the generated executable together with the `canvas.json` file in the same folder. The program reads the JSON file and calculates the bounding box for each stroke and for the whole canvas. How to edit the json file to calculate your own strokes and points is explained below.

### Build from source

1. Clone the repository:
   ```bash
   git clone https://github.com/UnnamedDavid/Week1-Bounding-Box.git
   cd Week1-Bounding-Box
   ```
2. Edit `canvas.json` in the project root directory with your stroke coordinates.

   The file is expected to look like this:
   ```json
   {
     "strokes": [
       { "points": [[10.5, 100.2], [12.0, 102.5], [18.7, 95.1]] },
       { "points": [[30.0, 40.0], [40.0, 48.5], [52.1, 58.2]] }
     ]
   }
   ```

   What this means:
   - `strokes` is the top-level array containing one object per stroke.
   - Each stroke has a `points` array.
   - Each point is a 2D coordinate written as `[x, y]`.
   - You can add more strokes by inserting more objects into the `strokes` array.
   - You can add more points by appending more `[x, y]` pairs inside each stroke.
   - The numbers can be integers or decimals, such as `10`, `12.5`, or `99.75`.

   Example with a single stroke and a few points:
   ```json
   {
     "strokes": [
       { "points": [[0, 0], [50, 10], [80, 40], [30, 60]] }
     ]
   }
   ```

   If you want to use a different file name, you can pass it as a command-line argument:
   ```bash
   cargo run --release -- my_canvas.json
   ```

3. Build and run from source:
   ```bash
   cargo run --release
   ```

To build a standalone binary for your platform without running it immediately, run:
```bash
cargo build --release
```

The executable will be generated in `target/release/`.