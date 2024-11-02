use proconio::input;

fn main() {
    // Step #1: Input
    input! {
        n: usize,
        mut a: [i64; n],
        mut b: [i64; n],
    }

    // Step #2: Sort both arrays
    a.sort();
    b.sort();

    // Step #3: Calculate the sum of absolute differences
    let answer: i64 = a.iter().zip(b.iter()).map(|(ai, bi)| (ai - bi).abs()).sum();

    // Output the answer
    println!("{}", answer);
}
