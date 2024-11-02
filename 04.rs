use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i32; w]; h],
    }

    let mut row_sum = vec![0; h];
    let mut col_sum = vec![0; w];

    // Calculate row and column sums
    for i in 0..h {
        for j in 0..w {
            row_sum[i] += a[i][j];
            col_sum[j] += a[i][j];
        }
    }

    // Calculate the result matrix
    let mut answer = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            answer[i][j] = row_sum[i] + col_sum[j] - a[i][j];
        }
    }

    // Output the result matrix
    for i in 0..h {
        for j in 0..w {
            if j > 0 {
                print!(" ");
            }
            print!("{}", answer[i][j]);
        }
        println!();
    }
}
