use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        a_in: [u128; n], // treat constraints as bitmasks
    }

    let mut answer: i128 = 0;

    let total = 1usize << n;
    for mask in 0..total {
        // OR all selected constraints
        let mut bit: u128 = 0;
        for j in 0..n {
            if (mask >> j) & 1 == 1 {
                bit |= a_in[j];
            }
        }

        // Count free digits among the lowest D bits
        let mut free_digits: usize = 0;
        for j in 0..d {
            if ((bit >> (j as u32)) & 1) == 0 {
                free_digits += 1;
            }
        }

        let ways: i128 = 1_i128 << (free_digits as u32);
        if mask.count_ones() % 2 == 0 {
            answer += ways;
        } else {
            answer -= ways;
        }
    }

    println!("{}", answer);
}