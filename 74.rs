use proconio::input;

fn main() {
    // Input
    input! {
        n: usize,
        s: String,
    }

    // Initialize answer to zero
    let mut answer: i64 = 0;

    // Calculate answer by iterating over each character in the string
    for (i, ch) in s.chars().enumerate() {
        if ch == 'b' {
            // If character is 'b', add 1 * 2^i to the answer
            answer += 1 << i;
        } else if ch == 'c' {
            // If character is 'c', add 2 * 2^i to the answer
            answer += 2 << i;
        }
    }

    // Output the final answer
    println!("{}", answer);
}
