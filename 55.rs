// ==========================================================================
// 典型90 #055  Select 5  (★2)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bc
// ==========================================================================
// 【アルゴリズム】 全探索（5重ループ。制約から C(N,5) が 1e8 程度に収まることが保証されている）
// 【計算量】       O(C(N, 5))
// 【学習計画】     第1週（★2）
//
// 【このファイルで覚えるRust文法】
//   - 5重ループでも Rust なら 1e8 回程度は1秒以内に回る（Py では TLE、PyPy でも厳しい）
//   - 掛け算の途中で毎回 % p して i64 の溢れを防ぐ
//
// 【Pythonで書くと（考え方の対応）】
//   from itertools import combinations
//   print(sum(1 for c in combinations(A, 5)
//             if A_prod_mod(c, P) == Q))   # Py では TLE しがち
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: i64,
        q: i64,
        a: [i64; n],
    }

    let mut ans = 0;
    for i in 0..n {
        // 0..i, 0..j, ... と範囲を狭めることで同じ組を重複して数えない（i > j > k > l > m）。
        for j in 0..i {
            for k in 0..j {
                for l in 0..k {
                    for m in 0..l {
                        // a[i] * a[j] は最大 1e18 で i64 に収まる。以降は毎回 % p して 1e9 未満に戻してから掛ける。
                        if a[i] * a[j] % p * a[k] % p * a[l] % p * a[m] % p == q {
                            ans += 1;
                        }
                    }
                }
            }
        }
    }

    println!("{}", ans);
}
