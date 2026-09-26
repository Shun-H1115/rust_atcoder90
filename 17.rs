// ==========================================================================
// 典型90 #017  Crossing Segments  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_q
// ==========================================================================
// 【アルゴリズム】 余事象＋BIT（全ペアから「交差しない組」= 端点共有・分離・包含 を引く）
// 【計算量】       O(M log N)
// 【学習計画】     第8週（★7 実装推奨5問の1つ）
//
// 【このファイルで覚えるRust文法】
//   - BIT（Fenwick Tree）の struct 実装 … Py でも書くが Rust は &self / &mut self を使い分ける
//   - !pos + 1 … ビット反転+1 = 2の補数 = -pos。pos & (!pos+1) で最下位ビット（Py の x & -x）
//   - （推奨）usize は負を持てないので pos & pos.wrapping_neg() と書くのが Rust らしい
//   - Vec<_> … 要素型の推論を任せる書き方
//
// 【Pythonで書くと（考え方の対応）】
//   # 交差する組 = 全組 - (端点共有 + 完全に離れている + 包含)
//   # 包含の数: r でソートして、自分の内側 (cl, cr) にある l の個数を BIT で数える
// ==========================================================================

use proconio::input;
use std::cmp::min;
use std::collections::VecDeque;

// Binary Indexed Tree。Py の class BIT と同じ構造。
struct BIT {
    size: usize,
    bit: Vec<i64>,
}

impl BIT {
    fn new(sz: usize) -> Self {
        BIT {
            size: sz + 2,
            bit: vec![0; sz + 3],
        }
    }

    // add は bit を書き換えるので &mut self。
    fn add(&mut self, pos: usize, x: i64) {
        let mut pos = pos + 1;
        while pos <= self.size {
            self.bit[pos] += x;
            // pos & (!pos + 1) … 最下位の1ビット（lowbit）。Py: pos & -pos
            pos += pos & (!pos + 1);
        }
    }

    // sum は読むだけなので &self（不変借用）。可変/不変を型で分けるのが Rust の特徴。
    fn sum(&self, pos: usize) -> i64 {
        let mut s = 0;
        let mut pos = pos + 1;
        // pos >= 1 … usize なので pos -= lowbit で 0 になって終了する（負にはならない）。
        while pos >= 1 {
            s += self.bit[pos];
            pos -= pos & (!pos + 1);
        }
        s
    }
}

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        m: usize,
        lr: [(usize, usize); m],
    }

    // Initialize vectors for calculations
    // v1, v2, v3 の要素型は後の answer との演算から i64 と推論される。
    let mut v1 = vec![0; n + 1];
    let mut v2 = vec![0; n + 1];
    let mut v3 = vec![0; n + 1];

    // Step #2. Get Answer1
    let mut answer1 = 0;
    for &(l, r) in &lr {
        v3[l] += 1;
        v3[r] += 1;
    }
    for i in 1..=n {
        // 同じ端点を共有する区間ペア数 = Σ C(v3[i], 2)
        answer1 += v3[i] * (v3[i] - 1) / 2;
    }

    // Step #3. Get Answer2
    let mut answer2 = 0;
    for &(_, r) in &lr {
        v1[r] += 1;
    }
    for &(l, _) in &lr {
        if l > 1 {
            v2[l - 1] += 1;
        }
    }
    for i in 1..=n {
        v1[i] += v1[i - 1];
        // v1[i] = 右端が i 以下の区間数（累積和）、v2[i] = 左端が i+1 の区間数 → 完全に分離したペア。
        answer2 += v1[i] * v2[i];
    }

    // Step #4. Sorting
    // (l, r) を (r, l) に入れ替えて r 昇順ソート。Py: sorted((r, l) for l, r in lr)
    let mut intervals: Vec<_> = lr.iter().map(|&(l, r)| (r, l)).collect();
    intervals.sort();

    // Step #5. Get Answer3
    let mut answer3 = 0;
    let mut bit = BIT::new(n);
    for &(cr, cl) in &intervals {
        // 既に追加した（r が小さい）区間のうち、l が (cl, cr] に入るもの = 自分に包含される区間。
        let ret = bit.sum(cr) - bit.sum(cl);
        answer3 += ret;
        bit.add(cl, 1);
    }

    // Step #6. Output The Answer!
    // m as i64 … usize のまま m*(m-1)/2 でも良いが、以後の i64 と合わせるため変換。
    let total = m as i64 * (m as i64 - 1) / 2;
    let sum_ans = answer1 + answer2 + answer3;
    println!("{}", total - sum_ans);
}
