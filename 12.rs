// ==========================================================================
// 典型90 #012  Red Painting  (★4)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_l
// ==========================================================================
// 【アルゴリズム】 Union-Find（塗ったマスを隣接する赤マスと結合し、連結判定クエリに答える）
// 【計算量】       O(Q α(HW))
// 【学習計画】     第4週（★4）
//
// 【このファイルで覚えるRust文法】
//   - struct + impl でデータ構造を作る … Py の class。new は慣習的なコンストラクタ名（__init__）
//   - &mut self … 自分自身を書き換えるメソッド。Py の self は常に書き換え可能だが Rust は明示が必要
//   - Self … impl 対象の型（ここでは UnionFind）の別名
//   - isize … 符号付きの添字計算用。負になりうる座標計算は usize を isize に変換して行う
//   - ループ内で input! を何度も呼べる（クエリ形式の入力）
//
// 【Pythonで書くと（考え方の対応）】
//   class UnionFind:
//       def __init__(self, n): self.par = [-1]*n
//       def root(self, x):
//           if self.par[x] == -1: return x
//           self.par[x] = self.root(self.par[x]); return self.par[x]
//       def unite(self, u, v):
//           ru, rv = self.root(u), self.root(v)
//           if ru != rv: self.par[ru] = rv
//
// 【注意・改善ポイント】
//   ! この UF は経路圧縮のみ（union by size なし）。実用上ほぼ問題ないが、ライブラリ化するなら
//     par[root] = -(サイズ) で持つ union by size 版にしておくと他問題（#068 など）でも使い回せる。
//   ! root の再帰は最悪ケースで深くなりうる。Rust のスタックはデフォルト 8MB 程度で Py より深い再帰に耐えるが、
//     経路圧縮＋union by size にしておけば深さは気にしなくてよい。
// ==========================================================================

use proconio::input;

// Py の class UnionFind に相当。フィールドは型付きで宣言する。
struct UnionFind {
    // -1 を「自分が根」の印に使うので符号付きの i32。添字として使うときは as usize に変換。
    par: Vec<i32>,
}

impl UnionFind {
    // fn new(...) -> Self … 慣習的なコンストラクタ。呼び出しは UnionFind::new(n)。
    fn new(size: usize) -> Self {
        UnionFind {
            par: vec![-1; size],
        }
    }

    // &mut self … 経路圧縮で par を書き換えるので可変借用。
    // そのため same() も &mut self が必要になる（読むだけに見えても root が書き換えるため）。
    fn root(&mut self, pos: usize) -> usize {
        if self.par[pos] == -1 {
            return pos;
        }
        // 経路圧縮: 親を直接根に付け替える。i32 ↔ usize の変換が多いのは par を i32 で持っているため。
        self.par[pos] = self.root(self.par[pos] as usize) as i32;
        self.par[pos] as usize
    }

    fn unite(&mut self, u: usize, v: usize) {
        let root_u = self.root(u);
        let root_v = self.root(v);
        if root_u != root_v {
            self.par[root_u] = root_v as i32;
        }
    }

    fn same(&mut self, u: usize, v: usize) -> bool {
        self.root(u) == self.root(v)
    }
}

fn to_index(x: usize, y: usize, width: usize) -> usize {
    // (x, y) 1始まり → 0始まりの1次元添字に変換。Py: (x-1)*W + (y-1)
    (x - 1) * width + (y - 1)
}

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(h * w);
    // used[x][y] は 1 始まりでアクセスするので (h+1)×(w+1) 確保。
    let mut used = vec![vec![false; w + 1]; h + 1];
    // 配列リテラル [..; 4]。要素は (i32, i32) と推論されるが、下で isize と足すので (isize, isize) に推論される。
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    for _ in 0..q {
        // input! はループ内でも使える。1クエリずつ読むので「ty によって続く個数が違う」入力に対応できる。
        input! {
            ty: usize,
        }

        if ty == 1 {
            input! {
                x: usize,
                y: usize,
            }
            used[x][y] = true;
            let current_index = to_index(x, y, w);
            for &(dx, dy) in &directions {
                // x as isize + dx … usize のまま -1 するとアンダーフローするので、符号付きに変換してから計算する定石。
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                // 範囲チェック後に usize に戻す。Py なら 1 <= nx <= H で済むところ。
                if nx > 0 && ny > 0 && (nx as usize) <= h && (ny as usize) <= w {
                    // 同名 nx でシャドーイングして型を usize に変えている。
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if used[nx][ny] {
                        let neighbor_index = to_index(nx, ny, w);
                        uf.unite(current_index, neighbor_index);
                    }
                }
            }
        } else if ty == 2 {
            input! {
                xa: usize,
                ya: usize,
                xb: usize,
                yb: usize,
            }
            // && は短絡評価。used が false なら uf.same は呼ばれない（Py の and と同じ）。
            if used[xa][ya] && used[xb][yb] && uf.same(to_index(xa, ya, w), to_index(xb, yb, w)) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
