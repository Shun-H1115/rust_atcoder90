use proconio::input;

const NEG_INF: i64 = -(1i64 << 60);

struct RangeMax {
    size: usize,      // セグ木の底のサイズ（2の冪）
    dat: Vec<i64>,    // 根=1 の完全二分木表現
}
impl RangeMax {
    fn new() -> Self {
        Self { size: 1, dat: Vec::new() }
    }
    fn init(&mut self, sz: usize) {
        self.size = 1;
        while self.size <= sz {
            self.size <<= 1;
        }
        self.dat = vec![NEG_INF; self.size * 2]; // ← assign の代わりにこれ
    }
    fn update(&mut self, mut pos: usize, x: i64) {
        pos += self.size;
        self.dat[pos] = x;
        while pos >= 2 {
            pos >>= 1;
            let l = self.dat[pos * 2];
            let r = self.dat[pos * 2 + 1];
            self.dat[pos] = l.max(r);
        }
    }
    // 区間 [l, r) の最大値
    fn query_range(&self, l: usize, r: usize) -> i64 {
        self.query_dfs(l, r, 0, self.size, 1)
    }
    fn query_dfs(&self, l: usize, r: usize, a: usize, b: usize, u: usize) -> i64 {
        if l <= a && b <= r {
            return self.dat[u];
        }
        if r <= a || b <= l {
            return NEG_INF;
        }
        let mid = (a + b) >> 1;
        let v1 = self.query_dfs(l, r, a, mid, u * 2);
        let v2 = self.query_dfs(l, r, mid, b, u * 2 + 1);
        v1.max(v2)
    }
}

fn main() {
    input! {
        w: usize,
        n: usize,
    }
    let mut l = vec![0usize; n + 1];
    let mut r = vec![0usize; n + 1];
    let mut v = vec![0i64; n + 1];
    for i in 1..=n {
        input! { li: usize, ri: usize, vi: i64 }
        l[i] = li;
        r[i] = ri;
        v[i] = vi;
    }

    // dp[i][j]: 先頭 i 個まで見て、総重み j を作る最大価値
    let mut dp = vec![vec![NEG_INF; w + 1]; n + 1];

    // 各行 i に対応するセグ木
    let mut z: Vec<RangeMax> = (0..=n).map(|_| RangeMax::new()).collect();
    for i in 0..=n {
        z[i].init(w + 2);
    }

    dp[0][0] = 0;
    z[0].update(0, 0);

    for i in 1..=n {
        // 使わない遷移
        for j in 0..=w {
            dp[i][j] = dp[i - 1][j];
        }
        // 使う遷移
        for j in 0..=w {
            let cl = if j >= r[i] { j - r[i] } else { 0 };
            let cr = if j + 1 >= l[i] { j + 1 - l[i] } else { 0 };
            if cl == cr { continue; }
            let best = z[i - 1].query_range(cl, cr);
            if best != NEG_INF {
                dp[i][j] = dp[i][j].max(best + v[i]);
            }
        }
        // セグ木反映
        for j in 0..=w {
            z[i].update(j, dp[i][j]);
        }
    }

    let ans = dp[n][w];
    if ans == NEG_INF {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}
