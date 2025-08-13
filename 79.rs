use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[i64; w]; h],
        b: [[i64; w]; h],
    }

    let mut ans: i64 = 0;

    // Apply 2x2 adjustments left-to-right, top-to-bottom
    for i in 0..h.saturating_sub(1) {
        for j in 0..w.saturating_sub(1) {
            let d = b[i][j] - a[i][j];
            a[i][j] += d;
            a[i][j + 1] += d;
            a[i + 1][j] += d;
            a[i + 1][j + 1] += d;
            ans += d.abs();
        }
    }

    if a == b {
        println!("Yes");
        println!("{}", ans);
    } else {
        println!("No");
    }
}