// ==========================================================================
// 典型90 #076  Cake Cut  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bx
// ==========================================================================
// 【アルゴリズム】 円環を2周分の累積和に展開＋二分探索（B[i] + 全体/10 となる B[j] が存在するか）
// 【計算量】       O(N log N)
// 【学習計画】     第3週（★3後半）
//
// 【このファイルで覚えるRust文法】
//   - 円環の処理: 配列を2周分に伸ばす（Py でも a + a とする定番）
//   - b.binary_search(&goal) … ソート済み配列で値を探す。Ok(位置) なら存在、Err なら無い
//   - （別解）HashSet に累積和を入れて contains で O(1) 判定、しゃくとり法で O(N)
//
// 【Pythonで書くと（考え方の対応）】
//   b = [0]
//   for x in a + a: b.append(b[-1] + x)
//   S = b[n]; st = set(b)
//   print('Yes' if S % 10 == 0 and any(b[i] + S // 10 in st for i in range(n+1)) else 'No')
// ==========================================================================

use proconio::input;

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        a: [i64; n],
    }

    // Step #2. Create Array B
    // 2周分の累積和 b[0..=2n]。要素型は a（i64）との加算から推論。
    let mut b = vec![0; 2 * n + 1];
    for i in 1..=n {
        b[i] = b[i - 1] + a[i - 1];
    }
    for i in 1..=n {
        b[i + n] = b[i + n - 1] + a[i - 1];
    }
    
    // Check if the total sum is not divisible by 10
    // 全体が10で割り切れなければ不可能。
    if b[n] % 10 != 0 {
        println!("No");
        return;
    }

    // Step #3. Get Answer
    for i in 0..=n {
        let goal = b[i] + b[n] / 10;
        // Binary search to find the position
        // binary_search は Ok/Err の Result。unwrap_or_else(|x| x) で位置を取り出してから値を比較している。
        // b.binary_search(&goal).is_ok() と書けば存在判定が1行で済む。
        let pos1 = b.binary_search(&goal).unwrap_or_else(|x| x);
        if pos1 < b.len() && b[pos1] == goal {
            println!("Yes");
            return;
        }
    }
    
    println!("No");
}
