// ==========================================================================
// 典型90 #041  Piles in AtCoder Farm  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ao
// ==========================================================================
// 【アルゴリズム】 凸包（Andrew のモノトーンチェイン）＋ ピックの定理（面積と辺上の格子点数から内部格子点数を求める）
// 【計算量】       O(N log N)
// 【学習計画】     第8週（★7 実装推奨5問の1つ。凸包は幾何の基本ライブラリ）
//
// 【このファイルで覚えるRust文法】
//   - impl std::ops::Add / Sub for Point … 演算子オーバーロード（Py の __add__ / __sub__）
//   - type Output = Self; … 演算結果の型
//   - sort_by(|a, b| ...) で複合キーのソート。(a.px, a.py).cmp(&(b.px, b.py)) と書くとより簡潔
//   - extend(iter) … Py の list.extend()
//   - iter().rev().skip(1).take(k) … Py: list(reversed(g2))[1:1+k]
//   - Point { px, py } … 変数名とフィールド名が同じなら省略可能
//
// 【Pythonで書くと（考え方の対応）】
//   pts.sort()
//   def half(pts, sign):
//       h = []
//       for p in pts:
//           while len(h) >= 2 and sign * cross(h[-1]-h[-2], p-h[-1]) <= 0: h.pop()
//           h.append(p)
//       return h
//   # 面積 S (2倍), 境界点 b → 内部点 i = (2S - b + 2)/2、答え = i + b - N
//
// 【注意・改善ポイント】
//   ! use std::cmp::{min, max} は未使用（warning）。
//   ! sort_by の比較は g.sort_by_key(|p| (p.px, p.py)); の1行で書ける（タプルは辞書式比較）。
// ==========================================================================

use proconio::input;
use std::cmp::{min, max};

// Struct to represent a point with x and y coordinates
// Copy を付けたので、Point を関数に渡しても move されずコピーされる（i64 2個なので軽い）。
#[derive(Copy, Clone)]
struct Point {
    px: i64,
    py: i64,
}

// Operator overloading for addition and subtraction of Points
// + 演算子を定義。
impl std::ops::Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point { px: self.px + other.px, py: self.py + other.py }
    }
}

impl std::ops::Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Point { px: self.px - other.px, py: self.py - other.py }
    }
}

// Cross product function to determine orientation
fn crs(p1: Point, p2: Point) -> i64 {
    // 外積。正なら左回り（反時計回り）。
    p1.px * p2.py - p1.py * p2.px
}

// Function to calculate the greatest common divisor
fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        // 再帰版 gcd。末尾の if 式がそのまま戻り値。
        gcd(b, a % b)
    }
}

fn main() {
    // Step 1: Input
    input! {
        n: usize,
        points: [(i64, i64); n],
    }
    // |&(px, py)| Point { px, py } … タプルを分解して struct に。フィールド初期化の省略記法。
    let mut g: Vec<Point> = points.iter().map(|&(px, py)| Point { px, py }).collect();
    // x 優先、同じなら y で比較。
    g.sort_by(|a, b| if a.px == b.px { a.py.cmp(&b.py) } else { a.px.cmp(&b.px) });

    // Step 2: Calculate the convex hull using monotone chain method
    let mut g1 = vec![g[0], g[1]];
    let mut g2 = vec![g[0], g[1]];
    for i in 2..n {
        // 下側凸包: 右に曲がる（外積 <= 0）間は pop。g1[g1.len()-1] は Py の g1[-1]（Rust に負の添字は無い）。
        // g1.last().unwrap() とも書ける。
        while g1.len() >= 2 && crs(g1[g1.len() - 1] - g1[g1.len() - 2], g[i] - g1[g1.len() - 1]) <= 0 {
            g1.pop();
        }
        // 上側凸包: 左に曲がる間は pop（符号が逆）。
        while g2.len() >= 2 && crs(g2[g2.len() - 1] - g2[g2.len() - 2], g[i] - g2[g2.len() - 1]) >= 0 {
            g2.pop();
        }
        g1.push(g[i]);
        g2.push(g[i]);
    }

    // Combine g1 and g2 to form the full convex hull in Totsuhou
    let mut totsuhou = g1.clone();
    // 上側を逆順にして両端（重複する端点）を除いて連結。
    totsuhou.extend(g2.iter().rev().skip(1).take(g2.len() - 2));

    // Step 3: Calculate the number of integer lattice points on the polygon edges
    let mut edge_point = totsuhou.len() as i64;
    for i in 0..totsuhou.len() {
        let ax = totsuhou[i % totsuhou.len()].px;
        let ay = totsuhou[i % totsuhou.len()].py;
        let bx = totsuhou[(i + 1) % totsuhou.len()].px;
        let by = totsuhou[(i + 1) % totsuhou.len()].py;
        let vx = (bx - ax).abs();
        let vy = (by - ay).abs();
        // 線分上の格子点数 = gcd(|dx|, |dy|) + 1。端点の重複を除くため -1 して頂点数に加算。
        edge_point += gcd(vx, vy) - 1;
    }

    // Step 4: Calculate the area using the shoelace formula (2 * area)
    let mut area = 0;
    for i in 0..totsuhou.len() {
        let ax = totsuhou[i % totsuhou.len()].px;
        let ay = totsuhou[i % totsuhou.len()].py;
        let bx = totsuhou[(i + 1) % totsuhou.len()].px;
        let by = totsuhou[(i + 1) % totsuhou.len()].py;
        // 台形公式（靴ひも公式の変形）で面積の2倍を計算。
        area += (bx - ax) * (by + ay);
    }
    area = area.abs();

    // Step 5: Calculate the result using Pick's Theorem
    // ピックの定理: S = i + b/2 - 1 → i + b = (2S + b + 2) / 2。既にある N 本を引く。
    let answer = (area + edge_point + 2) / 2 - n as i64;
    println!("{}", answer);
}
