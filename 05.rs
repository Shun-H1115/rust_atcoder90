use proconio::input;

const MOD: i64 = 1_000_000_007;

fn modpow(a: i64, b: i64, m: i64) -> i64 {
    let mut p = 1;
    let mut q = a;
    for i in 0..63 {
        if (b & (1 << i)) != 0 {
            p = p * q % m;
        }
        q = q * q % m;
    }
    p
}

fn main() {
    input! {
        n: i64,   // N
        b: usize, // B
        k: usize, // K
        c: [usize; k], // C array
    }

    // Step #2: Precompute powers of 10 modulo B
    let mut power10 = vec![0; 64];
    for i in 0..=62 {
        power10[i] = modpow(10, 1 << i, b as i64);
    }

    // Step #3: Initialize DP with the base cases for dp[1][i]
    let mut dp = vec![vec![0; b]; 64];
    for &digit in &c {
        dp[0][digit % b] += 1;
    }

    // Step #4: Compute dp[1][i], dp[2][i], ..., dp[2^n][i]
    for i in 0..62 {
        for j in 0..b {
            for k in 0..b {
                let nex = (j * power10[i] as usize + k) % b;
                dp[i + 1][nex] = (dp[i + 1][nex] + dp[i][j] * dp[i][k]) % MOD;
            }
        }
    }

    // Step #5: Use exponentiation by squaring to compute dp[N][i]
    let mut answer = vec![vec![0; b]; 64];
    answer[0][0] = 1;
    for i in 0..62 {
        if (n & (1 << i)) != 0 {
            let mut new_answer = vec![0; b];
            for j in 0..b {
                for k in 0..b {
                    let nex = (j * power10[i] as usize + k) % b;
                    new_answer[nex] = (new_answer[nex] + answer[i][j] * dp[i][k]) % MOD;
                }
            }
            answer[i + 1] = new_answer;
        } else {
            answer[i + 1] = answer[i].clone();
        }
    }

    // Step #6: Output the result
    println!("{}", answer[62][0]);
}
