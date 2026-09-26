// ==========================================================================
// 典型90 #025  Digit Product Equation  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_y
// ==========================================================================
// 【アルゴリズム】 全探索の工夫（x の値ではなく「各桁の数字の多重集合」を列挙。非増加列の DFS で重複なく列挙）
// 【計算量】       O(C(19, 9) 程度 × 11桁)
// 【学習計画】     第8週以降（★7。発想問題なので通勤で読んで理解できれば十分）
//
// 【このファイルで覚えるRust文法】
//   - &mut String を再帰で共有し push / pop でバックトラック
//   - chars().map(...).product() … Py: math.prod(...)
//   - to_string() … 数値→文字列。Py: str(x)
//   - sort_by_key(|&c| Reverse(c)) … 降順ソート。Py: sort(reverse=True)
//   - collect::<String>() … Vec<char> → String。Py: ''.join(chars)
//   - *current_str … &mut String の中身と比較するための参照外し
//   - any(|c| ...) … Py: any(...)
//
// 【Pythonで書くと（考え方の対応）】
//   from itertools import combinations_with_replacement
//   for L in range(1, 12):
//       for ds in combinations_with_replacement('987654321', L):
//           g = math.prod(map(int, ds)) + B
//           if g <= N and ''.join(sorted(str(g), reverse=True)) == ''.join(ds): ans += 1
// ==========================================================================

use proconio::input;
use std::cmp::Reverse;

// answer: &mut i64 … 再帰の中から結果を加算するための可変参照。Py の nonlocal/global の代わり。
fn dfs(pos: i32, last: i32, current_str: &mut String, n: i64, b: i64, answer: &mut i64) {
    if pos == 0 {
        // c as i64 - '0' as i64 … char をコードに変換して '0' を引く。c.to_digit(10).unwrap() as i64 とも書ける。
        let product: i64 = current_str.chars().map(|c| (c as i64 - '0' as i64)).product();
        let goal = product + b;
        if goal <= n {
            // goal.to_string().chars().collect() … Py: list(str(goal))
            let mut goal_str: Vec<char> = goal.to_string().chars().collect();
            // Reverse でキーを包むと降順。Py: sorted(..., reverse=True)。sort_unstable_by(|a, b| b.cmp(a)) でも可。
            goal_str.sort_by_key(|&c| Reverse(c));
            // Vec<char> を String に集め、*current_str（String の実体）と比較。
            if goal_str.into_iter().collect::<String>() == *current_str {
                // *answer += 1 … 参照先の値を書き換え。
                *answer += 1;
            }
        }
        return;
    }

    // (1..=last).rev() … last から 1 まで降順。非増加列にすることで同じ多重集合を1回だけ列挙。
    for i in (1..=last).rev() {
        // 数字 i を文字に。Py: str(i)。char::from_digit(i as u32, 10).unwrap() とも書ける。
        current_str.push((i as u8 + b'0') as char);
        dfs(pos - 1, i, current_str, n, b, answer);
        // pop() でバックトラック（直前に push した文字を戻す）。
        current_str.pop();
    }
}

fn main() {
    input! {
        n: i64,
        b: i64,
    }

    let mut answer = 0;
    for length in 1..=11 {
        // &mut String::new() … 一時的な String への可変参照をその場で作って渡している。
        dfs(length, 9, &mut String::new(), n, b, &mut answer);
    }

    // Special case for sequences that contain only zeros
    // 0 を含む数は積が 0 → goal = B。B 自身が 0 を含み N 以下なら条件を満たす特別ケース。
    if b.to_string().chars().any(|c| c == '0') && n >= b {
        answer += 1;
    }

    println!("{}", answer);
}
