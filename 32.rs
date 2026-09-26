// ==========================================================================
// 典型90 #032  AtCoder Ekiden  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_af
// ==========================================================================
// 【アルゴリズム】 順列全探索（N ≤ 10 なので N! 通りを全部試す）
// 【計算量】       O(N! × N)
// 【学習計画】     第2週（★3前半）
//
// 【このファイルで覚えるRust文法】
//   - Rust 標準には next_permutation / itertools.permutations が無い → 自作 or itertools クレート
//   - （AtCoder なら use itertools::Itertools; で (0..n).permutations(n) が使える）
//   - クロージャ let possible = |order: &Vec<usize>| -> bool { ... }; … 外側の変数 incompatible, n を捕獲
//   - loop { ... break; } … 無限ループ（Py の while True）
//   - wrapping_sub … usize の 0 - 1 を panic させず usize::MAX に「巻き戻す」引き算
//   - arr[i + 1..].reverse() … スライスの一部だけ反転
//
// 【Pythonで書くと（考え方の対応）】
//   from itertools import permutations
//   ans = INF
//   for p in permutations(range(n)):
//       if all(not bad[p[i]][p[i+1]] for i in range(n-1)):
//           ans = min(ans, sum(A[p[i]][i] for i in range(n)))
//   print(ans if ans < INF else -1)
// ==========================================================================

use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        incompatible_pairs: [(usize, usize); m],
    }

    // Initialize the incompatibility matrix
    let mut incompatible = vec![vec![false; n]; n];
    // incompatible_pairs を消費しながら分解。
    for (x, y) in incompatible_pairs {
        incompatible[x - 1][y - 1] = true;
        incompatible[y - 1][x - 1] = true;
    }

    // Initialize variables for brute force approach
    // (0..n).collect() … Py: list(range(n))。型注釈 Vec<usize> で collect 先を指定。
    let mut runners: Vec<usize> = (0..n).collect();
    // usize::MAX を「未更新」の印に使っている。
    let mut answer = usize::MAX;
    // クロージャは外側の変数（incompatible, n）を借用して使える。Py の lambda/内部関数と同じ。
    let possible = |order: &Vec<usize>| -> bool {
        for i in 0..(n - 1) {
            if incompatible[order[i]][order[i + 1]] {
                return false;
            }
        }
        true
    };

    // Brute force by trying all permutations of runners
    // loop は Py の while True。break で抜ける。
    loop {
        if possible(&runners) {
            let mut total_time = 0;
            for i in 0..n {
                total_time += a[runners[i]][i];
            }
            answer = min(answer, total_time);
        }

        // If no next permutation, break the loop
        // &mut runners … 次の順列に書き換える。辞書順で最後なら false。
        if !next_permutation(&mut runners) {
            break;
        }
    }

    // Output the result
    if answer == usize::MAX {
        println!("-1");
    } else {
        println!("{}", answer);
    }
}

// Helper function to generate the next lexicographical permutation
// C++ の std::next_permutation の移植。アルゴリズムとして覚えておくと他言語でも役立つ。
fn next_permutation(arr: &mut Vec<usize>) -> bool {
    let n = arr.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 2;
    // i が 0 の次に wrapping_sub で usize::MAX になったら「見つからなかった」と判定する小技。
    // isize で書くか、for i in (0..n-1).rev() で位置を探す方が読みやすい。
    while i != usize::MAX && arr[i] >= arr[i + 1] {
        i = i.wrapping_sub(1);
    }
    if i == usize::MAX {
        arr.reverse();
        return false;
    }
    let mut j = n - 1;
    while arr[j] <= arr[i] {
        j -= 1;
    }
    arr.swap(i, j);
    // arr[i+1..] … i+1 以降のスライス。Py: arr[i+1:] = reversed(arr[i+1:])
    arr[i + 1..].reverse();
    true
}
