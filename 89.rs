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
    fn add(&mut self, idx: isize, delta: i64) {
        if idx < 0 { return; }
        let mut i = (idx + 1) as usize; // 1-based
        while i <= self.n {
            self.bit[i] += delta;
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
    let mut v = a_in.clone();
    v.sort_unstable();
    v.dedup();
    let m = v.len(); // 圧縮後の値域は 0..m-1

    let mut comp = vec![0isize; n + 1];
    for i in 1..=n {
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
        while l >= 1 && cnt <= k {
            l -= 1;
            if l >= 1 {
                // ★ l は isize なので usize に直してから comp[ll] を使う
                let ll = l as usize;
                let idx = comp[ll];          // isize（0-based 値）
                cnt += bit.sum(idx - 1);     // idx が 0 のときは -1 → sum は 0 を返す
                bit.add(idx, 1);
            }
        }
        // r の要素を取り除く
        bit.add(comp[r], -1);
        // 現在の多重集合中で comp[r] より大きい個数を引く
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
            (ru[i - 1] - ru[cl[i] - 1]).rem_euclid(MOD)
        };
        ru[i] = (ru[i - 1] + dp[i]) % MOD;
    }

    println!("{}", dp[n] % MOD);
}
