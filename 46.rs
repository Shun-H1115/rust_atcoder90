use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i32; n],
        b: [i32; n],
        c: [i32; n],
    }

    // Count occurrences of each remainder when divided by 46
    let mut am = vec![0_i64; 46];
    let mut bm = vec![0_i64; 46];
    let mut cm = vec![0_i64; 46];

    for &val in &a {
        am[(val % 46) as usize] += 1;
    }
    for &val in &b {
        bm[(val % 46) as usize] += 1;
    }
    for &val in &c {
        cm[(val % 46) as usize] += 1;
    }

    // Calculate the number of valid triples
    let mut ans = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ans += am[i] * bm[j] * cm[k];
                }
            }
        }
    }

    // Output the answer
    println!("{}", ans);
}
