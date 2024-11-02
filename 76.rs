use proconio::input;

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        a: [i64; n],
    }

    // Step #2. Create Array B
    let mut b = vec![0; 2 * n + 1];
    for i in 1..=n {
        b[i] = b[i - 1] + a[i - 1];
    }
    for i in 1..=n {
        b[i + n] = b[i + n - 1] + a[i - 1];
    }
    
    // Check if the total sum is not divisible by 10
    if b[n] % 10 != 0 {
        println!("No");
        return;
    }

    // Step #3. Get Answer
    for i in 0..=n {
        let goal = b[i] + b[n] / 10;
        // Binary search to find the position
        let pos1 = b.binary_search(&goal).unwrap_or_else(|x| x);
        if pos1 < b.len() && b[pos1] == goal {
            println!("Yes");
            return;
        }
    }
    
    println!("No");
}
