use proconio::input;

struct State {
    n: usize,
    a: Vec<usize>,                 // 1-indexed
    g: Vec<Vec<usize>>,            // edges: x -> y
    c: Vec<i32>,                   // in-constraint counter (how many chosen predecessors point to v)
    vec: Vec<usize>,               // current chosen set
    answer: Vec<Vec<Vec<usize>>>,  // answer[sum] = list of chosen index lists (store up to 2)
    flag: bool,                    // stop once we found two
}

impl State {
    fn dfs(&mut self, pos: usize, dep: usize) {
        if self.flag {
            return;
        }
        if pos == self.n + 1 {
            // record this set for sum=dep
            if dep <= 10000 {
                self.answer[dep].push(self.vec.clone());
                if self.answer[dep].len() == 2 {
                    self.flag = true;
                }
            }
            return;
        }

        // Don't choose pos
        self.dfs(pos + 1, dep);

        // Choose pos, only if not constrained
        if self.c[pos] == 0 {
            self.vec.push(pos);
            for &to in &self.g[pos] {
                self.c[to] += 1;
            }
            self.dfs(pos + 1, dep + self.a[pos]);
            for &to in &self.g[pos] {
                self.c[to] -= 1;
            }
            self.vec.pop();
        }
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
        a_in: [usize; n],
        xy: [(usize, usize); q],
    }

    // 1-indexed arrays
    let mut a = vec![0usize; n + 1];
    for i in 1..=n {
        a[i] = a_in[i - 1];
    }
    let mut g = vec![Vec::<usize>::new(); n + 1];
    for (x, y) in xy {
        g[x].push(y);
    }

    let mut st = State {
        n,
        a,
        g,
        c: vec![0; n + 1],
        vec: Vec::new(),
        answer: vec![Vec::<Vec<usize>>::new(); 10001],
        flag: false,
    };

    st.dfs(1, 0);

    for s in 0..=10000 {
        if st.answer[s].len() > 1 {
            // print first set
            println!("{}", st.answer[s][0].len());
            for (j, v) in st.answer[s][0].iter().enumerate() {
                if j > 0 { print!(" "); }
                print!("{}", v);
            }
            println!();
            // print second set
            println!("{}", st.answer[s][1].len());
            for (j, v) in st.answer[s][1].iter().enumerate() {
                if j > 0 { print!(" "); }
                print!("{}", v);
            }
            println!();
            return;
        }
    }
}