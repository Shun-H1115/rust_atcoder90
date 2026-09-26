// ==========================================================================
// 典型90 #019  Pick Two  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_s
// ==========================================================================
// 【アルゴリズム】 区間DP（dp[l][r] = 区間[l,r]を全部消す最小コスト。分割 or 両端をペアにする）
// 【計算量】       O(N^3)
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - into_iter().enumerate() … a_in を消費しながら (添字, 値) を取り出す
//   - 0_i64 / 1_i64 << 60 … 型サフィックス付きリテラル（i64 と明示）
//   - (3..=m).step_by(2) … Py: range(3, m+1, 2)
//
// 【Pythonで書くと（考え方の対応）】
//   dp = [[INF]*(m+2) for _ in range(m+2)]
//   for i in range(1, m): dp[i][i+1] = abs(a[i] - a[i+1])
//   for ln in range(3, m+1, 2):
//       for l in range(1, m - ln + 1):
//           r = l + ln
//           dp[l][r] = min(min(dp[l][k] + dp[k+1][r] for k in range(l, r)),
//                          dp[l+1][r-1] + abs(a[l] - a[r]))
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        a_in: [i64; 2 * n],
    }

    let m = 2 * n; // total number of elements
    // 1-based indexing for simplicity (index 0 unused)
    // 1-indexed にするため先頭にダミーを入れた配列を作り直している。
    let mut a = vec![0_i64; m + 1];
    // into_iter() は a_in を消費（move）する。以後 a_in は使えない。iter() なら借用。
    for (i, v) in a_in.into_iter().enumerate() {
        a[i + 1] = v;
    }

    // 1_i64 << 60 … リテラル自体に型を付ける書き方。ここは `: i64` 注釈もあるので 1 << 60 でも可。
    // 型注釈のない場所（let x = 1 << 60;）では i32 と推論され溢れるので、サフィックスで固定する癖をつけると安全。
    let inf: i64 = 1_i64 << 60;
    // dp size: (m + 2) x (m + 2) to safely access l+1 and r-1
    let mut dp = vec![vec![inf; m + 2]; m + 2];

    // Base cases: segments of length 2
    for i in 1..m {
        dp[i][i + 1] = (a[i] - a[i + 1]).abs();
    }

    // Consider segments where (r - l + 1) is even and >= 4
    // len は r - l（区間長-1）。奇数刻みで「要素数が偶数の区間」だけを扱う。
    for len in (3..=m).step_by(2) {
        for l in 1..=m - len {
            let r = l + len;

            // Case 1: split at k
            // 区間を [l,k] と [k+1,r] に分割するケース。奇数長になる分割は INF のままなので自然に除外される。
            for k in l..r {
                let candidate = dp[l][k] + dp[k + 1][r];
                if candidate < dp[l][r] {
                    dp[l][r] = candidate;
                }
            }
            // Case 2: pair ends (l with r)
            // 両端 l と r を最後にペアで消すケース（内側を先に全部消しておく）。
            let ends = dp[l + 1][r - 1] + (a[l] - a[r]).abs();
            if ends < dp[l][r] {
                dp[l][r] = ends;
            }
        }
    }

    println!("{}", dp[1][m]);
}
