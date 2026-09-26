// ==========================================================================
// 典型90 #004  Cross Sum  (★2)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_d
// ==========================================================================
// 【アルゴリズム】 行の和・列の和を前計算（交差マスの重複を1回引く）
// 【計算量】       O(HW)
// 【学習計画】     第1週（★2）
//
// 【このファイルで覚えるRust文法】
//   - 2次元入力 [[i32; w]; h] … Py: [list(map(int, input().split())) for _ in range(h)]
//   - vec![vec![0; w]; h] で2次元配列（Py の [[0]*w]*h の罠は Rust には無い。各行は独立にコピーされる）
//   - print! と println! の違い: 改行なし / あり
//
// 【Pythonで書くと（考え方の対応）】
//   row = [sum(r) for r in A]
//   col = [sum(A[i][j] for i in range(H)) for j in range(W)]
//   for i in range(H):
//       print(*[row[i] + col[j] - A[i][j] for j in range(W)])
//
// 【注意・改善ポイント】
//   ! 出力が H×W で最大400万個。print! を大量に呼ぶと遅いので、Rustでは
//       let out = std::io::stdout(); let mut out = std::io::BufWriter::new(out.lock());
//       writeln!(out, ...) を使うのが定石（ここで一度覚えておくと以後ずっと使える）。
// ==========================================================================

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        // i32 の範囲は約±21億。ここでは和が最大 99×4000 程度なので i32 で足りる。迷ったら i64。
        a: [[i32; w]; h],
    }

    // vec![0; h] … 要素型は a の i32 と足し算することで i32 に推論される。
    let mut row_sum = vec![0; h];
    let mut col_sum = vec![0; w];

    // Calculate row and column sums
    for i in 0..h {
        for j in 0..w {
            row_sum[i] += a[i][j];
            col_sum[j] += a[i][j];
        }
    }

    // Calculate the result matrix
    let mut answer = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            // 行の和 + 列の和 - 交差マス（2回数えているので1回引く）
            answer[i][j] = row_sum[i] + col_sum[j] - a[i][j];
        }
    }

    // Output the result matrix
    for i in 0..h {
        for j in 0..w {
            // Py の print(*row) に相当する「空白区切り出力」を手で書いている。
            // 別解: println!("{}", answer[i].iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
            if j > 0 {
                print!(" ");
            }
            print!("{}", answer[i][j]);
        }
        println!();
    }
}
