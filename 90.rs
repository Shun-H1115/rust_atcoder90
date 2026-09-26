use ac_library::{convolution, ModInt998244353 as Mint};
use proconio::input;

fn polynomial_inverse(c: Vec<Mint>, l: usize) -> Vec<Mint> {
    // (C[0] + C[1]x + ... ) * P(x) == 1 (mod x^L)
    // を満たす P(x) を Newton 法で求める。
    //
    // 前提: C[0] == 1

    let n = c.len();

    let mut a = vec![Mint::new(1), Mint::new(0)];
    let mut level = 0usize;

    while (1usize << level) < l {
        let cs = std::cmp::min(2usize << level, n);

        let mut p = convolution(&a, &c[..cs]);

        let mut q = vec![Mint::new(0); 2usize << level];
        q[0] = Mint::new(1);

        for j in (1usize << level)..(2usize << level) {
            q[j] = -p[j];
        }

        a = convolution(&a, &q);
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

    let mut track = vec![n + s as i64 + 1];

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
        poly = convolution(&poly, &poly);

        if track[i] % 2 == 1 {
            poly.insert(0, Mint::new(0));
        } else {
            poly.push(Mint::new(0));
        }

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

    println!("{}", poly[s].val());
}