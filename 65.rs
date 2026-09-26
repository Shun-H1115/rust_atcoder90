// ==========================================================================
// 典型90 #065  RGB Balls 2  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bm
// ==========================================================================
// 【アルゴリズム】 組合せ＋畳み込み（NTT）。赤と緑の選び方の多項式を掛け、青と組み合わせて係数を足す
// 【計算量】       O(N log N)
// 【学習計画】     第8週以降（★7。FFT/NTT は2か月の範囲外。ac-library の使い方の例として読む）
//
// 【このファイルで覚えるRust文法】
//   - use ac_library::{convolution, ModInt998244353}; … AtCoder Library の Rust 版（ac-library-rs）
//   - type ModInt = ModInt998244353; … 型エイリアス（Py の型ヒント別名に近い）
//   - ModInt は +, *, += が MOD 付きで定義済み → 毎回 % MOD を書かなくてよい
//   - (0..=r).map(|i| ncr(r, i, &fact, &inv)).collect() … Py: [ncr(r, i) for i in range(r+1)]
//   - saturating_sub … 0 未満にならない引き算
//
// 【Pythonで書くと（考え方の対応）】
//   # f(x) = Σ_{i=r_left}^{min(R,K)} C(R, i) x^i,  g(x) = Σ C(G, j) x^j
//   # h = f * g （畳み込み）
//   # ans = Σ_{i} C(B, i) * h[K - i]
//
// 【注意・改善ポイント】
//   ! StaticModInt は未使用（warning）。
//   ! 階乗の逆元を全部 modpow で求めているので前計算が O(MAX log MOD)。#015 の注意と同じく後ろから作れば O(MAX)。
// ==========================================================================

// ac-library-rs。AtCoder のジャッジで使えるクレート（ローカルでは Cargo.toml に追加が必要）。
use ac_library::{convolution, StaticModInt, ModInt998244353};
use proconio::input;
const MOD: i64 = 998244353;
const MAX: usize = 500000;
// 型エイリアス。長い型名に短い別名を付ける。
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

// &[i64] で階乗テーブルを借用して受け取る（グローバル変数を避けた良い設計。#015 と比較してみよう）。
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
    // C(r, i) の配列を内包表記風に作る。
    let ar: Vec<i64> = (0..=r).map(|i| ncr(r, i, &fact, &inv)).collect();
    let ag: Vec<i64> = (0..=g).map(|i| ncr(g, i, &fact, &inv)).collect();
    let ab: Vec<i64> = (0..=b).map(|i| ncr(b, i, &fact, &inv)).collect();

    // Prepare bounds based on given constraints
    // 赤は K - Y 個以上必要（緑+青が Y 以下の条件）。saturating_sub で負なら 0。
    let r_left = k.saturating_sub(y);
    let g_left = k.saturating_sub(z);
    let b_left = k.saturating_sub(x);

    // Convert p1 and p2 into ModInt
    // ModInt::new(0) を r+1 個。vec! は Clone な型なら何でも初期化できる。
    let mut p1: Vec<ModInt> = vec![ModInt::new(0); r + 1];
    let mut p2: Vec<ModInt> = vec![ModInt::new(0); g + 1];
    for i in r_left..=r.min(k) {
        p1[i] = ModInt::new(ar[i]);
    }
    for i in g_left..=g.min(k) {
        p2[i] = ModInt::new(ag[i]);
    }

    // Perform FFT convolution
    // convolution … NTT による多項式の積。O(N log N)。
    let p3 = convolution(&p1, &p2);

    // Calculate the final answer by iterating through valid B values
    let mut answer = ModInt::new(0);
    for i in b_left..=b.min(k) {
        // k - i が p3 の範囲内か確認してから添字アクセス（範囲外は panic するため）。
        let ret1 = if k >= i && k - i < p3.len() { p3[k - i] } else { ModInt::new(0) };
        // ModInt 同士の * と += は自動で MOD を取る。
        answer += ret1 * ModInt::new(ab[i]);
    }
    // ModInt は Display を実装しているので {} でそのまま表示できる。
    println!("{}", answer);
}
