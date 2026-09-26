// ==========================================================================
// 典型90 #069 Colorful Blocks 2（★3） 修正版
// 【修正点】i64 オーバーフローを解消（N, K は最大 1e18 程度）
//   元: k * (k - 1) % MOD         … k が 1e18 なら積は 1e36 で i64 を超える
//   元: binpower 内の a * a % MOD  … 引数 a = k - 2 が MOD 以上のまま 2乗される
//   新: 掛け算の前に必ず % MOD を取る（Python では多倍長なので不要だった配慮）
// ==========================================================================

use proconio::input;

const MOD: i64 = 1_000_000_007;

// a^b mod MOD（繰り返し二乗法）
fn binpower(mut a: i64, mut b: i64) -> i64 {
    // [修正] 最初に a を MOD 未満にしておく → 以降の a * a は 1e18 未満で安全
    a %= MOD;
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
    input! {
        n: i64,
        k: i64,
    }

    // [修正] 掛ける前に MOD で割った余りにしておく
    let k0 = k % MOD;
    let k1 = (k - 1) % MOD;

    let result = if k == 1 {
        if n == 1 { 1 } else { 0 }
    } else if n == 1 {
        k0
    } else if n == 2 {
        k0 * k1 % MOD
    } else {
        // 1個目 K 通り、2個目 K-1 通り、3個目以降は K-2 通りずつ
        k0 * k1 % MOD * binpower(k - 2, n - 2) % MOD
    };

    println!("{}", result);
}
