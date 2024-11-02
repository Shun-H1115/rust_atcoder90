use proconio::input;
use std::cmp::max;

fn main() {
    // Step #1: Input
    input! {
        n: usize,
        tasks: [(usize, usize, i64); n], // Each task has (D, C, S) for Deadline, Cost, Score
    }

    // Sort tasks by their deadline to maximize chances of completion
    let mut tasks = tasks;
    tasks.sort_by_key(|&(d, _, _)| d);

    // Initialize DP table
    let mut dp = vec![vec![0; 5001]; n + 1];

    // Step #2: Dynamic Programming
    for i in 0..n {
        let (d, c, s) = tasks[i];
        for j in 0..=5000 {
            // Option 1: Skip this task
            dp[i + 1][j] = max(dp[i + 1][j], dp[i][j]);
            
            // Option 2: Take this task if it can be completed on time
            if j + c <= d {
                dp[i + 1][j + c] = max(dp[i + 1][j + c], dp[i][j] + s);
            }
        }
    }

    // Step #3: Output the maximum reward
    let answer = dp[n].iter().max().unwrap();
    println!("{}", answer);
}
