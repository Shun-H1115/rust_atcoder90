// ==========================================================================
// 典型90 #081  Friendly Group  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_cc
// ==========================================================================
// 【アルゴリズム】 2次元累積和（(A, B) の点を数える表を作り、K×K の正方形窓を全部スライドして最大値）
// 【計算量】       O(N + maxA × maxB)
// 【学習計画】     第6週（★5後半）
//
// 【このファイルで覚えるRust文法】
//   - 2次元累積和の包除: S = s[i2][j2] - s[i][j2] - s[i2][j] + s[i][j]
//   - if r >= k + 1 && ... … usize の引き算の前に大小を確認（r - k - 1 の panic 防止）
//   - 0..=r - k - 1 … 閉区間の上限に式
//
// 【Pythonで書くと（考え方の対応）】
//   s = [[0]*(C+1) for _ in range(R+1)]
//   for a, b in AB: s[a+1][b+1] += 1
//   for i in range(1, R+1):
//       for j in range(1, C+1): s[i][j] += s[i-1][j] + s[i][j-1] - s[i-1][j-1]
//   print(max(s[i+K+1][j+K+1] - s[i][j+K+1] - s[i+K+1][j] + s[i][j]
//             for i in range(R-K) for j in range(C-K)))
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    }

    // Separate A and B just to compute maxima (or compute on the fly)
    // 盤面サイズを入力の最大値から決める（最低 K）。
    let mut max_a = k;
    let mut max_b = k;
    for &(a, b) in &ab {
        if a > max_a { max_a = a; }
        if b > max_b { max_b = b; }
    }

    // Dimensions for 2D prefix sums (1-based board with extra 0-row/col)
    let r = max_a + 1;
    let c = max_b + 1;

    // sum has size (r+1) x (c+1); we'll use i32 since counts ≤ n
    // カウントは N 以下なので i32 で十分。
    let mut sum = vec![vec![0i32; c + 1]; r + 1];

    // Increment points at (a+1, b+1)
    for &(a, b) in &ab {
        sum[a + 1][b + 1] += 1;
    }

    // Prefix sums: first vertical, then horizontal (same as C++ code)
    // 縦方向 → 横方向の2回に分けて累積すると、包除の式を書かずに2次元累積和が作れる。
    for i in 1..=r {
        for j in 1..=c {
            sum[i][j] += sum[i - 1][j];
        }
    }
    for i in 1..=r {
        for j in 1..=c {
            sum[i][j] += sum[i][j - 1];
        }
    }

    // Slide KxK (inclusive) window:
    // Query rectangle via inclusion-exclusion:
    // S = sum[i][j] + sum[i+K+1][j+K+1] - sum[i][j+K+1] - sum[i+K+1][j]
    // with i in [0, r-K-1], j in [0, c-K-1]
    let mut answer: i32 = 0;
    // 先に大小比較してから引き算。これをしないと r < k + 1 のとき usize がアンダーフロー。
    if r >= k + 1 && c >= k + 1 {
        for i in 0..=r - k - 1 {
            let i2 = i + k + 1;
            for j in 0..=c - k - 1 {
                let j2 = j + k + 1;
                // [i, i2) × [j, j2) の長方形の点の数（包除原理）。
                let val = sum[i][j] + sum[i2][j2] - sum[i][j2] - sum[i2][j];
                if val > answer {
                    answer = val;
                }
            }
        }
    }

    println!("{}", answer);
}
