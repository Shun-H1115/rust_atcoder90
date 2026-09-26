// ==========================================================================
// 典型90 #005  Restricted Digits  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_e
// ==========================================================================
// 【アルゴリズム】 桁DP × ダブリング（dp[2^i] を倍々で作り、N の2進表現で合成）
// 【計算量】       O(B^2 log N)
// 【学習計画】     第8週（★7・発展。まずは通勤で読むだけで可）
//
// 【このファイルで覚えるRust文法】
//   - const MOD: i64 = 1_000_000_007; … 定数。数値の _ は桁区切り（Py の 1_000_000_007 と同じ）
//   - as による型変換 (b as i64, x as usize) … Rust は暗黙の型変換を一切しない
//   - vec.clone() … Py の list.copy()。Rust では代入は move なので、複製したいときは明示的に clone
//   - 0..=62 は 62 を含む閉区間（Py: range(0, 63)）
//
// 【Pythonで書くと（考え方の対応）】
//   # dp[i][r] = 2^i 桁の数で B で割った余りが r になる個数
//   # 2^i 桁同士を連結: 余り (j * 10^(2^i) + k) % B
//
// 【注意・改善ポイント】
//   ! dp[i][j] * dp[i][k] は最大 (1e9)^2 ≒ 1e18 で i64 の上限 9.2e18 以内。
//     MOD 演算の掛け算は「両方 MOD 未満なら i64 で安全」と覚えておく。
// ==========================================================================

use proconio::input;

const MOD: i64 = 1_000_000_007;

// 繰り返し二乗法 a^b mod m。Py なら組み込みの pow(a, b, m) 一発。Rust には無いので自作する。
fn modpow(a: i64, b: i64, m: i64) -> i64 {
    let mut p = 1;
    let mut q = a;
    for i in 0..63 {
        // b の i ビット目が立っていれば掛ける。1 << i は i64 として推論される（b と & するため）。
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
        // 1 << i … ここは modpow の引数 b: i64 から i64 と推論される。i が 63 以上だと溢れる。
        power10[i] = modpow(10, 1 << i, b as i64);
    }

    // Step #3: Initialize DP with the base cases for dp[1][i]
    // dp[i][r] : 桁数 2^i で、余り r になる数の個数。
    let mut dp = vec![vec![0; b]; 64];
    for &digit in &c {
        // 1桁の場合: 使える数字 c の余りごとに1個ずつ。
        dp[0][digit % b] += 1;
    }

    // Step #4: Compute dp[1][i], dp[2][i], ..., dp[2^n][i]
    for i in 0..62 {
        for j in 0..b {
            for k in 0..b {
                // power10[i] は i64 なので as usize で合わせる。Rust は i64 と usize を混ぜて計算できない。
                let nex = (j * power10[i] as usize + k) % b;
                // dp は vec![vec![0; b]; 64] で作ったが、MOD(i64) との演算から i64 と推論される。
                dp[i + 1][nex] = (dp[i + 1][nex] + dp[i][j] * dp[i][k]) % MOD;
            }
        }
    }

    // Step #5: Use exponentiation by squaring to compute dp[N][i]
    let mut answer = vec![vec![0; b]; 64];
    answer[0][0] = 1;
    for i in 0..62 {
        // N の i ビット目が立っていれば、今の答えに 2^i 桁ぶんを連結する。
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
            // Vec の代入は move（元の変数が使えなくなる）。answer[i] を残したいので clone で複製。
            answer[i + 1] = answer[i].clone();
        }
    }

    // Step #6: Output the result
    println!("{}", answer[62][0]);
}
