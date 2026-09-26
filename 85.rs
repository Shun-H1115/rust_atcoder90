// ==========================================================================
// 典型90 #085  Multiplication 085  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_cg
// ==========================================================================
// 【アルゴリズム】 約数列挙（K の約数だけを候補に a ≤ b ≤ c の組を数える。約数は高々 6720 個程度）
// 【計算量】       O(√K + d(K)^2)
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - 約数列挙の定番パターン（i と k / i を同時に push）
//   - vec.sort() … 整数の Vec は sort() でOK（f64 は不可。#009 参照）
//   - k / a < b で a * b の溢れを回避（k ≤ 1e12 なので a * b は i64 に収まるが、割り算判定は安全な癖）
//
// 【Pythonで書くと（考え方の対応）】
//   divs = sorted({d for i in range(1, int(K**0.5)+1) if K % i == 0 for d in (i, K // i)})
//   ans = 0
//   for i, a in enumerate(divs):
//       for b in divs[i:]:
//           if K // a < b: break
//           if K % (a*b) == 0 and b <= K // (a*b): ans += 1
//   print(ans)
// ==========================================================================

use proconio::input;

fn main() {
    // Step #1. Input
    input! {
        k: i64,
    }

    // Step #2. Enumerate Divisors
    let mut vec = Vec::new();
    // √K まで試す。f64 の sqrt は K ≤ 1e12 程度なら誤差の心配はほぼない（1e15 を超えると要注意）。
    for i in 1..=((k as f64).sqrt() as i64) {
        if k % i != 0 {
            continue;
        }
        vec.push(i);
        // 平方数のとき i と k/i が同じなので重複を避ける。
        if i != k / i {
            vec.push(k / i);
        }
    }
    vec.sort();

    // Step #3. Brute Force
    let mut answer = 0;
    let size = vec.len();
    for i in 0..size {
        for j in i..size {
            let a = vec[i];
            let b = vec[j];
            // b は昇順なので、ここで continue ではなく break してもよい（以降の b はもっと大きい）。
            if k / a < b {
                continue;
            }
            if k % (a * b) != 0 {
                continue;
            }
            let c = k / (a * b);
            // a ≤ b ≤ c の順序を保つことで同じ組を重複して数えない。
            if b <= c {
                answer += 1;
            }
        }
    }

    // Step #4. Output
    println!("{}", answer);
}
