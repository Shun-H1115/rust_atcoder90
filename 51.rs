use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: i64,
        a: [i64; n],
    }

    let mid = n / 2;
    let tail = n - mid;

    // vec1[c]: all sums of choosing c items from the first half
    // vec2[c]: all sums of choosing c items from the second half
    let mut vec1: Vec<Vec<i64>> = vec![Vec::new(); n + 1];
    let mut vec2: Vec<Vec<i64>> = vec![Vec::new(); n + 1];

    // Enumerate subsets of the first half
    for mask in 0..(1usize << mid) {
        let mut sum = 0i64;
        let mut cnt = 0usize;
        for j in 0..mid {
            if (mask >> j) & 1 == 1 {
                sum += a[j];
                cnt += 1;
            }
        }
        vec1[cnt].push(sum);
    }

    // Enumerate subsets of the second half
    for mask in 0..(1usize << tail) {
        let mut sum = 0i64;
        let mut cnt = 0usize;
        for j in 0..tail {
            if (mask >> j) & 1 == 1 {
                sum += a[mid + j];
                cnt += 1;
            }
        }
        vec2[cnt].push(sum);
    }

    // Sort for binary searches
    for i in 0..=n {
        vec1[i].sort_unstable();
        vec2[i].sort_unstable();
    }

    // Count pairs (h from 0..=K): pick h from first half and K-h from second half
    // For each sum s in vec1[h], count how many t in vec2[K-h] with s + t <= P
    let mut answer: i64 = 0;
    for h in 0..=k {
        if h > n || k - h > n {
            continue; // safety (normally K <= N)
        }
        let v1 = &vec1[h];
        let v2 = &vec2[k - h];
        for &s in v1.iter() {
            let target = p - s;
            // number of elements <= target
            let cnt = v2.partition_point(|x| *x <= target);
            answer += cnt as i64;
        }
    }

    println!("{}", answer);
}