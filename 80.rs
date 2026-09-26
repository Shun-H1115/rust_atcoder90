// ==========================================================================
// 典型90 #080  Let's Share Bit  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_cb
// ==========================================================================
// 【アルゴリズム】 包除原理（「A_i と AND が 0」の条件を満たさない集合を全列挙し、符号を交互に付けて数える）
// 【計算量】       O(2^N × (N + D))
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - u128 / i128 … 128bit 整数。2^D（D ≤ 60）を安全に扱うため（実は i64 でも収まる）
//   - mask.count_ones() … 選んだ条件の個数（偶奇で符号を決める）
//   - シフト量の型: bit >> (j as u32) … u128 のシフト量は u32 が標準（usize でも可）
//   - 1_i128 << x … i128 のリテラル
//
// 【Pythonで書くと（考え方の対応）】
//   ans = 0
//   for mask in range(1 << N):
//       bit = 0
//       for j in range(N):
//           if mask >> j & 1: bit |= A[j]
//       free = sum(1 for j in range(D) if not bit >> j & 1)
//       ans += (-1) ** bin(mask).count('1') * (1 << free)
//   print(ans)
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        // A_i < 2^60 なので u64 で十分。u128 は安全側に倒した選択。
        a_in: [u128; n], // treat constraints as bitmasks
    }

    let mut answer: i128 = 0;

    // 1usize << n … mask の型を usize に固定。
    let total = 1usize << n;
    for mask in 0..total {
        // OR all selected constraints
        let mut bit: u128 = 0;
        for j in 0..n {
            if (mask >> j) & 1 == 1 {
                bit |= a_in[j];
            }
        }

        // Count free digits among the lowest D bits
        let mut free_digits: usize = 0;
        for j in 0..d {
            // bit の j ビット目が 0 = その桁は自由に 0/1 を選べる。
            if ((bit >> (j as u32)) & 1) == 0 {
                free_digits += 1;
            }
        }

        // 自由な桁が f 個 → 2^f 通り。
        let ways: i128 = 1_i128 << (free_digits as u32);
        // 包除原理: 条件を偶数個選んだら +、奇数個なら -。
        if mask.count_ones() % 2 == 0 {
            answer += ways;
        } else {
            answer -= ways;
        }
    }

    println!("{}", answer);
}
