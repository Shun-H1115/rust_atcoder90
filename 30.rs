use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    // Initialize the counter array
    let mut cnt = vec![0; n + 1];

    // Step #2. Count Number of Distinct Prime Factors
    for i in 2..=n {
        if cnt[i] >= 1 {
            continue;
        }
        for j in (i..=n).step_by(i) {
            cnt[j] += 1;
        }
    }

    // Step #3. Calculate the Answer
    let answer = cnt.iter().filter(|&&x| x >= k).count();
    println!("{}", answer);
}
