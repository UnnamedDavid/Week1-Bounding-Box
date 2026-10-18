use serde::Deserialize;
use serde::Serialize;
use std::fs;

fn main() {
    let this_canvas = Canvas::from_json_file("src/canvas.json");

    let bbox_each = this_canvas.each_bounding_box();
    let bbox_total = this_canvas.overall_bounding_box();

    println!(
        "For each stroke on the canvas, the bounding box is {bbox_each:#?}, for the total canvas, the bounding box is {bbox_total:#?}"
    );
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Canvas {
    pub strokes: Vec<Stroke>,
}

impl Canvas {
    // From Json
    pub fn from_json_file(path: &str) -> Self {
        let content = fs::read_to_string(path).expect("Could not read json file");
        serde_json::from_str(&content).expect("Json file is not well-formatted")
    }

    // Find the bounding box that encloses all the strokes on the canvas
    fn overall_bounding_box(&self) -> ((f64, f64), (f64, f64)) {
        let mut total_min_x: f64 = f64::INFINITY;
        let mut total_min_y: f64 = f64::INFINITY;
        let mut total_max_x: f64 = f64::NEG_INFINITY;
        let mut total_max_y: f64 = f64::NEG_INFINITY;

        for stroke in &self.strokes {
            let ((min_x, min_y), (max_x, max_y)) = stroke.find_bounding_box();
            total_min_x = total_min_x.min(min_x);
            total_min_y = total_min_y.min(min_y);
            total_max_x = total_max_x.max(max_x);
            total_max_y = total_max_y.max(max_y);
        }

        ((total_min_x, total_min_y), (total_max_x, total_max_y))
    }

    // Find bounding box for each of the strokes on the canvas at once
    fn each_bounding_box(&self) -> Vec<((f64, f64), (f64, f64))> {
        let mut result: Vec<((f64, f64), (f64, f64))> = Vec::new();
        for stroke in &self.strokes {
            result.push(stroke.find_bounding_box());
        }
        result
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stroke {
    pub points: Vec<(f64, f64)>,
}

impl Stroke {
    // Contructor
    pub fn new(points: Vec<(f64, f64)>) -> Self {
        Self { points }
    }

    // Finds the bounding box of the stroke of line, that is, the smallest box
    // that encloses the stroke.
    fn find_bounding_box(&self) -> ((f64, f64), (f64, f64)) {
        let mut min_x: f64 = f64::INFINITY;
        let mut min_y: f64 = f64::INFINITY;
        let mut max_x: f64 = f64::NEG_INFINITY;
        let mut max_y: f64 = f64::NEG_INFINITY;
        for &(x, y) in &self.points {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
        ((min_x, min_y), (max_x, max_y))
    }
}
