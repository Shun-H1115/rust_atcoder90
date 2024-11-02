use ac_library::{convolution, StaticModInt, ModInt998244353};
use proconio::input;
const MOD: i64 = 998244353;
const MAX: usize = 500000;
type ModInt = ModInt998244353;

fn modpow(a: i64, b: i64, m: i64) -> i64 {
    let mut p = 1;
    let mut q = a;
    for i in 0..32 {
        if (b & (1 << i)) != 0 {
            p = p * q % m;
        }
        q = q * q % m;
    }
    p
}

fn div(a: i64, b: i64, m: i64) -> i64 {
    (a * modpow(b, m - 2, m)) % m
}

fn ncr(n: usize, r: usize, fact: &[i64], inv: &[i64]) -> i64 {
    if n < r {
        return 0;
    }
    fact[n] * inv[r] % MOD * inv[n - r] % MOD
}

fn main() {
    // Initialize factorials and modular inverses for nCr calculations
    let mut fact = vec![1; MAX + 1];
    let mut inv = vec![1; MAX + 1];
    for i in 1..=MAX {
        fact[i] = fact[i - 1] * i as i64 % MOD;
    }
    for i in 0..=MAX {
        inv[i] = div(1, fact[i], MOD);
    }

    // Read inputs
    input! {
        r: usize,
        g: usize,
        b: usize,
        k: usize,
        x: usize,
        y: usize,
        z: usize,
    }

    // Create arrays for nCr values of each color
    let ar: Vec<i64> = (0..=r).map(|i| ncr(r, i, &fact, &inv)).collect();
    let ag: Vec<i64> = (0..=g).map(|i| ncr(g, i, &fact, &inv)).collect();
    let ab: Vec<i64> = (0..=b).map(|i| ncr(b, i, &fact, &inv)).collect();

    // Prepare bounds based on given constraints
    let r_left = k.saturating_sub(y);
    let g_left = k.saturating_sub(z);
    let b_left = k.saturating_sub(x);

    // Convert p1 and p2 into ModInt
    let mut p1: Vec<ModInt> = vec![ModInt::new(0); r + 1];
    let mut p2: Vec<ModInt> = vec![ModInt::new(0); g + 1];
    for i in r_left..=r.min(k) {
        p1[i] = ModInt::new(ar[i]);
    }
    for i in g_left..=g.min(k) {
        p2[i] = ModInt::new(ag[i]);
    }

    // Perform FFT convolution
    let p3 = convolution(&p1, &p2);

    // Calculate the final answer by iterating through valid B values
    let mut answer = ModInt::new(0);
    for i in b_left..=b.min(k) {
        let ret1 = if k >= i && k - i < p3.len() { p3[k - i] } else { ModInt::new(0) };
        answer += ret1 * ModInt::new(ab[i]);
    }
    println!("{}", answer);
}
