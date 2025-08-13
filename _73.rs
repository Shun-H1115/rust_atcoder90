use proconio::{input, marker::Chars};

const MOD: i64 = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        cs: Chars,            // 長さ N の 'a'/'b' 列を 1 トークンで読み取る想定
    }

    let mut c = vec![' '; n + 1];
    for i in 1..=n {
        c[i] = cs[i - 1];
    }

    let mut g = vec![Vec::<usize>::new(); n + 1];
    for _ in 0..(n - 1) {
        input! { a: usize, b: usize }
        g[a].push(b);
        g[b].push(a);
    }

    // dp[v] = [dp0, dp1, dp2]
    // C[v]=='a' のとき dp0, 'b' のとき dp1 を val1 にして dp2 = val2 - val1
    let mut dp = vec![[0_i64; 3]; n + 1];

    // 反復DFS（後行順）: (v, parent, visited_flag)
    let mut stack: Vec<(usize, usize, bool)> = vec![(1, 0, false)];
    while let Some((v, p, visited)) = stack.pop() {
        if !visited {
            // 先に自分を「後処理」で戻す
            stack.push((v, p, true));
            // 子を積む
            for &to in &g[v] {
                if to == p {
                    continue;
                }
                stack.push((to, v, false));
            }
        } else {
            // 後処理：子の dp が確定している
            let mut val1: i64 = 1;
            let mut val2: i64 = 1;

            for &to in &g[v] {
                if to == p {
                    continue;
                }
                match c[v] {
                    'a' => {
                        // val1 *= (dp[to][0] + dp[to][2]);
                        let t1 = (dp[to][0] + dp[to][2]) % MOD;
                        val1 = (val1 * t1) % MOD;
                        // val2 *= (dp[to][0] + dp[to][1] + 2*dp[to][2]);
                        let t2 = (dp[to][0] + dp[to][1] + (2 * dp[to][2]) % MOD) % MOD;
                        val2 = (val2 * t2) % MOD;
                    }
                    'b' => {
                        // val1 *= (dp[to][1] + dp[to][2]);
                        let t1 = (dp[to][1] + dp[to][2]) % MOD;
                        val1 = (val1 * t1) % MOD;
                        // val2 *= (dp[to][0] + dp[to][1] + 2*dp[to][2]);
                        let t2 = (dp[to][0] + dp[to][1] + (2 * dp[to][2]) % MOD) % MOD;
                        val2 = (val2 * t2) % MOD;
                    }
                    _ => unreachable!("c[v] must be 'a' or 'b'"),
                }
            }

            match c[v] {
                'a' => {
                    dp[v][0] = val1 % MOD;
                    dp[v][2] = (val2 - val1).rem_euclid(MOD);
                }
                'b' => {
                    dp[v][1] = val1 % MOD;
                    dp[v][2] = (val2 - val1).rem_euclid(MOD);
                }
                _ => unreachable!(),
            }
        }
    }

    println!("{}", dp[1][2] % MOD);
}