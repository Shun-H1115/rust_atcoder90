use proconio::input;
use std::f64::consts::PI;

fn query(t: f64, l: f64, x: f64, y: f64, e: f64) -> f64 {
    let cx = 0.0;
    let cy = -(l / 2.0) * (e / t * 2.0 * PI).sin();
    let cz = (l / 2.0) - (l / 2.0) * (e / t * 2.0 * PI).cos();
    let d1 = ((cx - x).powi(2) + (cy - y).powi(2)).sqrt();
    let d2 = cz;
    let angle = d2.atan2(d1);
    angle * 180.0 / PI
}

fn main() {
    // Step #1: Input
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        e_values: [f64; q],
    }

    // Step #2: Process each query and output the result
    for &e in &e_values {
        println!("{:.12}", query(t, l, x, y, e));
    }
}
