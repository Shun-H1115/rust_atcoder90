use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let target = "atcoder";
    let mut dp = vec![0; 8];
    dp[0] = 1;

    for ch in s.chars() {
        for j in (0..7).rev() {
            if ch == target.chars().nth(j).unwrap() {
                dp[j + 1] = (dp[j + 1] + dp[j]) % MOD;
            }
        }
    }

    // Output the answer
    println!("{}", dp[7]);
}