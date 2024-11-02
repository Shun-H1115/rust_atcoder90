use proconio::input;
use std::collections::HashMap;

fn maximum_same(r: Vec<i32>) -> usize {
    let mut map = HashMap::new();
    let mut ret = 0;
    for &value in &r {
        let count = map.entry(value).or_insert(0);
        *count += 1;
        ret = ret.max(*count);
    }
    ret
}

fn main() {
    // Step 1: Input
    input! {
        h: usize,
        w: usize,
        p: [[i32; w]; h],
    }

    let mut answer = 0;

    // Step 2: Bit masking over rows
    for i in 1..(1u32 << h) { // Specify i as u32 to use count_ones method
        let mut r = Vec::new();
        for j in 0..w {
            let mut idx = -1;
            let mut flag = false;
            for k in 0..h {
                if (i & (1 << k)) == 0 {
                    continue;
                }
                if idx == -1 {
                    idx = p[k][j];
                } else if idx != p[k][j] {
                    flag = true;
                    break;
                }
            }
            if !flag {
                r.push(idx);
            }
        }

        let cnt_h = i.count_ones() as usize;
        let cnt_w = maximum_same(r);
        answer = answer.max(cnt_h * cnt_w);
    }

    // Output result
    println!("{}", answer);
}
