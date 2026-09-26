// ==========================================================================
// 典型90 #053 Discrete Dowsing（★7） 修正版
// 【修正点】全位置に質問する O(N) 回の方式 → フィボナッチ探索（1テストあたり15回）
//   元のコードは i = 1..=N すべてに "? i" を送っていたため、質問回数の上限（15回）を超える。
//   A は単峰（ある位置まで増加し、その後減少）なので、フィボナッチ探索で最大値を求められる。
//
// 【考え方】
//   - 区間 [l, r] の長さを常にフィボナッチ数 F[k] に保つ。
//   - 内分点 x1 = l + F[k-2], x2 = l + F[k-1] の値を比べ、小さい側の外側を捨てる。
//   - 残った区間の内分点の片方は、前回調べた点と一致する → 1回の質問で区間が F[k] → F[k-1] に縮む。
//   - F[16] = 1597 >= N + 1 なので、1..=1596 の仮想配列で探索する。
//     N より右の位置は「質問せずに、右へ行くほど小さくなる十分小さい値」を返す（単峰性を保つ）。
//   - 質問回数: 最初に2回 + k = 16 → 3 まで13回 = 15回。
// ==========================================================================

use std::io::{self, Read, Write};

fn read_token<T: std::str::FromStr>() -> T {
    // 1トークンだけ標準入力から読み取る（空白区切り）。インタラクティブなので1バイトずつ読む
    let mut buf = Vec::with_capacity(32);
    let mut byte = [0u8; 1];
    let mut started = false;
    loop {
        let n = io::stdin().read(&mut byte).unwrap();
        if n == 0 {
            break;
        }
        let b = byte[0];
        if b.is_ascii_whitespace() {
            if started {
                break;
            }
        } else {
            started = true;
            buf.push(b);
        }
    }
    // [修正] unsafe な from_utf8_unchecked をやめて通常の from_utf8 に
    let s = std::str::from_utf8(&buf).unwrap();
    s.parse().ok().unwrap()
}

// 質問のメモ化つきラッパー（同じ位置に2回質問しない）
struct Oracle {
    n: usize,
    memo: Vec<Option<i64>>,
}

impl Oracle {
    fn new(n: usize, len: usize) -> Self {
        Oracle { n, memo: vec![None; len + 1] }
    }

    fn get(&mut self, pos: usize) -> i64 {
        // 範囲外（0 や N より右）は質問しない。右に行くほど小さい値にして単峰性を保つ
        if pos == 0 || pos > self.n {
            return -1_000_000_000_000 - pos as i64;
        }
        if let Some(v) = self.memo[pos] {
            return v;
        }
        println!("? {}", pos);
        io::stdout().flush().unwrap(); // インタラクティブ問題では flush 必須
        let v: i64 = read_token();
        self.memo[pos] = Some(v);
        v
    }
}

fn solve() {
    let n: usize = read_token();

    // fib[0] = 1, fib[1] = 1, fib[2] = 2, ..., fib[16] = 1597
    let mut fib = vec![1usize; 17];
    for i in 2..17 {
        fib[i] = fib[i - 1] + fib[i - 2];
    }

    let mut k = 16;
    let mut l = 0usize;
    let mut r = fib[k]; // 常に r - l == fib[k]
    let mut oracle = Oracle::new(n, r);

    let mut x1 = l + fib[k - 2];
    let mut x2 = l + fib[k - 1];
    let mut v1 = oracle.get(x1);
    let mut v2 = oracle.get(x2);

    while r - l > 3 {
        if v1 > v2 {
            // 最大値は (l, x2) にある → 右側を捨てる
            r = x2;
            k -= 1;
            x2 = x1;
            v2 = v1;
            x1 = l + fib[k - 2];
            v1 = oracle.get(x1);
        } else {
            // 最大値は (x1, r) にある → 左側を捨てる
            l = x1;
            k -= 1;
            x1 = x2;
            v1 = v2;
            x2 = l + fib[k - 1];
            v2 = oracle.get(x2);
        }
    }

    // 残りの区間 (l, r) の内部は x1, x2 だけで、どちらも質問済み（追加の質問は発生しない）
    let mut answer = v1.max(v2);
    for pos in (l + 1)..r {
        answer = answer.max(oracle.get(pos));
    }

    println!("! {}", answer);
    io::stdout().flush().unwrap();
}

fn main() {
    let t: usize = read_token();
    for _ in 0..t {
        solve();
    }
}
