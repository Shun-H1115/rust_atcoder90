use proconio::input;

fn is_valid(s: &str) -> bool {
    let mut depth = 0;
    for ch in s.chars() {
        if ch == '(' {
            depth += 1;
        } else {
            depth -= 1;
        }
        if depth < 0 {
            return false;
        }
    }
    depth == 0
}

fn main() {
    input! {
        n: usize,
    }

    for i in 0..(1 << n) {
        let mut candidate = String::new();
        for j in (0..n).rev() {
            if (i & (1 << j)) == 0 {
                candidate.push('(');
            } else {
                candidate.push(')');
            }
        }

        if is_valid(&candidate) {
            println!("{}", candidate);
        }
    }
}
