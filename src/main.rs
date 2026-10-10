mod data;
fn main() {
    let test_stroke = Stroke {
        points: vec![
            (10.5, 100.2),
            (12.0, 102.5),
            (15.2, 107.0),
            (22.1, 115.4),
            (35.0, 120.0),
            (48.3, 119.2),
            (50.1, 110.0),
            (49.0, 95.5),
        ],
    };

    let bbox = test_stroke.find_bounding_box();
    println!("{bbox:#?}")
}

pub struct Stroke {
    points: Vec<(f64, f64)>,
}

impl Stroke {
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
