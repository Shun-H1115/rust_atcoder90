use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..=s {
            if j >= a && dp[i][j - a] {
                dp[i + 1][j] = true;
            }
            if j >= b && dp[i][j - b] {
                dp[i + 1][j] = true;
            }
        }
    }

    if !dp[n][s] {
        println!("Impossible");
    } else {
        let mut res = vec!['?'; n];
        let mut pos = s;
        for i in (0..n).rev() {
            let (a, b) = ab[i];
            if pos >= b && dp[i][pos - b] {
                res[i] = 'B';
                pos -= b;
            } else {
                res[i] = 'A';
                pos -= a;
            }
        }
        let result: String = res.iter().collect();
        println!("{}", result);
    }
}
