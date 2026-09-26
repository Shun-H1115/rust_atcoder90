// ==========================================================================
// 典型90 #037  Don't Leave the Spice  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ak
// ==========================================================================
// 【アルゴリズム】 ナップサックDP＋セグ木（遷移元が区間 [j-R, j-L] なので区間 max をセグ木で高速化）
// 【計算量】       O(NW log W)
// 【学習計画】     第5週（★5前半）
//
// 【このファイルで覚えるRust文法】
//   - 非再帰の点更新（ボトムアップ）＋再帰の区間取得（トップダウン）のセグ木
//   - self.size <<= 1 … Py: self.size <<= 1（同じ）
//   - (0..=n).map(|_| RangeMax::new()).collect() … struct の Vec を作る
//   - l.max(r) … i64 のメソッド版 max
//   - if 式でアンダーフローを回避: if j >= r { j - r } else { 0 }（Py の max(0, j - r)）
//
// 【Pythonで書くと（考え方の対応）】
//   # dp[i][j] = max(dp[i-1][j], max(dp[i-1][j-R..j-L]) + V)
//   # 区間 max をセグ木（or スライド最大値）で O(log W) に
//
// 【注意・改善ポイント】
//   ! j - R が負になるかを if で判定しているのは usize のため。j.saturating_sub(r) と書くと
//     「0 未満なら 0」に丸める引き算になり1行で済む（Py の max(0, j - r) と同じ）。
//   ! N 本のセグ木を全部保持しているが、使うのは直前の1本だけ。2本を交互に使えばメモリを節約できる。
// ==========================================================================

use proconio::input;

const NEG_INF: i64 = -(1i64 << 60);

struct RangeMax {
    size: usize,      // セグ木の底のサイズ（2の冪）
    dat: Vec<i64>,    // 根=1 の完全二分木表現
}
impl RangeMax {
    fn new() -> Self {
        // 空のセグ木を作り、あとで init でサイズを決める2段階初期化（C++ の移植スタイル）。
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
        // 葉の位置 = pos + size。そこから親へ上りながら更新。
        pos += self.size;
        self.dat[pos] = x;
        while pos >= 2 {
            // pos >>= 1 … 親ノードへ（Py の //= 2）。
            pos >>= 1;
            let l = self.dat[pos * 2];
            let r = self.dat[pos * 2 + 1];
            self.dat[pos] = l.max(r);
        }
    }
    // 区間 [l, r) の最大値
    // &self … 取得は読むだけなので不変借用でよい（#029 の遅延セグ木とは異なる）。
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
    // RangeMax は Clone を derive していないので vec![...; n] は使えない。map + collect で作る。
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
            // usize の j - r[i] が負にならないよう if で分岐。j.saturating_sub(r[i]) と同じ。
            let cl = if j >= r[i] { j - r[i] } else { 0 };
            let cr = if j + 1 >= l[i] { j + 1 - l[i] } else { 0 };
            if cl == cr { continue; }
            // 遷移元の区間 [cl, cr) の最大値。
            let best = z[i - 1].query_range(cl, cr);
            // NEG_INF のまま = 到達不能なので遷移しない（NEG_INF + v で「少し大きい負」になるのを防ぐ）。
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
