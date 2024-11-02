use proconio::input;
use std::collections::BTreeSet;

fn main() {
    // Read input
    input! {
        n: usize,
        q: usize,
        queries: [(i32, usize, usize, i64); q],
    }

    // Initialize values
    let mut sum = vec![0; n - 1];
    let mut t_vec = Vec::new();
    let mut x_vec = Vec::new();
    let mut y_vec = Vec::new();
    let mut v_vec = Vec::new();

    // Parse each query and update sum for type 0 queries
    for (t, x, y, v) in &queries {
        if *t == 0 {
            sum[x - 1] = *v;
        }
        t_vec.push(*t);
        x_vec.push(x - 1);
        y_vec.push(y - 1);
        v_vec.push(*v);
    }

    // Calculate potential values
    let mut pot = vec![0_i64; n];
    for i in 0..n - 1 {
        pot[i + 1] = sum[i] as i64 - pot[i];
    }

    // Set up a BTreeSet to track unset values
    let mut unset_indices = BTreeSet::new();
    for i in 0..=n {
        unset_indices.insert(i as i32 - 1);
    }

    // Process each query and produce results
    for i in 0..q {
        if t_vec[i] == 0 {
            unset_indices.remove(&(x_vec[i] as i32));
        } else {
            let p = x_vec[i].min(y_vec[i]);
            let q = x_vec[i].max(y_vec[i]);
            let lower_bound = unset_indices.range(p as i32..).next();
            
            // Determine the output based on index ranges
            if let Some(&lb) = lower_bound {
                if lb > (q - 1) as i32 {
                    let result = if (q - p) % 2 == 0 {
                        pot[y_vec[i]] + (v_vec[i] - pot[x_vec[i]])
                    } else {
                        pot[y_vec[i]] - (v_vec[i] - pot[x_vec[i]])
                    };
                    println!("{}", result);
                } else {
                    println!("Ambiguous");
                }
            }
        }
    }
}
