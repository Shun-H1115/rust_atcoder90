use proconio::input;

const MOD: i64 = 1_000_000_007;

fn count_assignments(n: usize, x: &[usize], y: &[usize], z: &[usize], wbits: &[i64]) -> i64 {
    let q = wbits.len();
    let mut ways: i64 = 0;
    let total = 1usize << n;
    'outer: for mask in 0..total {
        for j in 0..q {
            let b = (((mask >> x[j]) & 1) | ((mask >> y[j]) & 1) | ((mask >> z[j]) & 1)) as i64;
            if b != wbits[j] {
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
    for bit in 0..60 {
        for j in 0..q {
            wbits[j] = ((w[j] >> bit) & 1) as i64;
        }
        let ways = count_assignments(n, &x, &y, &z, &wbits);
        answer = (answer * (ways % MOD)) % MOD;
    }

    println!("{}", answer % MOD);
}