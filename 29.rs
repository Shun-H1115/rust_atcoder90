// ==========================================================================
// 典型90 #029  Long Bricks  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_ac
// ==========================================================================
// 【アルゴリズム】 遅延評価セグメント木（区間 chmax 更新・区間 max 取得）。座標圧縮版でも解ける
// 【計算量】       O(N log W)
// 【学習計画】     第5週（★5前半）
//
// 【このファイルで覚えるRust文法】
//   - 再帰するメソッド &mut self … self.update(...) を中で呼べる
//   - struct 初期化の省略記法 SegmentTree { sz, seg: ..., lazy: ... }
//   - 公開用ラッパー関数でインターフェースを簡潔にする設計（Py でも同じ）
//
// 【Pythonで書くと（考え方の対応）】
//   # 各レンガ [l, r) について
//   #   h = seg.range_max(l-1, r) + 1
//   #   seg.update(l-1, r, h)   # 区間を h で上書き（ここでは max 更新）
//   #   print(h)
//
// 【注意・改善ポイント】
//   ! AtCoder では ac-library-rs（use ac_library::LazySegtree）が使える。自作セグ木の理解後は、
//     ライブラリの使い方も覚えると★5以上の問題が一気に楽になる。
// ==========================================================================

use proconio::input;
use std::cmp::max;

struct SegmentTree {
    sz: usize,
    seg: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        // sz を n 以上の2のべき乗にする。Py: sz = 1 << (n-1).bit_length()。Rust: n.next_power_of_two()
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        // Self の代わりに型名 SegmentTree と書いても同じ。
        SegmentTree {
            sz,
            seg: vec![0; sz * 2],
            lazy: vec![0; sz * 2],
        }
    }

    // 遅延値を子に伝播し、自分の値に反映する。
    fn push(&mut self, k: usize) {
        if k < self.sz {
            self.lazy[k * 2] = max(self.lazy[k * 2], self.lazy[k]);
            self.lazy[k * 2 + 1] = max(self.lazy[k * 2 + 1], self.lazy[k]);
        }
        self.seg[k] = max(self.seg[k], self.lazy[k]);
        self.lazy[k] = 0;
    }

    fn update(&mut self, a: usize, b: usize, x: i32, k: usize, l: usize, r: usize) {
        // 区間の外なら何もしない、完全に含まれれば遅延値を置く、部分的なら子に再帰（セグ木の基本3分岐）。
        self.push(k);
        if r <= a || b <= l {
            return;
        }
        if a <= l && r <= b {
            self.lazy[k] = x;
            self.push(k);
            return;
        }
        let mid = (l + r) / 2;
        self.update(a, b, x, k * 2, l, mid);
        self.update(a, b, x, k * 2 + 1, mid, r);
        self.seg[k] = max(self.seg[k * 2], self.seg[k * 2 + 1]);
    }

    // 取得でも push で書き換えが起きるので &mut self（読むだけに見えても可変借用が必要）。
    fn range_max(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> i32 {
        self.push(k);
        if r <= a || b <= l {
            return 0;
        }
        if a <= l && r <= b {
            return self.seg[k];
        }
        let mid = (l + r) / 2;
        let left = self.range_max(a, b, k * 2, l, mid);
        let right = self.range_max(a, b, k * 2 + 1, mid, r);
        max(left, right)
    }

    // Public methods
    fn public_update(&mut self, l: usize, r: usize, x: i32) {
        // ノード1 が全体 [0, sz) を担当する1-indexedのセグ木。
        self.update(l, r, x, 1, 0, self.sz);
    }

    fn public_range_max(&mut self, l: usize, r: usize) -> i32 {
        self.range_max(l, r, 1, 0, self.sz)
    }
}

fn main() {
    input! {
        w: usize,
        n: usize,
        intervals: [(usize, usize); n],
    }

    let mut seg = SegmentTree::new(w);
    
    // intervals を消費してループ（以降使わないので move で OK）。
    for (l, r) in intervals {
        // l-1 で 0-indexed に、r はそのまま半開区間の右端に。
        let height = seg.public_range_max(l - 1, r) + 1;
        seg.public_update(l - 1, r, height);
        println!("{}", height);
    }
}
