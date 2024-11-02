use proconio::input;
use std::collections::VecDeque;
use std::io::{self, Write};

fn main() {
    // Step 1: Input
    input! {
        n: usize,
        ab: [(usize, usize); n],
    }

    let mut usable = vec![false; n + 1];
    let mut g = vec![vec![]; n + 1];
    let mut q = VecDeque::new();

    for (i, (a, b)) in ab.iter().enumerate() {
        let idx = i + 1;
        g[*a].push(idx);
        g[*b].push(idx);

        // Mark nodes directly usable
        if *a == idx || *b == idx {
            usable[idx] = true;
            q.push_back(idx);
        }
    }

    // Step 2: Process nodes in a queue
    let mut vec = Vec::new();
    while let Some(pos) = q.pop_front() {
        vec.push(pos);
        for &i in &g[pos] {
            if !usable[i] {
                usable[i] = true;
                q.push_back(i);
            }
        }
    }

    // Step 3: Output result
    vec.reverse();
    let stdout = io::stdout();
    let mut handle = io::BufWriter::new(stdout.lock());

    if vec.len() != n {
        writeln!(handle, "-1").unwrap();
    } else {
        for v in vec {
            writeln!(handle, "{}", v).unwrap();
        }
    }
}
