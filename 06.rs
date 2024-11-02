use proconio::input;

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        k: usize,
        s: String,
    }

    let s_chars: Vec<char> = s.chars().collect();
    let mut nex = vec![vec![n; 26]; n + 1];

    // Step #2. Precompute positions for each character
    for i in 0..26 {
        nex[n][i] = n;
    }
    for i in (0..n).rev() {
        for j in 0..26 {
            nex[i][j] = if (s_chars[i] as u8 - b'a') as usize == j {
                i
            } else {
                nex[i + 1][j]
            };
        }
    }

    // Step #3. Greedily build the answer one character at a time
    let mut answer = String::new();
    let mut current_pos = 0;
    for i in 1..=k {
        for j in 0..26 {
            let nex_pos = nex[current_pos][j];
            let max_possible_length = s.len() - nex_pos - 1 + i;
            if max_possible_length >= k {
                answer.push((b'a' + j as u8) as char);
                current_pos = nex_pos + 1;
                break;
            }
        }
    }

    // Step #4. Output the result
    println!("{}", answer);
}