use std::cmp::{max, min};
use std::io;

fn main() {
    let mut input = String::new();
    
    // Step #1. Input
    io::stdin().read_line(&mut input).unwrap();
    let mut input_iter = input.split_whitespace();
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let q: usize = input_iter.next().unwrap().parse().unwrap();

    let mut x = vec![0; n + 1];
    let mut y = vec![0; n + 1];
    let mut t = vec![0; q + 1];

    for i in 1..=n {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut xy_iter = input.split_whitespace();
        x[i] = xy_iter.next().unwrap().parse().unwrap();
        y[i] = xy_iter.next().unwrap().parse().unwrap();
    }

    for i in 1..=q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        t[i] = input.trim().parse().unwrap();
    }

    // Step #2. Rotate by 45 degrees
    let mut min_x = 1i64 << 60;
    let mut max_x = -(1i64 << 60);
    let mut min_y = 1i64 << 60;
    let mut max_y = -(1i64 << 60);

    for i in 1..=n {
        let p1 = x[i] + y[i];
        let p2 = y[i] - x[i];
        x[i] = p1;
        y[i] = p2;

        min_x = min(min_x, x[i]);
        max_x = max(max_x, x[i]);
        min_y = min(min_y, y[i]);
        max_y = max(max_y, y[i]);
    }

    // Step #3. Answer Queries
    for i in 1..=q {
        let idx = t[i];
        let ret1 = (x[idx] - min_x).abs();
        let ret2 = (x[idx] - max_x).abs();
        let ret3 = (y[idx] - min_y).abs();
        let ret4 = (y[idx] - max_y).abs();

        let answer = max(max(ret1, ret2), max(ret3, ret4));
        println!("{}", answer);
    }
}
