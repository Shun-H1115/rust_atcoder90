use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    // Separate A and B just to compute maxima (or compute on the fly)
    let mut max_a = k;
    let mut max_b = k;
    for &(a, b) in &ab {
        if a > max_a { max_a = a; }
        if b > max_b { max_b = b; }
    }

    // Dimensions for 2D prefix sums (1-based board with extra 0-row/col)
    let r = max_a + 1;
    let c = max_b + 1;

    // sum has size (r+1) x (c+1); we'll use i32 since counts ≤ n
    let mut sum = vec![vec![0i32; c + 1]; r + 1];

    // Increment points at (a+1, b+1)
    for &(a, b) in &ab {
        sum[a + 1][b + 1] += 1;
    }

    // Prefix sums: first vertical, then horizontal (same as C++ code)
    for i in 1..=r {
        for j in 1..=c {
            sum[i][j] += sum[i - 1][j];
        }
    }
    for i in 1..=r {
        for j in 1..=c {
            sum[i][j] += sum[i][j - 1];
        }
    }

    // Slide KxK (inclusive) window:
    // Query rectangle via inclusion-exclusion:
    // S = sum[i][j] + sum[i+K+1][j+K+1] - sum[i][j+K+1] - sum[i+K+1][j]
    // with i in [0, r-K-1], j in [0, c-K-1]
    let mut answer: i32 = 0;
    if r >= k + 1 && c >= k + 1 {
        for i in 0..=r - k - 1 {
            let i2 = i + k + 1;
            for j in 0..=c - k - 1 {
                let j2 = j + k + 1;
                let val = sum[i][j] + sum[i2][j2] - sum[i][j2] - sum[i2][j];
                if val > answer {
                    answer = val;
                }
            }
        }
    }

    println!("{}", answer);
}