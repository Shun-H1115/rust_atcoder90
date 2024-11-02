use proconio::input;
use std::cmp::max;

const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];
const INF: i32 = -10000;

fn dfs(
    sx: usize,
    sy: usize,
    px: usize,
    py: usize,
    h: usize,
    w: usize,
    c: &Vec<Vec<char>>,
    used: &mut Vec<Vec<bool>>,
) -> i32 {
    if sx == px && sy == py && used[px][py] {
        return 0;
    }
    used[px][py] = true;

    let mut ret = INF;
    for i in 0..4 {
        let nx = px as i32 + DX[i];
        let ny = py as i32 + DY[i];
        if nx < 1 || ny < 1 || nx > h as i32 || ny > w as i32 || c[nx as usize][ny as usize] == '#' {
            continue;
        }
        if (sx != nx as usize || sy != ny as usize) && used[nx as usize][ny as usize] {
            continue;
        }
        let v = dfs(sx, sy, nx as usize, ny as usize, h, w, c, used);
        ret = max(ret, v + 1);
    }
    used[px][py] = false;
    ret
}

fn main() {
    // Step #1. Input
    input! {
        h: usize,
        w: usize,
        grid: [String; h],
    }

    // Convert grid to char matrix
    let mut c = vec![vec!['#'; w + 1]; h + 1];
    for i in 1..=h {
        for (j, ch) in grid[i - 1].chars().enumerate() {
            c[i][j + 1] = ch;
        }
    }

    // Step #2. DFS
    let mut answer = -1;
    let mut used = vec![vec![false; w + 1]; h + 1];
    for i in 1..=h {
        for j in 1..=w {
            answer = max(answer, dfs(i, j, i, j, h, w, &c, &mut used));
        }
    }
    if answer <= 2 {
        answer = -1;
    }
    println!("{}", answer);
}
