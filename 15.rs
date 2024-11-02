use proconio::input;

const MOD: i64 = 1_000_000_007;
const MAX: usize = 200_000;

fn modpow(a: i64, b: i64, m: i64) -> i64 {
    // Calculate a^b mod m
    let mut p = 1;
    let mut q = a;
    for i in 0..30 {
        if (b & (1 << i)) != 0 {
            p = p * q % m;
        }
        q = q * q % m;
    }
    p
}

fn divmod(a: i64, b: i64, m: i64) -> i64 {
    // Calculate a / b mod m using modular inverse
    (a * modpow(b, m - 2, m)) % m
}

// Arrays for factorials and modular inverses of factorials
static mut FACT: [i64; MAX + 1] = [0; MAX + 1];
static mut FACTINV: [i64; MAX + 1] = [0; MAX + 1];

fn init() {
    unsafe {
        FACT[0] = 1;
        for i in 1..=MAX {
            FACT[i] = FACT[i - 1] * i as i64 % MOD;
        }
        for i in 0..=MAX {
            FACTINV[i] = divmod(1, FACT[i], MOD);
        }
    }
}

fn ncr(n: i32, r: i32) -> i64 {
    if n < r || r < 0 {
        0
    } else {
        unsafe { FACT[n as usize] * FACTINV[r as usize] % MOD * FACTINV[(n - r) as usize] % MOD }
    }
}

fn query(n: i32, k: i32) -> i64 {
    let mut ret = 0;
    // Calculate all valid combinations for given `k`
    for i in 1..=(n / k + 1) {
        let s1 = n - (k - 1) * (i - 1);
        let s2 = i;
        ret = (ret + ncr(s1, s2)) % MOD;
    }
    ret
}

fn main() {
    // Step #1: Input
    input! {
        n: i32,
    }

    // Step #2: Initialize factorials and modular inverses
    init();

    // Step #3: Output results for each k from 1 to N
    for k in 1..=n {
        let answer = query(n, k);
        println!("{}", answer);
    }
}
