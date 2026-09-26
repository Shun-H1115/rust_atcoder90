// ==========================================================================
// 典型90 #043  Maze Challenge with Lack of Sleep  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_aq
// ==========================================================================
// 【アルゴリズム】 01-BFS（状態 = (マス, 向き)。直進はコスト0で先頭へ、方向転換はコスト1で末尾へ）
// 【計算量】       O(4HW)
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - #[derive(Copy, Clone)] struct State … 状態を struct で表現（タプル (x, y, dir) でも可）
//   - 3次元 Vec: vec![vec![vec![INF; 4]; w]; h]
//   - grid[tx].as_bytes()[ty] … String をバイト列として添字アクセス（ASCII 限定で高速）
//   - push_front / push_back … deque の両端追加（Py の appendleft / append）
//   - (0..4).map(...).min() … Py: min(dist[gx][gy][i] for i in range(4))
//
// 【Pythonで書くと（考え方の対応）】
//   from collections import deque
//   dq = deque((sx, sy, d) for d in range(4))
//   while dq:
//       x, y, d = dq.popleft()
//       for nd, (dx, dy) in enumerate(DIRS):
//           nx, ny = x + dx, y + dy
//           ... c = dist[x][y][d] + (nd != d)
//           if dist[nx][ny][nd] > c:
//               dist[nx][ny][nd] = c
//               (dq.append if nd != d else dq.appendleft)((nx, ny, nd))
// ==========================================================================

use proconio::input;
use std::collections::VecDeque;

const INF: i32 = 1_012_345_678;
// グローバル定数配列。移動方向の差分。
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
        // mut で受けて直後に -1（0-indexed 化）。
        mut sx: usize,
        mut sy: usize,
        mut gx: usize,
        mut gy: usize,
        // [String; h] で受けて as_bytes() でアクセス。[Chars; h] にすると grid[x][y] == '.' と書ける。
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
            // usize のまま -1 するとアンダーフローするので i32 に変換して計算。
            let tx = u.x as i32 + DX[i];
            let ty = u.y as i32 + DY[i];
            if tx < 0 || tx >= h as i32 || ty < 0 || ty >= w as i32 {
                continue;
            }
            // 範囲チェック後にシャドーイングで usize に戻す。
            let (tx, ty) = (tx as usize, ty as usize);
            // as_bytes() は &[u8]。b'.' は u8 の '.'。
            if grid[tx].as_bytes()[ty] != b'.' {
                continue;
            }

            // if 式を足し算の中に書ける。Py: cost = dist[...] + (1 if u.dir != i else 0)
            let cost = dist[u.x][u.y][u.dir] + if u.dir != i { 1 } else { 0 };
            if dist[tx][ty][i] > cost {
                dist[tx][ty][i] = cost;
                if u.dir != i {
                    deq.push_back(State { x: tx, y: ty, dir: i });
                } else {
                    // コスト0の遷移は先頭に積む → 常に距離の小さい順に取り出される（01-BFS の核心）。
                    deq.push_front(State { x: tx, y: ty, dir: i });
                }
            }
        }
    }

    // min() は Option を返す。unwrap_or(INF) で空の場合の既定値。
    let answer = (0..4).map(|i| dist[gx][gy][i]).min().unwrap_or(INF);
    println!("{}", answer);
}
