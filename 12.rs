use proconio::input;

struct UnionFind {
    par: Vec<i32>,
}

impl UnionFind {
    fn new(size: usize) -> Self {
        UnionFind {
            par: vec![-1; size],
        }
    }

    fn root(&mut self, pos: usize) -> usize {
        if self.par[pos] == -1 {
            return pos;
        }
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
    (x - 1) * width + (y - 1)
}

fn main() {
    input! {
        h: usize,
        w: usize,
        q: usize,
    }

    let mut uf = UnionFind::new(h * w);
    let mut used = vec![vec![false; w + 1]; h + 1];
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];

    for _ in 0..q {
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
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx > 0 && ny > 0 && (nx as usize) <= h && (ny as usize) <= w {
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
            if used[xa][ya] && used[xb][yb] && uf.same(to_index(xa, ya, w), to_index(xb, yb, w)) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
