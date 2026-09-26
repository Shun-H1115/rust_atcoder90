// ==========================================================================
// 典型90 #057  Flip Flap  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_be
// ==========================================================================
// 【アルゴリズム】 GF(2) 上の掃き出し法（ランク r を求め、解があれば 2^(N - r) 通り）
// 【計算量】       O(N M^2)
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - Option を使ったピボット探索: let mut pivot_row = None; ... Some(j)
//   - d.swap(pr, pos) … Vec<Vec<u8>> の行（内側の Vec）を丸ごと入れ替え（中身はコピーされずポインタ交換だけ）
//   - ^= による XOR 演算で GF(2) の足し算
//   - s.iter().all(|&x| x == 0) … Py: all(x == 0 for x in s)
//   - （発展）行を u64 のビット列で持つと XOR が 64 倍速くなる（bitset 高速化）
//
// 【Pythonで書くと（考え方の対応）】
//   rank = 0
//   for col in range(M):
//       piv = next((r for r in range(rank, N) if D[r][col]), None)
//       if piv is None: continue
//       D[rank], D[piv] = D[piv], D[rank]
//       for r in range(N):
//           if r != rank and D[r][col]: D[r] = [x ^ y for x, y in zip(D[r], D[rank])]
//       ...; rank += 1
//   print(pow(2, N - rank, MOD) if consistent else 0)
// ==========================================================================

use proconio::input;

const MOD: i64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // 行列 D (n x m) を 0/1 で保持（GF(2) 演算用に u8 が手軽）
    // u8 で 0/1 を持つ。bool でもよいが、XOR を ^= で書けるので u8 が手軽。
    let mut d = vec![vec![0u8; m]; n];
    for i in 0..n {
        input! { t: usize }
        for _ in 0..t {
            // 1個ずつ読んで mut で受け -1。
            input! { mut x: usize }
            x -= 1;                // 1-indexed -> 0-indexed
            d[i][x] = 1;
        }
    }

    // 右辺ベクトル S (長さ m)
    input! { s_in: [usize; m] }
    // (x & 1) as u8 … 0/1 に正規化して u8 に。
    let mut s: Vec<u8> = s_in.into_iter().map(|x| (x & 1) as u8).collect();

    // 掃き出し（列ごとにピボットを探す）
    let mut pos = 0usize; // 現在のピボット行
    for i in 0..m {
        // 列 i で 1 のある行を pos..n-1 から探す
        // 型は Some(j) から Option<usize> と推論される。
        let mut pivot_row = None;
        for j in pos..n {
            if d[j][i] == 1 {
                pivot_row = Some(j);
                break;
            }
        }
        // if let で見つかった場合だけ処理（見つからなければこの列は飛ばす）。
        if let Some(pr) = pivot_row {
            if pr != pos {
                // Vec の swap は外側の要素（行）を入れ替える。O(1)。
                d.swap(pr, pos);
            }
            // 列 i の他行を 0 にする（XOR）
            for r in 0..n {
                if r != pos && d[r][i] == 1 {
                    for k in i..m {
                        d[r][k] ^= d[pos][k];
                    }
                }
            }
            // 右辺ベクトル側にも同じ操作を反映
            if s[i] == 1 {
                for k in i..m {
                    s[k] ^= d[pos][k];
                }
            }
            pos += 1; // 次のピボット行へ
        }
    }

    // s がすべて 0 なら解が存在。解の数は 2^(n - pos)
    // all は1つでも false があれば即 false（短絡評価）。
    if s.iter().all(|&x| x == 0) {
        let mut ans: i64 = 1;
        // 2^(n - pos) を繰り返しで計算。pos はランク。
        for _ in pos..n {
            ans = (ans * 2) % MOD;
        }
        println!("{}", ans);
    } else {
        println!("0");
    }
}
