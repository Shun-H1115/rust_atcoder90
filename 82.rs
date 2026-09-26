// ==========================================================================
// 典型90 #082  Counting Numbers  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_cd
// ==========================================================================
// 【アルゴリズム】 桁数ごとに区切って等差数列の和（桁数 i の区間 [10^(i-1), 10^i - 1] と [L, R] の共通部分）
// 【計算量】       O(19)
// 【学習計画】     第3週（★3後半）
//
// 【このファイルで覚えるRust文法】
//   - u128 … L, R が 10^18 級で、x + 1 や 10^19 の計算で u64 に不安があるため
//   - (x % (MOD as u128)) as i64 … 大きい型で余りを取ってから小さい型へ（Py では不要な配慮）
//   - [0u128; 20] … 固定長配列（Vec でなく配列。サイズがコンパイル時に決まる）
//   - rem_euclid(MOD) … 負にならない余り（f(vr) - f(vl-1) が負になりうるため）
//   - mod_inv で「÷2」を逆元の掛け算に置き換える
//
// 【Pythonで書くと（考え方の対応）】
//   ans = 0
//   for i in range(1, 20):
//       lo, hi = max(L, 10**(i-1)), min(R, 10**i - 1)
//       if lo > hi: continue
//       ans += i * (hi*(hi+1)//2 - (lo-1)*lo//2)
//   print(ans % MOD)    # Py は多倍長なので途中で MOD を取らなくても正しい
// ==========================================================================

use proconio::input;

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    let mut r = 1i64;
    // 最初に a %= MOD しておくのが安全な繰り返し二乗法（#069 の注意と比較）。
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
// u128 で受けて、MOD で割った余りにしてから i64 に落とす。
fn f(x: u128) -> i64 {
    let v1 = (x % (MOD as u128)) as i64;
    let v2 = ((x + 1) % (MOD as u128)) as i64;
    // x(x+1)/2 の /2 は、MOD の世界では 2 の逆元を掛ける。
    div_mod((v1 * v2) % MOD, 2)
}

fn main() {
    input! {
        l: u128,
        r: u128,
    }

    // power10[i] = 10^i (u128)
    // 固定長配列 [型; 長さ]。Vec と違い伸び縮みしない。10^19 は u64 の上限付近なので u128。
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
        // 引き算の結果は負になりうるので rem_euclid で 0 以上に。
        let val = (f(vr) - f(vl - 1)).rem_euclid(MOD);
        // i as i64 … usize のループ変数を i64 に揃えて掛ける。
        ans = (ans + (i as i64) * val) % MOD;
    }

    println!("{}", ans.rem_euclid(MOD));
}
