// ==========================================================================
// 典型90 #089  Partitions and Inversions  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ck
// ==========================================================================
// 【アルゴリズム】 しゃくとり法＋BIT で「転倒数 K 以下となる最左 cl[r]」を求め、累積和で DP を O(1) 遷移
// 【計算量】       O(N log N)
// 【学習計画】     第8週以降（★7。BIT(#017) と座標圧縮の復習として読む）
//
// 【このファイルで覚えるRust文法】
//   - 座標圧縮の定番: clone → sort_unstable → dedup → binary_search（Py: sorted(set(a)) + bisect）
//   - v.dedup() … 連続する重複を除去（ソート済みなら Py の set 化と同じ効果）
//   - isize で -1 を許す BIT の add/sum（負の添字は無視する設計）
//   - i &= i - 1 … 最下位ビットを消す（i -= i & -i と同じ）
//   - rem_euclid(MOD) … 累積和の差が負になる場合の正規化
//
// 【Pythonで書くと（考え方の対応）】
//   vals = sorted(set(A)); comp = [bisect_left(vals, x) for x in A]
//   # cl[r] = [cl[r], r] の転倒数が K 以下となる最小の左端（しゃくとり＋BIT）
//   # dp[i] = Σ_{j=cl[i]-1}^{i-1} dp[j]  → 累積和 ru で O(1)
// ==========================================================================

use proconio::input;

const MOD: i64 = 1_000_000_007;

struct BIT {
    n: usize,       // 1-based 内部配列の最大添字
    bit: Vec<i64>,  // 長さ n+1（0番は未使用）
}
impl BIT {
    fn new(size: usize) -> Self {
        Self { n: size, bit: vec![0; size + 1] }
    }
    // add at original index idx (0-based). Ignore if idx < 0
    // idx: isize … 呼び出し側で idx - 1 が -1 になっても受け取れるように符号付き。
    fn add(&mut self, idx: isize, delta: i64) {
        if idx < 0 { return; }
        let mut i = (idx + 1) as usize; // 1-based
        while i <= self.n {
            self.bit[i] += delta;
            // i & (!i + 1) … 最下位ビット（Py: i & -i）。usize では !i + 1 が 2 の補数の -i になる。
            i += i & (!i + 1); // i += i & -i
        }
    }
    // sum of [0..=idx] for 0-based idx. If idx < 0 => 0. If idx >= n-1 => total
    fn sum(&self, idx: isize) -> i64 {
        if idx < 0 {
            return 0;
        }
        let mut i = (idx + 1) as usize; // 1-based
        if i > self.n {
            i = self.n;
        }
        let mut s = 0i64;
        while i >= 1 {
            s += self.bit[i];
            // i &= i - 1 … 最下位の 1 を消す。BIT の sum で頻出の書き方。
            i &= i - 1; // i -= i & -i
        }
        s
    }
    // sum of all elements
    fn total(&self) -> i64 {
        self.sum(self.n as isize)
    }
}

fn main() {
    input! {
        n: usize,
        k: i64,
        a_in: [i64; n],
    }

    // 1-indexed 配列へ
    let mut a = vec![0i64; n + 1];
    for i in 1..=n {
        a[i] = a_in[i - 1];
    }

    // 座標圧縮（0-based 値にする）
    // a_in.clone() でコピーしてからソート（a_in は後で使うため）。
    let mut v = a_in.clone();
    v.sort_unstable();
    // dedup は「連続する」重複だけを消す。ソート後に呼ぶことで全体の重複が消える。
    v.dedup();
    let m = v.len(); // 圧縮後の値域は 0..m-1

    let mut comp = vec![0isize; n + 1];
    for i in 1..=n {
        // 値は必ず v に存在するので binary_search は Ok → unwrap で位置（圧縮後の値）を取り出す。
        let idx = v.binary_search(&a[i]).unwrap();
        comp[i] = idx as isize; // 0-based
    }

    // 前処理: cl[r] を求める（C++ と同じロジックを BIT で）
    let mut cl = vec![0usize; n + 1];
    let mut bit = BIT::new(m + 5); // 余裕を持たせる
    let mut l: isize = n as isize;
    let mut cnt: i64 = 0;

    // 最初に A[N] を入れておく（C++ 準拠）
    bit.add(comp[n], 1);

    for r in (1..=n).rev() {
        // しゃくとり: 転倒数が K を超えるまで左端を伸ばす。
        while l >= 1 && cnt <= k {
            l -= 1;
            if l >= 1 {
                // ★ l は isize なので usize に直してから comp[ll] を使う
                let ll = l as usize;
                let idx = comp[ll];          // isize（0-based 値）
                // 新しく左に追加した要素より小さい要素の数 = 増える転倒数。
                cnt += bit.sum(idx - 1);     // idx が 0 のときは -1 → sum は 0 を返す
                bit.add(idx, 1);
            }
        }
        // r の要素を取り除く
        bit.add(comp[r], -1);
        // 現在の多重集合中で comp[r] より大きい個数を引く
        // 右端 r を除くとき、r より大きい要素の数だけ転倒数が減る。
        let greater = bit.total() - bit.sum(comp[r]);
        cnt -= greater;

        cl[r] = if l >= 0 { l as usize } else { 0 };
    }

    // DP（累積和で O(1) 遷移）
    let mut dp = vec![0i64; n + 1];
    let mut ru = vec![0i64; n + 1];
    dp[0] = 1;
    ru[0] = 1;

    for i in 1..=n {
        dp[i] = if cl[i] == 0 {
            ru[i - 1]
        } else {
            // 累積和の差。負になりうるので rem_euclid。
            (ru[i - 1] - ru[cl[i] - 1]).rem_euclid(MOD)
        };
        ru[i] = (ru[i - 1] + dp[i]) % MOD;
    }

    println!("{}", dp[n] % MOD);
}
