// ==========================================================================
// 典型90 #072  Loop Railway Plan  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bt
// ==========================================================================
// 【アルゴリズム】 DFS 全探索（バックトラック）で各始点からの最長の閉路を探す（H×W ≤ 16 なので間に合う）
// 【計算量】       O(HW × 3^(HW))（枝刈りで実際はずっと小さい）
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - 引数の多い再帰関数（&Vec<Vec<char>> と &mut Vec<Vec<bool>> の使い分け）
//   - 周囲を '#' で囲んだグリッド（番兵）… 1-indexed にして範囲外チェックを単純化
//   - grid[i-1].chars().enumerate() … 文字列を1文字ずつ添字付きで
//
// 【Pythonで書くと（考え方の対応）】
//   def dfs(sx, sy, px, py):
//       if (px, py) == (sx, sy) and used[px][py]: return 0
//       used[px][py] = True; ret = -10**9
//       for dx, dy in DIRS:
//           nx, ny = px + dx, py + dy
//           if not inside or c[nx][ny] == '#': continue
//           if (nx, ny) != (sx, sy) and used[nx][ny]: continue
//           ret = max(ret, dfs(sx, sy, nx, ny) + 1)
//       used[px][py] = False
//       return ret
// ==========================================================================

use proconio::input;
use std::cmp::max;

const DX: [i32; 4] = [0, 1, 0, -1];
const DY: [i32; 4] = [1, 0, -1, 0];
// INF という名前だが実際は「負の大きな値」（到達不能の印）。NEG_INF の方が意図が明確。
const INF: i32 = -10000;

fn dfs(
    sx: usize,
    sy: usize,
    px: usize,
    py: usize,
    h: usize,
    w: usize,
    // c は読むだけ（&）、used は書き換える（&mut）。
    c: &Vec<Vec<char>>,
    used: &mut Vec<Vec<bool>>,
) -> i32 {
    // 始点に戻ってきたら閉路完成。
    if sx == px && sy == py && used[px][py] {
        return 0;
    }
    used[px][py] = true;

    let mut ret = INF;
    for i in 0..4 {
        // px as i32 + DX[i] … 負になりうる計算は符号付きで。
        let nx = px as i32 + DX[i];
        let ny = py as i32 + DY[i];
        if nx < 1 || ny < 1 || nx > h as i32 || ny > w as i32 || c[nx as usize][ny as usize] == '#' {
            continue;
        }
        // 始点以外の訪問済みマスには入らない（始点だけは戻ってきてよい）。
        if (sx != nx as usize || sy != ny as usize) && used[nx as usize][ny as usize] {
            continue;
        }
        // c と used は既に参照なのでそのまま渡す。
        let v = dfs(sx, sy, nx as usize, ny as usize, h, w, c, used);
        ret = max(ret, v + 1);
    }
    // バックトラック: 戻るときに訪問マークを外す。
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
    // 周囲を '#' で埋めた (h+1)×(w+1) の配列。0 行目と 0 列目が番兵。
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
    // 長さ 2 以下の閉路は往復（同じ辺を2回）なので無効。
    if answer <= 2 {
        answer = -1;
    }
    println!("{}", answer);
}
