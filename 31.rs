use proconio::input;

const MAX_A: usize = 50;
const MAX_B: usize = 1500;

fn init_grundy() -> Vec<Vec<usize>> {
    let mut grundy = vec![vec![0; MAX_B + 1]; MAX_A + 1];
    
    for i in 0..=MAX_A {
        for j in 0..=MAX_B {
            let mut mex = vec![false; MAX_B + 1];
            
            // Case 1: Subtracting `i` from `j`
            if i >= 1 && j + i <= MAX_B {
                mex[grundy[i - 1][j + i]] = true;
            }
            
            // Case 2: Halving `j`
            if j >= 2 {
                for k in 1..=(j / 2) {
                    mex[grundy[i][j - k]] = true;
                }
            }
            
            // Find the minimum non-occurring value
            grundy[i][j] = mex.iter().position(|&x| !x).unwrap();
        }
    }
    
    grundy
}

fn main() {
    // Input
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    // Initialize Grundy table
    let grundy = init_grundy();

    // Calculate the XOR sum of Grundy numbers for each pair (A[i], B[i])
    let mut sum_xor = 0;
    for i in 0..n {
        sum_xor ^= grundy[a[i]][b[i]];
    }

    // Output the result
    if sum_xor != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
