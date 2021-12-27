extern crate nalgebra as na;
use na::{matrix, vector};

fn main() {
    let s = vector![1.0, 1.0];
    let sin: f64 = s[0]/s.norm();
    let cos: f64 = s[1]/s.norm();
    let polygon = matrix![0.0, 0.0; 0.0, 1.0; 1.0, 1.0; 1.0, 0.0];
    dbg!(sin, cos, polygon);
    let rot_matrix = matrix![cos, -sin; sin, cos];
    let rotated_polygon = polygon*rot_matrix;
    dbg!(rotated_polygon.column(1)[0]);
    dbg!((0..polygon.len()/2).collect::<Vec<usize>>().sort_by(|a, b| rotated_polygon.column(1)[a].cmp(rotated_polygon.column(1)[b])));
}
