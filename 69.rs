use proconio::input;
const MOD: i64 = 1_000_000_007;

// Function to compute a^b % mod using binary exponentiation
fn binpower(mut a: i64, mut b: i64) -> i64 {
    let mut ans = 1;
    while b != 0 {
        if b % 2 == 1 {
            ans = ans * a % MOD;
        }
        a = a * a % MOD;
        b /= 2;
    }
    ans
}

fn main() {
    // Input: Read N (long long) and K (int)
    input! {
        n: i64,
        k: i64,
    }

    // Handling cases based on values of N and K
    let result = if k == 1 {
        if n == 1 { 1 } else { 0 }
    } else if n == 1 {
        k % MOD
    } else if n == 2 {
        (k * (k - 1) % MOD) as i64
    } else {
        // General case using the binpower function
        (k * (k - 1) % MOD * binpower(k - 2, n - 2) % MOD) as i64
    };

    // Output the result
    println!("{}", result);
}
