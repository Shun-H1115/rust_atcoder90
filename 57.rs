use proconio::input;

const MOD: i64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    // 行列 D (n x m) を 0/1 で保持（GF(2) 演算用に u8 が手軽）
    let mut d = vec![vec![0u8; m]; n];
    for i in 0..n {
        input! { t: usize }
        for _ in 0..t {
            input! { mut x: usize }
            x -= 1;                // 1-indexed -> 0-indexed
            d[i][x] = 1;
        }
    }

    // 右辺ベクトル S (長さ m)
    input! { s_in: [usize; m] }
    let mut s: Vec<u8> = s_in.into_iter().map(|x| (x & 1) as u8).collect();

    // 掃き出し（列ごとにピボットを探す）
    let mut pos = 0usize; // 現在のピボット行
    for i in 0..m {
        // 列 i で 1 のある行を pos..n-1 から探す
        let mut pivot_row = None;
        for j in pos..n {
            if d[j][i] == 1 {
                pivot_row = Some(j);
                break;
            }
        }
        if let Some(pr) = pivot_row {
            if pr != pos {
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
    if s.iter().all(|&x| x == 0) {
        let mut ans: i64 = 1;
        for _ in pos..n {
            ans = (ans * 2) % MOD;
        }
        println!("{}", ans);
    } else {
        println!("0");
    }
}