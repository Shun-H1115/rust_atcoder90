// ==========================================================================
// 典型90 #063  Monochromatic Subgrid  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bk
// ==========================================================================
// 【アルゴリズム】 bit全探索（行の選び方 2^H を全探索し、選んだ行ですべて同じ値の列のうち最頻値の個数を数える）
// 【計算量】       O(2^H × HW)
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - 1u32 << h … i を u32 にしておくと count_ones()（立っているビット数）が使える。Py: bin(i).count('1') / i.bit_count()
//   - HashMap の entry API で頻度カウント（Py の Counter）
//   - ret.max(*count) … メソッド版 max
//   - 関数に Vec を値で渡す maximum_same(r) … r の所有権が関数に移る（呼び出し後 r は使えない）
//
// 【Pythonで書くと（考え方の対応）】
//   from collections import Counter
//   ans = 0
//   for S in range(1, 1 << H):
//       rows = [k for k in range(H) if S >> k & 1]
//       vals = [P[rows[0]][j] for j in range(W) if all(P[k][j] == P[rows[0]][j] for k in rows)]
//       if vals: ans = max(ans, len(rows) * Counter(vals).most_common(1)[0][1])
// ==========================================================================

use proconio::input;
use std::collections::HashMap;

// r: Vec<i32> を値で受け取る（所有権ごと）。読むだけなら r: &[i32] の方が一般的。
fn maximum_same(r: Vec<i32>) -> usize {
    let mut map = HashMap::new();
    let mut ret = 0;
    for &value in &r {
        // Py の cnt[value] += 1（defaultdict）に相当。count は &mut i32。
        let count = map.entry(value).or_insert(0);
        *count += 1;
        // ret は戻り値の型 usize と推論され、ret.max(*count) により map の値型（count）も usize に決まる。
        // Rust の型推論は関数全体を見て決まる（後ろの行の情報が前の行の型を決めることもある）。
        ret = ret.max(*count);
    }
    ret
}

fn main() {
    // Step 1: Input
    input! {
        h: usize,
        w: usize,
        p: [[i32; w]; h],
    }

    let mut answer = 0;

    // Step 2: Bit masking over rows
    // 1u32 << h で範囲の型を u32 に固定。後で i.count_ones() を呼ぶため。
    for i in 1..(1u32 << h) { // Specify i as u32 to use count_ones method
        let mut r = Vec::new();
        for j in 0..w {
            // -1 を「まだ値が決まっていない」の印に（p の値は正）。Option<i32> で書くとより安全。
            let mut idx = -1;
            let mut flag = false;
            for k in 0..h {
                // 1 << k … i が u32 なので u32 として推論される。
                if (i & (1 << k)) == 0 {
                    continue;
                }
                if idx == -1 {
                    idx = p[k][j];
                } else if idx != p[k][j] {
                    flag = true;
                    break;
                }
            }
            if !flag {
                r.push(idx);
            }
        }

        // count_ones() … 立っているビット数 = 選んだ行数。
        let cnt_h = i.count_ones() as usize;
        // r を move して渡す。この後 r は使わないので問題ない。
        let cnt_w = maximum_same(r);
        answer = answer.max(cnt_h * cnt_w);
    }

    // Output result
    println!("{}", answer);
}
