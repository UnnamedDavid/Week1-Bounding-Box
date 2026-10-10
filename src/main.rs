mod data;
fn main() {
    let bbox = find_bounding_box(&data::STROKE);
    println!("{bbox:#?}")
}

// Finds the bounding box of the stroke of line, that is, the smallest box
// that encloses the stroke.
fn find_bounding_box(stroke: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    let mut min_x: f64 = f64::INFINITY;
    let mut min_y: f64 = f64::INFINITY;
    let mut max_x: f64 = f64::NEG_INFINITY;
    let mut max_y: f64 = f64::NEG_INFINITY;
    for &(x, y) in stroke {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }
    ((min_x, min_y), (max_x, max_y))
}
