// ==========================================================================
// 典型90 #087  Chokudai's Demand  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ci
// ==========================================================================
// 【アルゴリズム】 二分探索＋ワーシャルフロイド（未定の辺長 X を大きくするほど「距離 P 以下の組数」は単調減少 → 境界を2回二分探索）
// 【計算量】       O(N^3 log(上限))
// 【学習計画】     第6週（★5後半）
//
// 【このファイルで覚えるRust文法】
//   - ワーシャルフロイドの3重ループ（Py と同じ構造。Rust なら N=40 × 40回 でも余裕）
//   - a: &Vec<Vec<i64>> … 2次元配列を借用で受け取る（&[Vec<i64>] の方が一般的）
//   - 5_000_000_000 … i32 に収まらない数は i64 型の変数に入れる（型注釈 : i64 が必須）
//
// 【Pythonで書くと（考え方の対応）】
//   def count(X):
//       d = [[X if a == -1 else a for a in row] for row in A]
//       for k in range(N):
//           for i in range(N):
//               for j in range(N): d[i][j] = min(d[i][j], d[i][k] + d[k][j])
//       return sum(d[i][j] <= P for i in range(N) for j in range(i+1, N))
//   # 「count(X) <= K となる最小の X」と「count(X) <= K-1 となる最小の X」の差が答え
// ==========================================================================

use proconio::input;
use std::cmp::min;

fn count_number(lens: i64, n: usize, p: i64, a: &Vec<Vec<i64>>) -> i64 {
    // dist 初期化（a[i][j] == -1 を lens に置換）
    let mut dist = vec![vec![0i64; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            // -1 の辺（未定）を X に置き換える。if 式で値を選ぶ。
            dist[i][j] = if a[i][j] == -1 { lens } else { a[i][j] };
        }
    }

    // Floyd–Warshall
    // k を一番外側にするのがワーシャルフロイドの鉄則（順番を間違えると不正解）。
    for k in 1..=n {
        for i in 1..=n {
            for j in 1..=n {
                let cand = dist[i][k] + dist[k][j];
                if cand < dist[i][j] {
                    dist[i][j] = cand;
                }
            }
        }
    }

    // i < j で dist[i][j] <= P の個数
    let mut cnt: i64 = 0;
    for i in 1..=n {
        for j in (i + 1)..=n {
            if dist[i][j] <= p {
                cnt += 1;
            }
        }
    }
    cnt
}

fn get_border(cnts: i64, n: usize, p: i64, a: &Vec<Vec<i64>>) -> i64 {
    let mut cl: i64 = 1;
    // 上限 5e9。i32 の上限（約 2.1e9）を超えるので i64 が必要。
    let mut cr: i64 = 5_000_000_000;
    let mut minx: i64 = 5_000_000_000;
    // 元コードは 40 回固定反復の疑似二分探索
    // while 条件でなく回数固定の二分探索。範囲が 5e9 なら 33 回程度で収束するので 40 回で十分。
    for _ in 0..40 {
        let cm = (cl + cr) / 2;
        let res = count_number(cm, n, p, a);
        if res <= cnts {
            cr = cm;
            minx = min(minx, cm);
        } else {
            cl = cm;
        }
    }
    minx
}

fn main() {
    input! {
        n: usize,
        p: i64,
        k: i64,
    }

    // 1-indexed 行列
    let mut a = vec![vec![0i64; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            input! { v: i64 }
            a[i][j] = v;
        }
    }

    let l = get_border(k, n, p, &a);
    let r = get_border(k - 1, n, p, &a);

    // 境界の差が上限付近 → X をいくら大きくしても条件を満たし続ける（無限個）。
    if r - l >= 2_000_000_000 {
        println!("Infinity");
    } else {
        println!("{}", r - l);
    }
}
