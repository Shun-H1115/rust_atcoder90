// ==========================================================================
// 典型90 #030  K Factors  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ad
// ==========================================================================
// 【アルゴリズム】 エラトステネスの篩の変形（素数 p の倍数すべてに +1 → 素因数の種類数）
// 【計算量】       O(N log log N)
// 【学習計画】     第5週（★5前半）
//
// 【このファイルで覚えるRust文法】
//   - (i..=n).step_by(i) … Py: range(i, n+1, i)
//   - cnt.iter().filter(|&&x| x >= k).count() … Py: sum(1 for x in cnt if x >= k)
//   - |&&x| … iter() の要素が &usize、filter の引数がさらにその参照 &&usize なので & を2回外す
//
// 【Pythonで書くと（考え方の対応）】
//   cnt = [0]*(n+1)
//   for i in range(2, n+1):
//       if cnt[i]: continue          # 素数以外はスキップ
//       for j in range(i, n+1, i): cnt[j] += 1
//   print(sum(1 for c in cnt if c >= k))
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    // Initialize the counter array
    let mut cnt = vec![0; n + 1];

    // Step #2. Count Number of Distinct Prime Factors
    for i in 2..=n {
        // cnt[i] >= 1 なら既に小さい素因数を持つ＝合成数。素数だけで篩う。
        if cnt[i] >= 1 {
            continue;
        }
        // step_by(i) で i の倍数を列挙。
        for j in (i..=n).step_by(i) {
            cnt[j] += 1;
        }
    }

    // Step #3. Calculate the Answer
    // filter のクロージャは「要素への参照」を受け取る。iter() の要素 &usize への参照で &&usize。
    // |x| **x >= k と書いても同じ。混乱したら .copied() を挟むと |&x| になる。
    let answer = cnt.iter().filter(|&&x| x >= k).count();
    println!("{}", answer);
}
