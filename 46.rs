// ==========================================================================
// 典型90 #046  I Love 46  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_at
// ==========================================================================
// 【アルゴリズム】 余りで分類（各配列を mod 46 で数え上げ、46^3 通りの組を調べる）
// 【計算量】       O(N + 46^3)
// 【学習計画】     第3週（★3後半）
//
// 【このファイルで覚えるRust文法】
//   - (val % 46) as usize … 余りを添字に変換
//   - 0_i64 … i64 で数える（組の数は最大 (1e5)^3 = 1e15 で i32 では溢れる）
//
// 【Pythonで書くと（考え方の対応）】
//   from collections import Counter
//   am = Counter(x % 46 for x in A); bm = ...; cm = ...
//   print(sum(am[i]*bm[j]*cm[k] for i in range(46) for j in range(46) for k in range(46) if (i+j+k) % 46 == 0))
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
        c: [i32; n],
    }

    // Count occurrences of each remainder when divided by 46
    // カウントは i64。掛け算で最大 1e15 になるため。
    let mut am = vec![0_i64; 46];
    let mut bm = vec![0_i64; 46];
    let mut cm = vec![0_i64; 46];

    for &val in &a {
        // val は正なので % は非負。負の可能性があれば rem_euclid(46) を使う。
        am[(val % 46) as usize] += 1;
    }
    for &val in &b {
        bm[(val % 46) as usize] += 1;
    }
    for &val in &c {
        cm[(val % 46) as usize] += 1;
    }

    // Calculate the number of valid triples
    // ans の型は am[i] * ... から i64。
    let mut ans = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                // 3重ループを k = (92 - i - j) % 46 と直接求めれば 46^2 に減らせる。
                if (i + j + k) % 46 == 0 {
                    ans += am[i] * bm[j] * cm[k];
                }
            }
        }
    }

    // Output the answer
    println!("{}", ans);
}
