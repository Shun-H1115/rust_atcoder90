use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
        b: [i64; n],
    }

    // Step #2. Calculate the absolute difference sum
    let mut diff = 0;
    for i in 0..n {
        diff += (a[i] - b[i]).abs();
    }

    // Step #3. Check if the total difference exceeds K
    if diff > k {
        println!("No");
        return;
    }

    // Step #4. Check parity
    if diff % 2 != k % 2 {
        println!("No");
        return;
    }

    // Step #5. Output Yes if both conditions are satisfied
    println!("Yes");
}
