// ==========================================================================
// 典型90 #090  Tenka1 Programming Contest  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_cl
// ==========================================================================
// 【アルゴリズム】 形式的冪級数（多項式の逆元をニュートン法で求める）＋ Kitamasa 法風の線形漸化式の高速計算
// 【計算量】       O(K log K × log N) 程度
// 【学習計画】     範囲外（★7 最難関。2か月の学習後、余力があれば挑戦）
//
// 【このファイルで覚えるRust文法】
//   - use ac_library::{convolution, ModInt998244353 as Mint}; … as で別名インポート
//   - &c[..cs] … スライスの先頭 cs 要素（Py: c[:cs]）
//   - -p[j] … ModInt は単項マイナス（Neg トレイト）も実装済み
//   - a.resize(len, value) … 長さを変える（伸ばす部分は value で埋める。Py: a = a[:L] + [0]*(L - len(a))）
//   - *track.last().unwrap() … 末尾要素（Py の track[-1]）。last() は Option<&T>
//   - poly[(s + 1)..].to_vec() … スライスをコピーして新しい Vec に
//   - poly[s].val() … ModInt から整数値を取り出す
//
// 【Pythonで書くと（考え方の対応）】
//   # 1/C(x) mod x^L をニュートン法: A_{2m} = A_m (2 - C A_m)  mod x^{2m}
//   # 巨大な N に対し「x^N mod 特性多項式」をダブリングで計算して答えを得る
//
// 【注意・改善ポイント】
//   ! let mut p = convolution(...) の mut は不要（warning）。
//   ! c: Vec<Mint> を値で受け取っているので、呼び出し側で cl.clone() が必要になっている。
//     引数を c: &[Mint] にすれば clone が不要になる（所有権の設計の練習にちょうど良い）。
// ==========================================================================

// as Mint で ModInt998244353 に短い別名を付けて import。
use ac_library::{convolution, ModInt998244353 as Mint};
use proconio::input;

// c を値で受け取る（move）。読むだけなら &[Mint] で十分。
fn polynomial_inverse(c: Vec<Mint>, l: usize) -> Vec<Mint> {
    // (C[0] + C[1]x + ... ) * P(x) == 1 (mod x^L)
    // を満たす P(x) を Newton 法で求める。
    //
    // 前提: C[0] == 1

    let n = c.len();

    let mut a = vec![Mint::new(1), Mint::new(0)];
    let mut level = 0usize;

    // 精度を倍々にしていくニュートン法（1 → 2 → 4 → ...項）。
    while (1usize << level) < l {
        let cs = std::cmp::min(2usize << level, n);

        // &c[..cs] … 先頭 cs 項だけのスライス。&[Mint] として convolution に渡る。
        let mut p = convolution(&a, &c[..cs]);

        let mut q = vec![Mint::new(0); 2usize << level];
        q[0] = Mint::new(1);

        for j in (1usize << level)..(2usize << level) {
            q[j] = -p[j];
        }

        a = convolution(&a, &q);
        // resize で長さを 4·2^level に（不足分は 0 埋め、超過分は切り捨て）。
        a.resize(4usize << level, Mint::new(0));

        level += 1;
    }

    a.resize(l, Mint::new(0));
    a
}

fn main() {
    input! {
        n: i64,
        k: usize,
    }

    // k+1 個の空の多項式。
    let mut dp: Vec<Vec<Mint>> = vec![Vec::new(); k + 1];

    dp[k] = vec![
        Mint::new(1),
        Mint::new(1),
        Mint::new(1),
    ];

    for i in (1..k).rev() {
        let limit = std::cmp::min(k / i, n as usize);

        let mut c = vec![Mint::new(0); dp[i + 1].len()];
        c[0] = Mint::new(1);

        for j in 1..dp[i + 1].len() {
            c[j] = -dp[i + 1][j];
        }

        let g = polynomial_inverse(c, limit + 2);
        dp[i] = g;
    }

    let s = std::cmp::min(k, n as usize);

    // n + s as i64 + 1 … as は + より優先度が高いので n + (s as i64) + 1 の意味。
    let mut track = vec![n + s as i64 + 1];

    // *track.last().unwrap() … last() は Option<&i64>。unwrap で &i64、* で値を取り出す。
    while *track.last().unwrap() >= s as i64 + 1 {
        let x = *track.last().unwrap();
        track.push(x / 2);
    }

    track.reverse();

    let mut cl = vec![Mint::new(0); s + 2];
    cl[0] = Mint::new(1);

    for i in 1..s + 2 {
        cl[i] = -dp[1][i];
    }

    let gl = polynomial_inverse(cl.clone(), s + 2);

    cl.reverse();

    let mut poly = vec![Mint::new(0); s + 1];
    poly[track[0] as usize] = Mint::new(1);

    for i in 1..track.len() {
        // 多項式の2乗（畳み込み）。
        poly = convolution(&poly, &poly);

        if track[i] % 2 == 1 {
            poly.insert(0, Mint::new(0));
        } else {
            poly.push(Mint::new(0));
        }

        // [s+1..] 以降をコピー。to_vec() でスライス → 所有する Vec。
        let mut p1 = poly[(s + 1)..].to_vec();
        p1.reverse();

        let mut p2 = convolution(&p1, &gl);
        p2.resize(s + 1, Mint::new(0));
        p2.reverse();

        let p3 = convolution(&p2, &cl);

        for j in 0..(2 * s + 1) {
            poly[j] -= p3[j];
        }

        poly.resize(s + 1, Mint::new(0));
    }

    // val() で u32 の値を取り出して表示。
    println!("{}", poly[s].val());
}
