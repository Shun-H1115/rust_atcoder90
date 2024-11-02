use proconio::input;
use std::collections::VecDeque;

const INF: i32 = 1_012_345_678;
const DX: [i32; 4] = [1, 0, -1, 0];
const DY: [i32; 4] = [0, 1, 0, -1];

#[derive(Copy, Clone)]
struct State {
    x: usize,
    y: usize,
    dir: usize,
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut sx: usize,
        mut sy: usize,
        mut gx: usize,
        mut gy: usize,
        grid: [String; h],
    }

    sx -= 1;
    sy -= 1;
    gx -= 1;
    gy -= 1;

    let mut dist = vec![vec![vec![INF; 4]; w]; h];
    let mut deq = VecDeque::new();

    for i in 0..4 {
        dist[sx][sy][i] = 0;
        deq.push_back(State { x: sx, y: sy, dir: i });
    }

    while let Some(u) = deq.pop_front() {
        for i in 0..4 {
            let tx = u.x as i32 + DX[i];
            let ty = u.y as i32 + DY[i];
            if tx < 0 || tx >= h as i32 || ty < 0 || ty >= w as i32 {
                continue;
            }
            let (tx, ty) = (tx as usize, ty as usize);
            if grid[tx].as_bytes()[ty] != b'.' {
                continue;
            }

            let cost = dist[u.x][u.y][u.dir] + if u.dir != i { 1 } else { 0 };
            if dist[tx][ty][i] > cost {
                dist[tx][ty][i] = cost;
                if u.dir != i {
                    deq.push_back(State { x: tx, y: ty, dir: i });
                } else {
                    deq.push_front(State { x: tx, y: ty, dir: i });
                }
            }
        }
    }

    let answer = (0..4).map(|i| dist[gx][gy][i]).min().unwrap_or(INF);
    println!("{}", answer);
}
