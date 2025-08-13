use proconio::input;

fn main() {
    input! {
        n: usize,
        a_in: [i64; 2 * n],
    }

    let m = 2 * n; // total number of elements
    // 1-based indexing for simplicity (index 0 unused)
    let mut a = vec![0_i64; m + 1];
    for (i, v) in a_in.into_iter().enumerate() {
        a[i + 1] = v;
    }

    let inf: i64 = 1_i64 << 60;
    // dp size: (m + 2) x (m + 2) to safely access l+1 and r-1
    let mut dp = vec![vec![inf; m + 2]; m + 2];

    // Base cases: segments of length 2
    for i in 1..m {
        dp[i][i + 1] = (a[i] - a[i + 1]).abs();
    }

    // Consider segments where (r - l + 1) is even and >= 4
    for len in (3..=m).step_by(2) {
        for l in 1..=m - len {
            let r = l + len;

            // Case 1: split at k
            for k in l..r {
                let candidate = dp[l][k] + dp[k + 1][r];
                if candidate < dp[l][r] {
                    dp[l][r] = candidate;
                }
            }
            // Case 2: pair ends (l with r)
            let ends = dp[l + 1][r - 1] + (a[l] - a[r]).abs();
            if ends < dp[l][r] {
                dp[l][r] = ends;
            }
        }
    }

    println!("{}", dp[1][m]);
}