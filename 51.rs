// ==========================================================================
// 典型90 #051  Typical Shop  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ay
// ==========================================================================
// 【アルゴリズム】 半分全列挙（前半・後半の部分集合和を個数別に列挙 → ソート＋二分探索で組を数える）
// 【計算量】       O(2^(N/2) × N)
// 【学習計画】     第5週（★5前半）
//
// 【このファイルで覚えるRust文法】
//   - 1usize << mid … シフト結果の型を明示
//   - partition_point(|x| *x <= target) … 条件を満たす先頭部分の長さ = Py の bisect_right(v, target)
//   - let v1 = &vec1[h]; … 借用して別名を付ける（コピーしない）
//   - sort_unstable() … 数値ソートは unstable で十分かつ速い
//
// 【Pythonで書くと（考え方の対応）】
//   from bisect import bisect_right
//   # v1[c], v2[c] = 前半/後半から c 個選んだ和のリスト（ソート済み）
//   ans = 0
//   for h in range(K+1):
//       for s in v1[h]:
//           ans += bisect_right(v2[K-h], P - s)
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: i64,
        a: [i64; n],
    }

    let mid = n / 2;
    let tail = n - mid;

    // vec1[c]: all sums of choosing c items from the first half
    // vec2[c]: all sums of choosing c items from the second half
    // vec![Vec::new(); n + 1] … 個数ごとにリストを持つ（Py: [[] for _ in range(n+1)]）。
    let mut vec1: Vec<Vec<i64>> = vec![Vec::new(); n + 1];
    let mut vec2: Vec<Vec<i64>> = vec![Vec::new(); n + 1];

    // Enumerate subsets of the first half
    // 前半 mid 個の部分集合を全列挙。
    for mask in 0..(1usize << mid) {
        let mut sum = 0i64;
        let mut cnt = 0usize;
        for j in 0..mid {
            if (mask >> j) & 1 == 1 {
                sum += a[j];
                cnt += 1;
            }
        }
        vec1[cnt].push(sum);
    }

    // Enumerate subsets of the second half
    for mask in 0..(1usize << tail) {
        let mut sum = 0i64;
        let mut cnt = 0usize;
        for j in 0..tail {
            if (mask >> j) & 1 == 1 {
                sum += a[mid + j];
                cnt += 1;
            }
        }
        vec2[cnt].push(sum);
    }

    // Sort for binary searches
    for i in 0..=n {
        vec1[i].sort_unstable();
        vec2[i].sort_unstable();
    }

    // Count pairs (h from 0..=K): pick h from first half and K-h from second half
    // For each sum s in vec1[h], count how many t in vec2[K-h] with s + t <= P
    let mut answer: i64 = 0;
    for h in 0..=k {
        // k - h は h <= k なので安全。h > n のチェックは念のため（Rust は範囲外アクセスで panic するので防御的に）。
        if h > n || k - h > n {
            continue; // safety (normally K <= N)
        }
        // &vec1[h] … 参照を取るだけでコピーは発生しない。
        let v1 = &vec1[h];
        let v2 = &vec2[k - h];
        for &s in v1.iter() {
            let target = p - s;
            // number of elements <= target
            // partition_point は「条件が true の区間の終わり」を返す。<= target なら bisect_right。
            // < target にすると bisect_left。これさえ覚えれば二分探索はほぼ困らない。
            let cnt = v2.partition_point(|x| *x <= target);
            answer += cnt as i64;
        }
    }

    println!("{}", answer);
}
