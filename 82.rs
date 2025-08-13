use proconio::input;

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    let mut r = 1i64;
    a %= MOD;
    while e > 0 {
        if (e & 1) != 0 {
            r = (r * a) % MOD;
        }
        a = (a * a) % MOD;
        e >>= 1;
    }
    r
}

fn mod_inv(a: i64) -> i64 {
    mod_pow(a, MOD - 2)
}

fn div_mod(a: i64, b: i64) -> i64 {
    (a * mod_inv(b)) % MOD
}

// f(X) = 1 + 2 + ... + X (mod MOD)
fn f(x: u128) -> i64 {
    let v1 = (x % (MOD as u128)) as i64;
    let v2 = ((x + 1) % (MOD as u128)) as i64;
    div_mod((v1 * v2) % MOD, 2)
}

fn main() {
    input! {
        l: u128,
        r: u128,
    }

    // power10[i] = 10^i (u128)
    let mut power10 = [0u128; 20];
    power10[0] = 1;
    for i in 1..=19 {
        power10[i] = power10[i - 1] * 10;
    }

    // Sum over digit lengths i = 1..19 of i * sum_{x in [vl..vr]} x
    let mut ans: i64 = 0;
    for i in 1..=19 {
        let vl = l.max(power10[i - 1]);
        let vr = r.min(power10[i] - 1);
        if vl > vr {
            continue;
        }
        let val = (f(vr) - f(vl - 1)).rem_euclid(MOD);
        ans = (ans + (i as i64) * val) % MOD;
    }

    println!("{}", ans.rem_euclid(MOD));
}