use proconio::input;

fn main() {
    input! {
        n: usize,          // Number of intervals
        ranges: [(i32, i32); n],  // Vector of (L, R) pairs representing ranges
    }

    let mut expsum = 0.0;

    // Loop over each pair of intervals
    for i in 0..n {
        for j in (i + 1)..n {
            let (li, ri) = ranges[i];  // Range [L[i], R[i]]
            let (lj, rj) = ranges[j];  // Range [L[j], R[j]]
            let len_i = (ri - li + 1) as f64;
            let len_j = (rj - lj + 1) as f64;

            let mut count = 0.0;

            // Calculate the probability of values in [L[i], R[i]] being greater than in [L[j], R[j]]
            if ri < lj {
                // All values in range i are less than all in range j
                count = 0.0;
            } else if rj < li {
                // All values in range j are less than all in range i
                count = 1.0;
            } else {
                // Handle overlapping ranges
                let overlap_min = li.max(lj);
                let overlap_max = ri.min(rj);

                // Sum probabilities where a number from range i is greater than a number from range j
                for k in li..=ri {
                    if k > rj {
                        // If k in range i is greater than all values in range j
                        count += len_j;
                    } else if k >= lj {
                        // If k in range i is within the overlap with range j
                        count += (k - lj) as f64;
                    }
                }
                count /= len_i * len_j;
            }

            // Add the probability for this pair to the total expected sum
            expsum += count;
        }
    }

    // Print the result with 15 decimal precision
    println!("{:.15}", expsum);
}
