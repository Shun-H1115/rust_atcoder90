use proconio::input;
const MOD: i32 = 1_000_000_007;

fn main() {
    input! {
        k: usize,
    }

    if k % 9 == 0 {
        let mut dp = vec![0; k + 1];
        dp[0] = 1;

        for i in 1..=k {
            for j in (i.saturating_sub(9))..i {
                dp[i] = (dp[i] + dp[j]) % MOD;
            }
        }

        println!("{}", dp[k]);
    } else {
        println!("0");
    }
}
