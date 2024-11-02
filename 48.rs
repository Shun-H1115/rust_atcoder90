use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(i64, i64); n],
    }
    
    // Step #1. Populate the `vec` with values `B[i]` and `A[i] - B[i]`
    let mut vec = Vec::new();
    for (a, b) in ab {
        vec.push(b);
        vec.push(a - b);
    }
    
    // Step #2. Sort `vec` in descending order and sum the top `K` elements
    vec.sort_unstable_by(|a, b| b.cmp(a));  // Sorting in descending order
    let answer: i64 = vec.iter().take(k).sum();

    // Step #3. Output the answer
    println!("{}", answer);
}
