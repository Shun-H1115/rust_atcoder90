use proconio::input;
use std::cmp::min;
use std::collections::VecDeque;

struct BIT {
    size: usize,
    bit: Vec<i64>,
}

impl BIT {
    fn new(sz: usize) -> Self {
        BIT {
            size: sz + 2,
            bit: vec![0; sz + 3],
        }
    }

    fn add(&mut self, pos: usize, x: i64) {
        let mut pos = pos + 1;
        while pos <= self.size {
            self.bit[pos] += x;
            pos += pos & (!pos + 1);
        }
    }

    fn sum(&self, pos: usize) -> i64 {
        let mut s = 0;
        let mut pos = pos + 1;
        while pos >= 1 {
            s += self.bit[pos];
            pos -= pos & (!pos + 1);
        }
        s
    }
}

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); m],
    }

    // Initialize vectors for calculations
    let mut v1 = vec![0; n + 1];
    let mut v2 = vec![0; n + 1];
    let mut v3 = vec![0; n + 1];

    // Step #2. Get Answer1
    let mut answer1 = 0;
    for &(l, r) in &lr {
        v3[l] += 1;
        v3[r] += 1;
    }
    for i in 1..=n {
        answer1 += v3[i] * (v3[i] - 1) / 2;
    }

    // Step #3. Get Answer2
    let mut answer2 = 0;
    for &(_, r) in &lr {
        v1[r] += 1;
    }
    for &(l, _) in &lr {
        if l > 1 {
            v2[l - 1] += 1;
        }
    }
    for i in 1..=n {
        v1[i] += v1[i - 1];
        answer2 += v1[i] * v2[i];
    }

    // Step #4. Sorting
    let mut intervals: Vec<_> = lr.iter().map(|&(l, r)| (r, l)).collect();
    intervals.sort();

    // Step #5. Get Answer3
    let mut answer3 = 0;
    let mut bit = BIT::new(n);
    for &(cr, cl) in &intervals {
        let ret = bit.sum(cr) - bit.sum(cl);
        answer3 += ret;
        bit.add(cl, 1);
    }

    // Step #6. Output The Answer!
    let total = m as i64 * (m as i64 - 1) / 2;
    let sum_ans = answer1 + answer2 + answer3;
    println!("{}", total - sum_ans);
}
