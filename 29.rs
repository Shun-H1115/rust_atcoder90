use proconio::input;
use std::cmp::max;

struct SegmentTree {
    sz: usize,
    seg: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegmentTree {
    fn new(n: usize) -> Self {
        let mut sz = 1;
        while sz < n {
            sz *= 2;
        }
        SegmentTree {
            sz,
            seg: vec![0; sz * 2],
            lazy: vec![0; sz * 2],
        }
    }

    fn push(&mut self, k: usize) {
        if k < self.sz {
            self.lazy[k * 2] = max(self.lazy[k * 2], self.lazy[k]);
            self.lazy[k * 2 + 1] = max(self.lazy[k * 2 + 1], self.lazy[k]);
        }
        self.seg[k] = max(self.seg[k], self.lazy[k]);
        self.lazy[k] = 0;
    }

    fn update(&mut self, a: usize, b: usize, x: i32, k: usize, l: usize, r: usize) {
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
    
    for (l, r) in intervals {
        let height = seg.public_range_max(l - 1, r) + 1;
        seg.public_update(l - 1, r, height);
        println!("{}", height);
    }
}
