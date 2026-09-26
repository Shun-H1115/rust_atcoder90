// ==========================================================================
// 典型90 #088  Similar but Different Ways  (★6)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_cj
// ==========================================================================
// 【アルゴリズム】 鳩の巣原理＋DFS（和の取りうる値は 8889 通り以下なので、条件を満たす集合を 8890 個列挙すれば必ず和が一致する2つが出る）
// 【計算量】       O(8889 × N)
// 【学習計画】     第7週（★6）
//
// 【このファイルで覚えるRust文法】
//   - 状態を struct State にまとめ、impl State { fn dfs(&mut self, ...) } … #071 の「引数が多い再帰」との比較に最適
//   - for &to in &self.g[pos] { self.c[to] += 1; } … self の別フィールドなら不変借用中でも書き換え可能
//   - Vec<Vec<Vec<usize>>> … 3次元の可変長リスト
//   - State { n, a, g, ... } … 変数を move して struct に格納（フィールド名省略記法）
//
// 【Pythonで書くと（考え方の対応）】
//   def dfs(pos, s):
//       global found
//       if found: return
//       if pos == n + 1:
//           ans[s].append(chosen[:])
//           if len(ans[s]) == 2: found = True
//           return
//       dfs(pos + 1, s)
//       if c[pos] == 0:
//           chosen.append(pos)
//           for t in g[pos]: c[t] += 1
//           dfs(pos + 1, s + a[pos])
//           for t in g[pos]: c[t] -= 1
//           chosen.pop()
//
// 【注意・改善ポイント】
//   ! 出力部分が2回重複している。for idx in 0..2 { ... } でまとめると短くなる。
// ==========================================================================

use proconio::input;

// 再帰で共有する状態を1つの struct にまとめる設計。&mut self を渡すだけで済む。
struct State {
    n: usize,
    a: Vec<usize>,                 // 1-indexed
    g: Vec<Vec<usize>>,            // edges: x -> y
    // c[v]: 選んだ頂点のうち v を「選べなくする」ものの個数。0 なら選べる。
    c: Vec<i32>,                   // in-constraint counter (how many chosen predecessors point to v)
    vec: Vec<usize>,               // current chosen set
    answer: Vec<Vec<Vec<usize>>>,  // answer[sum] = list of chosen index lists (store up to 2)
    flag: bool,                    // stop once we found two
}

impl State {
    // &mut self … 全フィールドを書き換え可能な状態で再帰。
    fn dfs(&mut self, pos: usize, dep: usize) {
        if self.flag {
            return;
        }
        if pos == self.n + 1 {
            // record this set for sum=dep
            if dep <= 10000 {
                // self.vec.clone() … 現在の選択をコピーして保存。
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
            // self.g を不変借用しながら self.c を書き換えている。別フィールドなので借用チェッカーは許可する
            // （self 全体をメソッド呼び出しで借用していると不可。ここは直接フィールドアクセスなのでOK）。
            for &to in &self.g[pos] {
                self.c[to] += 1;
            }
            // self.dfs(...) を呼ぶ時点では self.g の借用は終わっている。
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

    // n, a, g を move して struct に。以降 main の a, g は使えない（st.a, st.g でアクセス）。
    let mut st = State {
        n,
        a,
        g,
        c: vec![0; n + 1],
        vec: Vec::new(),
        // 和ごとのリスト（最大 10000）。
        answer: vec![Vec::<Vec<usize>>::new(); 10001],
        flag: false,
    };

    st.dfs(1, 0);

    // 和が同じ集合が2つ見つかった s を出力。
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
