// ==========================================================================
// 典型90 #031  VS AtCoder  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ae
// ==========================================================================
// 【アルゴリズム】 Grundy数（各山の Grundy 数を DP で求め、XOR が 0 でなければ先手必勝）
// 【計算量】       O(MAX_A × MAX_B^2)
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - 関数が Vec<Vec<usize>> を返す（所有権ごと呼び出し元へ）
//   - mex の計算: iter().position(|&x| !x) … Py: next(i for i, v in enumerate(mex) if not v)
//   - ^= … XOR 代入（Py と同じ）
//
// 【Pythonで書くと（考え方の対応）】
//   g = [[0]*(MB+1) for _ in range(MA+1)]
//   for i in range(MA+1):
//       for j in range(MB+1):
//           s = set()
//           if i >= 1 and j + i <= MB: s.add(g[i-1][j+i])
//           for k in range(1, j//2 + 1): s.add(g[i][j-k])
//           g[i][j] = next(x for x in range(len(s)+1) if x not in s)
//
// 【注意・改善ポイント】
//   ! mex 用の配列を毎回 MAX_B+1 個確保しているのは無駄が大きい。Grundy 数は遷移先の数以下なので、
//     vec![false; j/2 + 3] 程度で十分。確保回数を減らすのは Rust での高速化の基本。
// ==========================================================================

use proconio::input;

const MAX_A: usize = 50;
const MAX_B: usize = 1500;

fn init_grundy() -> Vec<Vec<usize>> {
    // grundy[白石数][青石数]
    let mut grundy = vec![vec![0; MAX_B + 1]; MAX_A + 1];
    
    for i in 0..=MAX_A {
        for j in 0..=MAX_B {
            let mut mex = vec![false; MAX_B + 1];
            
            // Case 1: Subtracting `i` from `j`
            // 白石を1個減らし、青石を白石の数（i 個）増やす操作。
            if i >= 1 && j + i <= MAX_B {
                mex[grundy[i - 1][j + i]] = true;
            }
            
            // Case 2: Halving `j`
            if j >= 2 {
                // 青石を k 個（1 <= k <= j/2）取り除く操作。
                for k in 1..=(j / 2) {
                    mex[grundy[i][j - k]] = true;
                }
            }
            
            // Find the minimum non-occurring value
            // position は Option<usize>。mex は必ず見つかるので unwrap。
            grundy[i][j] = mex.iter().position(|&x| !x).unwrap();
        }
    }
    
    // ; なしで grundy を返す。Vec の所有権が main 側の変数に移る（コピーは発生しない）。
    grundy
}

fn main() {
    // Input
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    // Initialize Grundy table
    let grundy = init_grundy();

    // Calculate the XOR sum of Grundy numbers for each pair (A[i], B[i])
    // 型は grundy の要素（usize）との XOR から usize と推論される。
    let mut sum_xor = 0;
    for i in 0..n {
        sum_xor ^= grundy[a[i]][b[i]];
    }

    // Output the result
    if sum_xor != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
