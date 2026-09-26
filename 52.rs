// ==========================================================================
// 典型90 #052  Dice Product  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_az
// ==========================================================================
// 【アルゴリズム】 因数分解（Σ の積 = 各サイコロの目の合計の積）
// 【計算量】       O(6N)
// 【学習計画】     第3週（★3後半）
//
// 【このファイルで覚えるRust文法】
//   - [[i64; 6]; n] … 1行6個を n 行
//   - a[i].iter().sum() … 行の合計（Py: sum(a[i])）
//
// 【Pythonで書くと（考え方の対応）】
//   ans = 1
//   for row in A: ans = ans * sum(row) % MOD
//   print(ans)
// ==========================================================================

use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [[i64; 6]; n],
    }

    // 型は answer * sum（i64）から推論される。
    let mut answer = 1;

    // Compute the answer by multiplying sums of each row
    for i in 0..n {
        // sum() の型は左辺の : i64 で指定。
        let sum: i64 = a[i].iter().sum();
        // answer < MOD, sum <= 600 なので積は溢れない。
        answer = (answer * sum) % MOD;
    }

    println!("{}", answer);
}
