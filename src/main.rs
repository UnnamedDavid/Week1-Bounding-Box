use serde::Deserialize;
use serde::Serialize;
use std::fs;

fn main() {
    let this_canvas = Canvas::from_json_file("src/canvas.json");

    // let bbox = this_canvas.find_bounding_box();
    // println!("{bbox:#?}")
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

    // TO DO
    fn total_bounding_box(&self) {}
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
