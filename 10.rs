use proconio::input;

fn main() {
    // Step #1: Input
    input! {
        n: usize,
        cp: [(usize, i32); n],  // Each entry contains C[i] and P[i]
        q: usize,
        lr: [(usize, usize); q],  // Each query contains L[i] and R[i]
    }

    // Step #2: Initialize cumulative sums for class 1 and class 2
    let mut sum1 = vec![0; n + 1];
    let mut sum2 = vec![0; n + 1];
    
    for i in 1..=n {
        let (c, p) = cp[i - 1];
        sum1[i] = sum1[i - 1];
        sum2[i] = sum2[i - 1];
        if c == 1 {
            sum1[i] += p;
        } else if c == 2 {
            sum2[i] += p;
        }
    }

    // Step #3: Process each query and output the result
    for &(l, r) in &lr {
        let answer1 = sum1[r] - sum1[l - 1];
        let answer2 = sum2[r] - sum2[l - 1];
        println!("{} {}", answer1, answer2);
    }
}
