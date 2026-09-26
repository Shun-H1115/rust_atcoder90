// ==========================================================================
// 典型90 #009  Three Point Angle  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_i
// ==========================================================================
// 【アルゴリズム】 偏角ソート＋二分探索（中心点を固定し、各方向の真反対に最も近い点を探す）
// 【計算量】       O(N^2 log N)
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - struct と #[derive(Clone, Copy, Debug)] … Py の @dataclass に近い。Copy を付けると代入でコピーされる
//   - impl std::ops::Sub for Point … 演算子オーバーロード（Py の __sub__）
//   - f64 のソート: sort_by(|a, b| a.partial_cmp(b).unwrap()) … f64 は NaN があるので sort() が使えない！
//   - Ok(p) | Err(p) => … パターンの「または」
//   - f64 のメソッド: x.sqrt(), x.acos(), a.max(b)（Py の math.sqrt 等は関数、Rust はメソッド）
//   - println!("{:.12}", x) … Py: print(f"{x:.12f}")
//
// 【Pythonで書くと（考え方の対応）】
//   import math
//   # 角度は math.degrees(math.atan2(dy, dx)) % 360 の方が簡潔
//   angles = sorted(math.degrees(math.atan2(q[1]-p[1], q[0]-p[0])) % 360 for q in pts if q is not p)
//   for a in angles:
//       t = (a + 180) % 360
//       k = bisect_left(angles, t)
//       ...
// ==========================================================================

use proconio::input;
use std::f64::consts::PI;
use std::cmp::Ordering;

// derive で Clone/Copy/Debug を自動実装。Copy がないと p - points[pos] で所有権が移動してしまう。
#[derive(Clone, Copy, Debug)]
struct Point {
    px: f64,
    py: f64,
}

// 演算子 - を定義。Py: def __sub__(self, other): return Point(...)
impl std::ops::Sub for Point {
    // type Output … 演算結果の型（関連型）。
    type Output = Point;
    fn sub(self, other: Point) -> Point {
        Point { px: self.px - other.px, py: self.py - other.py }
    }
}

fn get_angle(g: Point) -> f64 {
    // Calculate the polar angle of point G
    // acos で角度を求め、y<0 なら 360 から引く。
    // 代替: g.py.atan2(g.px).to_degrees() → -180..180 なので負なら +360。こちらが一般的。
    let angle = (g.px / (g.px * g.px + g.py * g.py).sqrt()).acos() * 180.0 / PI;
    if g.py >= 0.0 {
        angle
    } else {
        360.0 - angle
    }
}

fn get_angle_diff(i1: f64, i2: f64) -> f64 {
    // Calculate the smallest angle between two angles i1 and i2
    let res = (i1 - i2).abs();
    if res >= 180.0 {
        360.0 - res
    } else {
        res
    }
}

fn solve(pos: usize, points: &[Point]) -> f64 {
    // Calculate the maximum angle for a given point pos
    let mut angles = Vec::new();
    // iter().enumerate() … Py の enumerate(points)。&p で要素をコピーして受け取る（Point が Copy だから可能）。
    for (i, &p) in points.iter().enumerate() {
        if i == pos { continue; }
        let sa = p - points[pos];
        angles.push(get_angle(sa));
    }
    // f64 は全順序(Ord)でないので sort() 不可。partial_cmp で比較し Option を unwrap する。
    // Rust 1.62+ なら angles.sort_by(|a, b| a.total_cmp(b)); が簡潔。
    angles.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));

    let mut max_angle: f64 = 0.0;
    for &angle in &angles {
        let target = if angle + 180.0 >= 360.0 {
            angle + 180.0 - 360.0
        } else {
            angle + 180.0
        };
        // match で Ok と Err の両方の値 p を取り出す。% len で末尾を超えたら先頭に巻き戻す（円環）。
        let pos1 = match angles.binary_search_by(|&x| x.partial_cmp(&target).unwrap_or(Ordering::Equal)) {
            Ok(p) | Err(p) => p % angles.len(),
        };
        let cand1 = get_angle_diff(angle, angles[pos1]);
        // (pos1 + len - 1) % len … 1つ前（円環）。pos1 - 1 だと 0 のとき usize アンダーフローするので +len してから引いている。
        let cand2 = get_angle_diff(angle, angles[(pos1 + angles.len() - 1) % angles.len()]);
        // f64 の最大値は max メソッド（std::cmp::max は f64 に使えない）。
        max_angle = max_angle.max(cand1.max(cand2));
    }
    max_angle
}

fn solve_fast(points: &[Point]) -> f64 {
    let mut max_angle: f64 = 0.0;
    for i in 0..points.len() {
        max_angle = max_angle.max(solve(i, points));
    }
    max_angle
}

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        points: [(f64, f64); n],
    }
    // タプルの Vec を struct の Vec に変換。map(|&(x, y)| ...) は引数を分解するクロージャ。Py: [Point(x, y) for x, y in points]
    let points: Vec<Point> = points.iter().map(|&(x, y)| Point { px: x, py: y }).collect();

    // Step #2. Output
    let final_answer = solve_fast(&points);
    // {:.12} … 小数点以下12桁。
    println!("{:.12}", final_answer);
}
