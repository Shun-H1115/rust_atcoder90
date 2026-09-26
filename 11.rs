// ==========================================================================
// 典型90 #011  Gravy Jobs  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_k
// ==========================================================================
// 【アルゴリズム】 締切順ソート＋ナップサックDP（dp[i][j] = i個目まで見て合計日数jのときの最大報酬）
// 【計算量】       O(N × 5000)
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - let mut tasks = tasks; … シャドーイング。同名で再宣言して mut に変える（Py には無い概念）
//   - sort_by_key(|&(d, _, _)| d) … Py: tasks.sort(key=lambda t: t[0])
//   - std::cmp::max … 2引数の max
//   - iter().max() は Option<&T> を返す → unwrap() が必要（空配列なら None）
//
// 【Pythonで書くと（考え方の対応）】
//   tasks.sort()
//   dp = [[0]*5001 for _ in range(n+1)]
//   for i, (d, c, s) in enumerate(tasks):
//       for j in range(5001):
//           dp[i+1][j] = max(dp[i+1][j], dp[i][j])
//           if j + c <= d:
//               dp[i+1][j+c] = max(dp[i+1][j+c], dp[i][j] + s)
//   print(max(dp[n]))
// ==========================================================================

use proconio::input;
use std::cmp::max;

fn main() {
    // Step #1: Input
    input! {
        n: usize,
        // (D, C, S) の3つ組。D と C は添字計算に使うので usize、報酬 S は大きくなるので i64。
        tasks: [(usize, usize, i64); n], // Each task has (D, C, S) for Deadline, Cost, Score
    }

    // Sort tasks by their deadline to maximize chances of completion
    // シャドーイング: 不変で受け取った tasks を可変の tasks として再定義。
    // （input! 内で mut tasks: [...] と書いても同じ）
    let mut tasks = tasks;
    // 締切の早い順に処理するのが「締切付きスケジューリング」の定石（交換論法で正当化できる）。
    // |&(d, _, _)| d … 引数のタプルを分解し d だけ使う。_ は無視。
    tasks.sort_by_key(|&(d, _, _)| d);

    // Initialize DP table
    // 5001 は締切の最大値 5000 + 1。値は dp[i][j] + s（i64）との演算で i64 に推論される。
    let mut dp = vec![vec![0; 5001]; n + 1];

    // Step #2: Dynamic Programming
    for i in 0..n {
        let (d, c, s) = tasks[i];
        for j in 0..=5000 {
            // Option 1: Skip this task
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            
            // Option 2: Take this task if it can be completed on time
            // j + c <= d なので j + c は 5000 以下 → 添字範囲外にならない。
            if j + c <= d {
                dp[i + 1][j + c] = max(dp[i + 1][j + c], dp[i][j] + s);
            }
        }
    }

    // Step #3: Output the maximum reward
    // dp[n].iter().max() は Option<&i64>。unwrap で &i64 を取り出す（println! は参照でもそのまま表示できる）。
    let answer = dp[n].iter().max().unwrap();
    println!("{}", answer);
}
