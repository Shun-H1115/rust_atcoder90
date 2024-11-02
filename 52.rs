use proconio::input;

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        a: [[i64; 6]; n],
    }

    let mut answer = 1;

    // Compute the answer by multiplying sums of each row
    for i in 0..n {
        let sum: i64 = a[i].iter().sum();
        answer = (answer * sum) % MOD;
    }

    println!("{}", answer);
}
