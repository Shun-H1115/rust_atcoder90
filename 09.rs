use proconio::input;
use std::f64::consts::PI;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug)]
struct Point {
    px: f64,
    py: f64,
}

impl std::ops::Sub for Point {
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point { px: self.px - other.px, py: self.py - other.py }
    }
}

fn get_angle(g: Point) -> f64 {
    // Calculate the polar angle of point G
    let angle = (g.px / (g.px * g.px + g.py * g.py).sqrt()).acos() * 180.0 / PI;
    if g.py >= 0.0 {
        angle
    } else {
        360.0 - angle
    }
}

fn get_angle_diff(i1: f64, i2: f64) -> f64 {
    // Calculate the smallest angle between two angles i1 and i2
    let res = (i1 - i2).abs();
    if res >= 180.0 {
        360.0 - res
    } else {
        res
    }
}

fn solve(pos: usize, points: &[Point]) -> f64 {
    // Calculate the maximum angle for a given point pos
    let mut angles = Vec::new();
    for (i, &p) in points.iter().enumerate() {
        if i == pos { continue; }
        let sa = p - points[pos];
        angles.push(get_angle(sa));
    }
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let mut max_angle: f64 = 0.0;
    for &angle in &angles {
        let target = if angle + 180.0 >= 360.0 {
            angle + 180.0 - 360.0
        } else {
            angle + 180.0
        };
        let pos1 = match angles.binary_search_by(|&x| x.partial_cmp(&target).unwrap_or(Ordering::Equal)) {
            Ok(p) | Err(p) => p % angles.len(),
        };
        let cand1 = get_angle_diff(angle, angles[pos1]);
        let cand2 = get_angle_diff(angle, angles[(pos1 + angles.len() - 1) % angles.len()]);
        max_angle = max_angle.max(cand1.max(cand2));
    }
    max_angle
}

fn solve_fast(points: &[Point]) -> f64 {
    let mut max_angle: f64 = 0.0;
    for i in 0..points.len() {
        max_angle = max_angle.max(solve(i, points));
    }
    max_angle
}

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        points: [(f64, f64); n],
    }
    let points: Vec<Point> = points.iter().map(|&(x, y)| Point { px: x, py: y }).collect();

    // Step #2. Output
    let final_answer = solve_fast(&points);
    println!("{:.12}", final_answer);
}
