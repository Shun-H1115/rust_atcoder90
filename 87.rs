use proconio::input;
use std::cmp::min;

fn count_number(lens: i64, n: usize, p: i64, a: &Vec<Vec<i64>>) -> i64 {
    // dist 初期化（a[i][j] == -1 を lens に置換）
    let mut dist = vec![vec![0i64; n + 1]; n + 1];
    for i in 1..=n {
        for j in 1..=n {
            dist[i][j] = if a[i][j] == -1 { lens } else { a[i][j] };
        }
    }

    // Floyd–Warshall
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
    let mut cr: i64 = 5_000_000_000;
    let mut minx: i64 = 5_000_000_000;
    // 元コードは 40 回固定反復の疑似二分探索
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

    if r - l >= 2_000_000_000 {
        println!("Infinity");
    } else {
        println!("{}", r - l);
    }
}