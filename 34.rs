// ==========================================================================
// 典型90 #034  There are few types of elements  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ah
// ==========================================================================
// 【アルゴリズム】 しゃくとり法＋HashMap（区間内の種類数が K 以下を保ちながら右端を伸ばす）
// 【計算量】       O(N)
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - entry(key).or_insert(0) … Py の defaultdict(int) / d.setdefault(k, 0)。可変参照 &mut V を返す
//   - *entry += 1 … 返ってきた &mut i32 の中身を書き換え
//   - get_mut(&key) … 値の可変参照を Option で返す
//   - 借用のスコープ: entry は while の1周ごとに作り直すので、get_mut との衝突が起きない
//
// 【Pythonで書くと（考え方の対応）】
//   from collections import defaultdict
//   cnt = defaultdict(int); r = 0; kinds = 0; ans = 0
//   for l in range(n):
//       while r < n and (cnt[a[r]] > 0 or kinds < K):
//           if cnt[a[r]] == 0: kinds += 1
//           cnt[a[r]] += 1; r += 1
//       ans = max(ans, r - l)
//       cnt[a[l]] -= 1
//       if cnt[a[l]] == 0: kinds -= 1
// ==========================================================================

use proconio::input;
use std::collections::HashMap;
use std::cmp::max;

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        k: usize,
        a: [i32; n],
    }

    // Step #2. Sliding Window (Shakutori method)
    let mut answer = 0;
    // 型は entry(a[cr])（i32）と or_insert(0) から HashMap<i32, i32> と推論される。
    let mut count_map = HashMap::new();
    let mut cr = 0;
    let mut distinct_count = 0;

    for i in 0..n {
        while cr < n {
            // entry API … キーが無ければ 0 を入れ、その値への &mut を返す。Py: cnt.setdefault(a[cr], 0)
            // この entry を持っている間は count_map を他で触れない（可変借用中）。
            let entry = count_map.entry(a[cr]).or_insert(0);
            // 新しい種類を追加すると K を超えるなら右端を止める。
            if *entry == 0 && distinct_count == k {
                break;
            }
            if *entry == 0 {
                distinct_count += 1;
            }
            // *entry … 参照先の値を +1。
            *entry += 1;
            cr += 1;
        }
        // cr - i … 現在の区間長。どちらも usize、cr >= i なので安全。
        answer = max(answer, cr - i);
        // get_mut は Option<&mut i32>。if let で Some のときだけ書き換える。
        if let Some(count) = count_map.get_mut(&a[i]) {
            *count -= 1;
            if *count == 0 {
                distinct_count -= 1;
            }
        }
    }

    // Step #3. Output the Answer
    println!("{}", answer);
}
