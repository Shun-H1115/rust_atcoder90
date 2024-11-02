use proconio::input;

fn main() {
    // Input
    input! {
        n: usize,
        rectangles: [(usize, usize, usize, usize); n],
    }

    // Initialize the grid and answer arrays
    let mut cnt = vec![vec![0; 1001]; 1001];
    let mut answer = vec![0; n + 1];

    // Step #2. Imos Method in 2D
    for &(lx, ly, rx, ry) in &rectangles {
        cnt[lx][ly] += 1;
        cnt[lx][ry] -= 1;
        cnt[rx][ly] -= 1;
        cnt[rx][ry] += 1;
    }

    // Horizontal cumulative sum
    for i in 0..=1000 {
        for j in 1..=1000 {
            cnt[i][j] += cnt[i][j - 1];
        }
    }

    // Vertical cumulative sum
    for i in 1..=1000 {
        for j in 0..=1000 {
            cnt[i][j] += cnt[i - 1][j];
        }
    }

    // Step #3. Count the number of cells with each overlap count
    for i in 0..=1000 {
        for j in 0..=1000 {
            if cnt[i][j] >= 1 {
                answer[cnt[i][j] as usize] += 1;
            }
        }
    }

    // Step #4. Output the Answer
    for i in 1..=n {
        println!("{}", answer[i]);
    }
}
