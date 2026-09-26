// ==========================================================================
// 典型90 #086  Snuke's Favorite Arrays  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ch
// ==========================================================================
// 【アルゴリズム】 ビットごとに独立（60桁それぞれで 2^N 通りの 0/1 割当を全探索し、条件を満たす数の積が答え）
// 【計算量】       O(60 × 2^N × Q)
// 【学習計画】     第6週（★5後半）
//
// 【このファイルで覚えるRust文法】
//   - ラベル付きループ 'outer: for ... { for ... { continue 'outer; } } … 外側ループを直接 continue（Py には無い！）
//   - （Py なら for-else 構文や all() で書くところ）
//   - 関数に複数のスライス &[usize] を渡す
//   - queries.into_iter().enumerate() で消費しながら (i, (xi, yi, zi, wi)) に分解
//
// 【Pythonで書くと（考え方の対応）】
//   ans = 1
//   for bit in range(60):
//       ways = 0
//       for mask in range(1 << N):
//           if all(((mask >> x | mask >> y | mask >> z) & 1) == (w >> bit & 1) for x, y, z, w in Q):
//               ways += 1
//       ans = ans * ways % MOD
//   print(ans)
// ==========================================================================

use proconio::input;

const MOD: i64 = 1_000_000_007;

fn count_assignments(n: usize, x: &[usize], y: &[usize], z: &[usize], wbits: &[i64]) -> i64 {
    let q = wbits.len();
    let mut ways: i64 = 0;
    let total = 1usize << n;
    // 'outer: … ループにラベルを付ける。内側から continue 'outer で外側の次の周回へ飛べる。
    'outer: for mask in 0..total {
        for j in 0..q {
            // A_x | A_y | A_z の該当ビットを計算。
            let b = (((mask >> x[j]) & 1) | ((mask >> y[j]) & 1) | ((mask >> z[j]) & 1)) as i64;
            if b != wbits[j] {
                // 条件違反 → この mask は不採用、外側ループの次の mask へ。Py の for-else / any で書く処理。
                continue 'outer;
            }
        }
        ways += 1;
    }
    ways
}

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, usize, usize, u64); q], // (X, Y, Z, W)
    }

    // 0-index the variables
    let mut x = vec![0usize; q];
    let mut y = vec![0usize; q];
    let mut z = vec![0usize; q];
    let mut w = vec![0u64; q];
    for (i, (xi, yi, zi, wi)) in queries.into_iter().enumerate() {
        x[i] = xi - 1;
        y[i] = yi - 1;
        z[i] = zi - 1;
        w[i] = wi;
    }

    let mut answer: i64 = 1;
    let mut wbits = vec![0i64; q];
    // 各ビットは独立なので、ビットごとの場合の数を掛け合わせる。
    for bit in 0..60 {
        for j in 0..q {
            wbits[j] = ((w[j] >> bit) & 1) as i64;
        }
        let ways = count_assignments(n, &x, &y, &z, &wbits);
        answer = (answer * (ways % MOD)) % MOD;
    }

    println!("{}", answer % MOD);
}
