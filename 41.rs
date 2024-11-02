use proconio::input;
use std::cmp::{min, max};

// Struct to represent a point with x and y coordinates
#[derive(Copy, Clone)]
struct Point {
    px: i64,
    py: i64,
}

// Operator overloading for addition and subtraction of Points
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point { px: self.px + other.px, py: self.py + other.py }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Point { px: self.px - other.px, py: self.py - other.py }
    }
}

// Cross product function to determine orientation
fn crs(p1: Point, p2: Point) -> i64 {
    p1.px * p2.py - p1.py * p2.px
}

// Function to calculate the greatest common divisor
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    // Step 1: Input
    input! {
        n: usize,
        points: [(i64, i64); n],
    }
    let mut g: Vec<Point> = points.iter().map(|&(px, py)| Point { px, py }).collect();
    g.sort_by(|a, b| if a.px == b.px { a.py.cmp(&b.py) } else { a.px.cmp(&b.px) });

    // Step 2: Calculate the convex hull using monotone chain method
    let mut g1 = vec![g[0], g[1]];
    let mut g2 = vec![g[0], g[1]];
    for i in 2..n {
        while g1.len() >= 2 && crs(g1[g1.len() - 1] - g1[g1.len() - 2], g[i] - g1[g1.len() - 1]) <= 0 {
            g1.pop();
        }
        while g2.len() >= 2 && crs(g2[g2.len() - 1] - g2[g2.len() - 2], g[i] - g2[g2.len() - 1]) >= 0 {
            g2.pop();
        }
        g1.push(g[i]);
        g2.push(g[i]);
    }

    // Combine g1 and g2 to form the full convex hull in Totsuhou
    let mut totsuhou = g1.clone();
    totsuhou.extend(g2.iter().rev().skip(1).take(g2.len() - 2));

    // Step 3: Calculate the number of integer lattice points on the polygon edges
    let mut edge_point = totsuhou.len() as i64;
    for i in 0..totsuhou.len() {
        let ax = totsuhou[i % totsuhou.len()].px;
        let ay = totsuhou[i % totsuhou.len()].py;
        let bx = totsuhou[(i + 1) % totsuhou.len()].px;
        let by = totsuhou[(i + 1) % totsuhou.len()].py;
        let vx = (bx - ax).abs();
        let vy = (by - ay).abs();
        edge_point += gcd(vx, vy) - 1;
    }

    // Step 4: Calculate the area using the shoelace formula (2 * area)
    let mut area = 0;
    for i in 0..totsuhou.len() {
        let ax = totsuhou[i % totsuhou.len()].px;
        let ay = totsuhou[i % totsuhou.len()].py;
        let bx = totsuhou[(i + 1) % totsuhou.len()].px;
        let by = totsuhou[(i + 1) % totsuhou.len()].py;
        area += (bx - ax) * (by + ay);
    }
    area = area.abs();

    // Step 5: Calculate the result using Pick's Theorem
    let answer = (area + edge_point + 2) / 2 - n as i64;
    println!("{}", answer);
}
