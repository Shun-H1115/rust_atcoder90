// ==========================================================================
// 典型90 #014  We Used to Sing a Song Together  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_n
// ==========================================================================
// 【アルゴリズム】 貪欲法（両方ソートして同じ順位同士を対応させると差の総和が最小）
// 【計算量】       O(N log N)
// 【学習計画】     第2週（★3前半）
//
// 【このファイルで覚えるRust文法】
//   - イテレータチェーン: a.iter().zip(b.iter()).map(...).sum() … Py の sum(abs(x-y) for x, y in zip(a, b))
//   - sum() は戻り値の型を指定する必要がある → let answer: i64 = ...
//   - クロージャ |(ai, bi)| … Py の lambda。ai, bi は &i64（参照）だが算術演算はそのまま書ける
//
// 【Pythonで書くと（考え方の対応）】
//   a.sort(); b.sort()
//   print(sum(abs(x - y) for x, y in zip(a, b)))
// ==========================================================================

use proconio::input;

fn main() {
    // Step #1: Input
    input! {
        n: usize,
        // mut a … ソートするので可変で受け取る。
        mut a: [i64; n],
        mut b: [i64; n],
    }

    // Step #2: Sort both arrays
    a.sort();
    b.sort();

    // Step #3: Calculate the sum of absolute differences
    // 【イテレータチェーンの基本形】Python のジェネレータ式と1対1で対応する。
    //   a.iter()        → for x in a
    //   .zip(b.iter())  → zip(a, b)
    //   .map(|..| ..)   → 式部分
    //   .sum()          → sum(...)
    // sum は結果の型が決まらないとコンパイルできないので `: i64` の型注釈が必要。
    let answer: i64 = a.iter().zip(b.iter()).map(|(ai, bi)| (ai - bi).abs()).sum();

    // Output the answer
    println!("{}", answer);
}
