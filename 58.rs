use proconio::input;

fn digit_sum(x: i32) -> i32 {
    let mut ans = 0;
    let mut num = x;
    // Calculate the sum of the digits of `num`
    while num > 0 {
        ans += num % 10;
        num /= 10;
    }
    ans
}

fn main() {
    const MOD: usize = 100000;
    input! {
        n: usize,
        mut k: i64,
    }

    // Create the next step array based on the digit sums
    let mut nxt = vec![0; MOD];
    for i in 0..MOD {
        nxt[i] = (i + digit_sum(i as i32) as usize) % MOD;
    }

    // Record the first occurrence time for each position
    let mut time_stamp = vec![-1; MOD];
    let mut pos = n;
    let mut cnt = 0;

    // Detect the cycle by marking each visited position with its timestamp
    while time_stamp[pos] == -1 {
        time_stamp[pos] = cnt;
        pos = nxt[pos];
        cnt += 1;
    }

    // Calculate cycle length and adjust `K` to be within the cycle range
    let cycle_length = cnt - time_stamp[pos];
    if k >= time_stamp[pos] as i64 {
        k = (k - time_stamp[pos] as i64) % cycle_length as i64 + time_stamp[pos] as i64;
    }

    // Find the number corresponding to the adjusted step `K`
    let mut answer = -1;
    for i in 0..MOD {
        if time_stamp[i] == k as i32 {
            answer = i as i32;
            break;
        }
    }

    println!("{}", answer);
}
