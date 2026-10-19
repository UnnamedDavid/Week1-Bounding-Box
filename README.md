# Bounding box finder
This program finds the bounding box of a series of points (a stroke of line), it can find individual bounding box for each stroke, as well as the bounding box for the entire canvas, which is a vector that holds many strokes. Check the canvas.json for formatting your points and replace the existing canvas.json with your data to try it out.

This is useful in a drawing app such as a selection tool, where we compute if the selection has hit our bounding box, before calculating if it has hit the actual line (or stroke), which can be more expensive to compute as multiple coordinates are involved.

Usage:
```
git clone https://github.com/UnnamedDavid/Week1-Bounding-Box.git
cd Week-1-Bounding-Box
cargo run
```