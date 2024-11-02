use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = HashMap::new();

    for (i, word) in s.iter().enumerate() {
        if !map.contains_key(word) {
            map.insert(word, true);
            println!("{}", i + 1);
        }
    }
}
