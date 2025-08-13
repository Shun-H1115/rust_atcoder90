use proconio::input;

fn main() {
    input! {
        n: usize,
        pts: [(i64, i64); n],
    }

    let mut xs: Vec<i64> = pts.iter().map(|&(x, _)| x).collect();
    let mut ys: Vec<i64> = pts.iter().map(|&(_, y)| y).collect();

    let mid = n / 2;
    xs.select_nth_unstable(mid);
    ys.select_nth_unstable(mid);
    let mx = xs[mid];
    let my = ys[mid];

    let mut ans: i128 = 0;
    for &(x, y) in &pts {
        ans += (x - mx).abs() as i128;
        ans += (y - my).abs() as i128;
    }

    println!("{}", ans);
}