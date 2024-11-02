use proconio::input;

fn main() {
    // Step #1: Read input
    input! {
        n: usize,
        q: usize,
        mut a: [i64; n], // initial heights
        queries: [(usize, usize, i64); q] // list of queries (L, R, V)
    }

    // Step #2: Initialize the initial inconvenience
    let mut b = vec![0; n - 1]; // differences between consecutive areas
    let mut inconvenience = 0;

    for i in 0..n - 1 {
        b[i] = a[i + 1] - a[i];
        inconvenience += b[i].abs();
    }

    // Step #3: Process each query and output the updated inconvenience
    for (l, r, v) in queries {
        let l = l - 1; // convert to 0-based index
        let r = r - 1; // convert to 0-based index

        // Calculate the effect before applying the change
        let before_change = if l > 0 { b[l - 1].abs() } else { 0 } + if r < n - 1 { b[r].abs() } else { 0 };

        // Update the differences array
        if l > 0 {
            b[l - 1] += v;
        }
        if r < n - 1 {
            b[r] -= v;
        }

        // Calculate the effect after applying the change
        let after_change = if l > 0 { b[l - 1].abs() } else { 0 } + if r < n - 1 { b[r].abs() } else { 0 };

        // Update the total inconvenience
        inconvenience += after_change - before_change;

        // Print the result for the current query
        println!("{}", inconvenience);
    }
}
