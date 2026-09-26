// ==========================================================================
// 典型90 #015  Don't be too close  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_o
// ==========================================================================
// 【アルゴリズム】 組合せ（nCr）＋調和級数（各 k について選ぶ個数 i を全探索、Σ N/k = O(N log N)）
// 【計算量】       O(N log N)（前計算 O(N log MOD)）
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - static mut と unsafe … グローバルな可変配列。Rust では危険操作として unsafe ブロックが必須
//   - （推奨）グローバル変数ではなく struct や Vec を作って引数で渡すのが Rust らしい書き方
//   - 型を混ぜないために i as i64 などの明示変換
//   - for i in 1..=(n / k + 1) … 範囲の上限は式でもよい
//
// 【Pythonで書くと（考え方の対応）】
//   fact = [1]*(MAX+1)
//   for i in range(1, MAX+1): fact[i] = fact[i-1]*i % MOD
//   inv = [pow(f, MOD-2, MOD) for f in fact]
//   def ncr(n, r): return 0 if r < 0 or n < r else fact[n]*inv[r]%MOD*inv[n-r]%MOD
//   for k in range(1, N+1):
//       print(sum(ncr(N - (k-1)*(i-1), i) for i in range(1, N//k + 2)) % MOD)
//
// 【注意・改善ポイント】
//   ! static mut は Rust 2024 edition では参照を取るとエラー/警告になる方向。AtCoder(1.70)では動くが、
//     次のように書き換えるのがおすすめ:
//       struct Comb { fact: Vec<i64>, inv: Vec<i64> }
//       impl Comb { fn new(n: usize) -> Self {...} fn ncr(&self, n: usize, r: usize) -> i64 {...} }
//     → #056 以降の組合せ問題でもそのまま使い回せるライブラリになる。
//   ! 逆元を全部 modpow で求めると O(N log MOD)。fact_inv[MAX] だけ modpow で求め、
//     fact_inv[i-1] = fact_inv[i] * i で後ろから作ると O(N) になる（定番テク）。
// ==========================================================================

use proconio::input;

const MOD: i64 = 1_000_000_007;
const MAX: usize = 200_000;

fn modpow(a: i64, b: i64, m: i64) -> i64 {
    // Calculate a^b mod m
    let mut p = 1;
    let mut q = a;
    // 指数 b は MOD-2 < 2^30 なので 30 ビットで十分。
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
    // フェルマーの小定理: b^(MOD-2) が b の逆元（MOD が素数のとき）。
    (a * modpow(b, m - 2, m)) % m
}

// Arrays for factorials and modular inverses of factorials
// static mut … プログラム全体で共有される可変配列。読み書きは unsafe でしか許されない。
static mut FACT: [i64; MAX + 1] = [0; MAX + 1];
static mut FACTINV: [i64; MAX + 1] = [0; MAX + 1];

fn init() {
    // unsafe { } … 「データ競合が起きないことは自分が保証する」という宣言。競プロでは避けるのが無難。
    unsafe {
        FACT[0] = 1;
        for i in 1..=MAX {
            // i as i64 … usize の i を i64 に揃えてから掛ける。FACT[i-1] < MOD なので積は 2e14 程度で安全。
            FACT[i] = FACT[i - 1] * i as i64 % MOD;
        }
        for i in 0..=MAX {
            FACTINV[i] = divmod(1, FACT[i], MOD);
        }
    }
}

// i32 で受けているのは引き算で負になりうるため（n < r のチェックを負値で行う）。
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
    // k 間隔で i 個置けるのは (k-1)(i-1) <= N - i のとき。上限は N/k + 1 程度で打ち切れる。
    for i in 1..=(n / k + 1) {
        // 隙間を詰めると「N - (k-1)(i-1) 個から i 個選ぶ」に帰着する（典型の言い換え）。
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
