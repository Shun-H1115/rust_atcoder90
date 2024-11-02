use proconio::input;
use std::cmp::min;

fn main() {
    input! {
        n: usize,
        a: [[usize; n]; n],
        m: usize,
        incompatible_pairs: [(usize, usize); m],
    }

    // Initialize the incompatibility matrix
    let mut incompatible = vec![vec![false; n]; n];
    for (x, y) in incompatible_pairs {
        incompatible[x - 1][y - 1] = true;
        incompatible[y - 1][x - 1] = true;
    }

    // Initialize variables for brute force approach
    let mut runners: Vec<usize> = (0..n).collect();
    let mut answer = usize::MAX;
    let possible = |order: &Vec<usize>| -> bool {
        for i in 0..(n - 1) {
            if incompatible[order[i]][order[i + 1]] {
                return false;
            }
        }
        true
    };

    // Brute force by trying all permutations of runners
    loop {
        if possible(&runners) {
            let mut total_time = 0;
            for i in 0..n {
                total_time += a[runners[i]][i];
            }
            answer = min(answer, total_time);
        }

        // If no next permutation, break the loop
        if !next_permutation(&mut runners) {
            break;
        }
    }

    // Output the result
    if answer == usize::MAX {
        println!("-1");
    } else {
        println!("{}", answer);
    }
}

// Helper function to generate the next lexicographical permutation
fn next_permutation(arr: &mut Vec<usize>) -> bool {
    let n = arr.len();
    if n < 2 {
        return false;
    }
    let mut i = n - 2;
    while i != usize::MAX && arr[i] >= arr[i + 1] {
        i = i.wrapping_sub(1);
    }
    if i == usize::MAX {
        arr.reverse();
        return false;
    }
    let mut j = n - 1;
    while arr[j] <= arr[i] {
        j -= 1;
    }
    arr.swap(i, j);
    arr[i + 1..].reverse();
    true
}
