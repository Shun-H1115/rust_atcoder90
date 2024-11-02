use proconio::input;

fn main() {
    const MOD: i32 = 1_000_000_007;
    
    // Step #1: Input
    input! {
        n: usize,
        l: usize,
    }

    // Step #2: Dynamic Programming Initialization
    let mut dp = vec![0; n + 1];
    dp[0] = 1;

    for i in 1..=n {
        if i < l {
            dp[i] = dp[i - 1];
        } else {
            dp[i] = (dp[i - 1] + dp[i - l]) % MOD;
        }
    }

    // Step #3: Output the result
    println!("{}", dp[n]);
}
