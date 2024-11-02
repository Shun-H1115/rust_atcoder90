use proconio::input;
use std::cmp::Reverse;

fn dfs(pos: i32, last: i32, current_str: &mut String, n: i64, b: i64, answer: &mut i64) {
    if pos == 0 {
        let product: i64 = current_str.chars().map(|c| (c as i64 - '0' as i64)).product();
        let goal = product + b;
        if goal <= n {
            let mut goal_str: Vec<char> = goal.to_string().chars().collect();
            goal_str.sort_by_key(|&c| Reverse(c));
            if goal_str.into_iter().collect::<String>() == *current_str {
                *answer += 1;
            }
        }
        return;
    }

    for i in (1..=last).rev() {
        current_str.push((i as u8 + b'0') as char);
        dfs(pos - 1, i, current_str, n, b, answer);
        current_str.pop();
    }
}

fn main() {
    input! {
        n: i64,
        b: i64,
    }

    let mut answer = 0;
    for length in 1..=11 {
        dfs(length, 9, &mut String::new(), n, b, &mut answer);
    }

    // Special case for sequences that contain only zeros
    if b.to_string().chars().any(|c| c == '0') && n >= b {
        answer += 1;
    }

    println!("{}", answer);
}
