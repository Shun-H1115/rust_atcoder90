use proconio::input;

fn main() {
    input! {
        n: usize,
        s: String,
    }

    let mut cnt = 0;
    let mut vec = Vec::new();

    // Count consecutive characters
    for i in 0..s.len() {
        cnt += 1;
        if i == s.len() - 1 || s.as_bytes()[i] != s.as_bytes()[i + 1] {
            vec.push((s.as_bytes()[i] as char, cnt));
            cnt = 0;
        }
    }

    let mut ret = 0;
    for (_, count) in vec {
        // Calculate the sum of pairs in the consecutive characters
        ret += count * (count + 1) / 2;
    }

    // Total pairs minus the pairs of consecutive characters
    println!("{}", n * (n + 1) / 2 - ret);
}
