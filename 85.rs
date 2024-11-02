use proconio::input;

fn main() {
    // Step #1. Input
    input! {
        k: i64,
    }

    // Step #2. Enumerate Divisors
    let mut vec = Vec::new();
    for i in 1..=((k as f64).sqrt() as i64) {
        if k % i != 0 {
            continue;
        }
        vec.push(i);
        if i != k / i {
            vec.push(k / i);
        }
    }
    vec.sort();

    // Step #3. Brute Force
    let mut answer = 0;
    let size = vec.len();
    for i in 0..size {
        for j in i..size {
            let a = vec[i];
            let b = vec[j];
            if k / a < b {
                continue;
            }
            if k % (a * b) != 0 {
                continue;
            }
            let c = k / (a * b);
            if b <= c {
                answer += 1;
            }
        }
    }

    // Step #4. Output
    println!("{}", answer);
}
