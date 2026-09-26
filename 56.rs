// ==========================================================================
// 典型90 #056  Lucky Bag  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bd
// ==========================================================================
// 【アルゴリズム】 部分和DP＋復元（dp[i][j] = i 日目まででちょうど j 円にできるか。後ろから選択を復元）
// 【計算量】       O(NS)
// 【学習計画】     第6週（★5後半）
//
// 【このファイルで覚えるRust文法】
//   - bool の2次元 DP: vec![vec![false; s + 1]; n + 1]
//   - vec!['?'; n] … char の配列
//   - res.iter().collect::<String>() … Vec<char> → String（Py: ''.join(res)）
//   - j >= a && dp[i][j - a] … && の短絡評価で usize のアンダーフローを防ぐ定番パターン
//
// 【Pythonで書くと（考え方の対応）】
//   dp = [[False]*(S+1) for _ in range(n+1)]; dp[0][0] = True
//   for i, (a, b) in enumerate(AB):
//       for j in range(S+1):
//           dp[i+1][j] = (j >= a and dp[i][j-a]) or (j >= b and dp[i][j-b])
//   # 復元: 後ろから、B で来られたなら B、そうでなければ A
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..=s {
            // j >= a を先に書くことが重要。左が false なら右（j - a）は評価されないので panic しない。
            if j >= a && dp[i][j - a] {
                dp[i + 1][j] = true;
            }
            if j >= b && dp[i][j - b] {
                dp[i + 1][j] = true;
            }
        }
    }

    if !dp[n][s] {
        println!("Impossible");
    } else {
        let mut res = vec!['?'; n];
        let mut pos = s;
        // DP 復元は「後ろから、どの遷移で来たか」を辿る。
        for i in (0..n).rev() {
            let (a, b) = ab[i];
            if pos >= b && dp[i][pos - b] {
                res[i] = 'B';
                pos -= b;
            } else {
                res[i] = 'A';
                // B で来られないなら A で来たことが dp[n][s] = true から保証される。
                pos -= a;
            }
        }
        // 型注釈 : String があるので collect() の変換先が決まる。
        let result: String = res.iter().collect();
        println!("{}", result);
    }
}
