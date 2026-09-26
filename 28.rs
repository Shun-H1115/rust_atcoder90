// ==========================================================================
// 典型90 #028  Cluttered Paper  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ab
// ==========================================================================
// 【アルゴリズム】 2次元いもす法（四隅に ±1 を置き、横→縦に累積和）
// 【計算量】       O(N + 1000^2)
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - for &(lx, ly, rx, ry) in &rectangles … 4要素タプルを分解
//   - cnt[i][j] as usize … i32 の値を添字に使うときの変換
//   - 2重ループの累積和（Py では numpy.cumsum を使いたくなるところ）
//
// 【Pythonで書くと（考え方の対応）】
//   cnt = [[0]*1001 for _ in range(1001)]
//   for lx, ly, rx, ry in R:
//       cnt[lx][ly] += 1; cnt[lx][ry] -= 1; cnt[rx][ly] -= 1; cnt[rx][ry] += 1
//   # 横方向・縦方向に累積和 → 値 k のマス数を数える
// ==========================================================================

use proconio::input;

fn main() {
    // Input
    input! {
        n: usize,
        rectangles: [(usize, usize, usize, usize); n],
    }

    // Initialize the grid and answer arrays
    // 要素型は -= 1 と as usize から i32（整数の既定型）になる。負になりうるので usize は不可。
    let mut cnt = vec![vec![0; 1001]; 1001];
    let mut answer = vec![0; n + 1];

    // Step #2. Imos Method in 2D
    for &(lx, ly, rx, ry) in &rectangles {
        // 左下 +1, 右下 -1, 左上 -1, 右上 +1（座標は半開区間 [lx, rx) × [ly, ry)）。
        cnt[lx][ly] += 1;
        cnt[lx][ry] -= 1;
        cnt[rx][ly] -= 1;
        cnt[rx][ry] += 1;
    }

    // Horizontal cumulative sum
    for i in 0..=1000 {
        for j in 1..=1000 {
            // 横方向の累積和。j-1 を使うので j は 1 から。
            cnt[i][j] += cnt[i][j - 1];
        }
    }

    // Vertical cumulative sum
    for i in 1..=1000 {
        for j in 0..=1000 {
            cnt[i][j] += cnt[i - 1][j];
        }
    }

    // Step #3. Count the number of cells with each overlap count
    for i in 0..=1000 {
        for j in 0..=1000 {
            if cnt[i][j] >= 1 {
                // 重なり枚数 cnt[i][j] ごとに面積を数える。i32 → usize 変換して添字に。
                answer[cnt[i][j] as usize] += 1;
            }
        }
    }

    // Step #4. Output the Answer
    for i in 1..=n {
        println!("{}", answer[i]);
    }
}
