use proconio::input;
use std::collections::HashMap;
use std::cmp::max;

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        k: usize,
        a: [i32; n],
    }

    // Step #2. Sliding Window (Shakutori method)
    let mut answer = 0;
    let mut count_map = HashMap::new();
    let mut cr = 0;
    let mut distinct_count = 0;

    for i in 0..n {
        while cr < n {
            let entry = count_map.entry(a[cr]).or_insert(0);
            if *entry == 0 && distinct_count == k {
                break;
            }
            if *entry == 0 {
                distinct_count += 1;
            }
            *entry += 1;
            cr += 1;
        }
        answer = max(answer, cr - i);
        if let Some(count) = count_map.get_mut(&a[i]) {
            *count -= 1;
            if *count == 0 {
                distinct_count -= 1;
            }
        }
    }

    // Step #3. Output the Answer
    println!("{}", answer);
}
