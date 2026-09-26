// ==========================================================================
// 典型90 #058  Original Calculator  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bf
// ==========================================================================
// 【アルゴリズム】 周期検出（状態は 0〜99999 の10万通りしかないので必ずループする。初回訪問時刻を記録）
// 【計算量】       O(10^5)
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - 関数内 const
//   - vec![-1; MOD] … 未訪問を -1 で表す（要素型は i32 に推論）
//   - 型の混在に注意: time_stamp は i32、k は i64、添字は usize … as による変換が多発する
//
// 【Pythonで書くと（考え方の対応）】
//   nxt = [(i + sum(map(int, str(i)))) % 100000 for i in range(100000)]
//   ts = [-1]*100000; pos = N; cnt = 0
//   while ts[pos] == -1: ts[pos] = cnt; pos = nxt[pos]; cnt += 1
//   cyc = cnt - ts[pos]
//   if K >= ts[pos]: K = (K - ts[pos]) % cyc + ts[pos]
//   print(ts.index(K))
//
// 【注意・改善ポイント】
//   ! i32 / i64 / usize が混在して as が多い。time_stamp と cnt を i64 に統一すると変換が減って読みやすくなる。
//     「最初に型をそろえて設計する」のは Rust で書くときの大事なコツ。
//   ! 最後の線形探索の代わりに、ループ中に history: Vec<usize> へ pos を push しておけば history[k] で O(1)。
// ==========================================================================

use proconio::input;

// 桁和。Py: sum(map(int, str(x)))
fn digit_sum(x: i32) -> i32 {
    let mut ans = 0;
    let mut num = x;
    // Calculate the sum of the digits of `num`
    while num > 0 {
        ans += num % 10;
        num /= 10;
    }
    ans
}

fn main() {
    const MOD: usize = 100000;
    input! {
        n: usize,
        mut k: i64,
    }

    // Create the next step array based on the digit sums
    let mut nxt = vec![0; MOD];
    for i in 0..MOD {
        // i as i32 で渡し、戻り値を as usize で戻す。digit_sum を usize 版にすれば変換不要。
        nxt[i] = (i + digit_sum(i as i32) as usize) % MOD;
    }

    // Record the first occurrence time for each position
    // -1 は「まだ訪れていない」の印。
    let mut time_stamp = vec![-1; MOD];
    let mut pos = n;
    let mut cnt = 0;

    // Detect the cycle by marking each visited position with its timestamp
    // 既に訪れた状態に戻ったらループ検出。
    while time_stamp[pos] == -1 {
        time_stamp[pos] = cnt;
        pos = nxt[pos];
        cnt += 1;
    }

    // Calculate cycle length and adjust `K` to be within the cycle range
    let cycle_length = cnt - time_stamp[pos];
    if k >= time_stamp[pos] as i64 {
        // 周期に入った後は K を周期で割った余りに縮める。
        k = (k - time_stamp[pos] as i64) % cycle_length as i64 + time_stamp[pos] as i64;
    }

    // Find the number corresponding to the adjusted step `K`
    let mut answer = -1;
    for i in 0..MOD {
        // k as i32 … i64 と i32 は直接比較できない。
        if time_stamp[i] == k as i32 {
            answer = i as i32;
            break;
        }
    }

    println!("{}", answer);
}
