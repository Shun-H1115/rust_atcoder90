// ==========================================================================
// 典型90 #075  Magic For Balls  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bw
// ==========================================================================
// 【アルゴリズム】 素因数分解（素因数の個数 k に対し、1回の魔法で個数を半分にできる → 答えは ceil(log2 k)）
// 【計算量】       O(√N)
// 【学習計画】     第3週（★3後半）
//
// 【このファイルで覚えるRust文法】
//   - (n as f64).sqrt() as i64 … 平方根の整数部。f64 の誤差が心配なら while i * i <= n ループで書く
//   - 関数が Vec<i64> を返す
//   - for ... { if ... { answer = i; break; } } … 条件を満たす最初の i を探す（Py の next(...)）
//
// 【Pythonで書くと（考え方の対応）】
//   def factors(n):
//       res = []; i = 2
//       while i * i <= n:
//           while n % i == 0: res.append(i); n //= i
//           i += 1
//       if n > 1: res.append(n)
//       return res
//   k = len(factors(N))
//   print((k - 1).bit_length())   # ceil(log2 k)
// ==========================================================================

use proconio::input;

fn prime_factors(n: i64) -> Vec<i64> {
    let mut rem = n;
    let mut p = Vec::new();
    
    // Finding prime factors up to sqrt(n)
    // √N まで試し割り。整数だけで書くなら let mut i = 2; while i * i <= n { ... i += 1; }
    for i in 2..=((n as f64).sqrt() as i64) {
        while rem % i == 0 {
            rem /= i;
            p.push(i);
        }
    }
    
    // If there's a remaining factor greater than sqrt(n)
    // 残りが 1 でなければ √N より大きい素因数が1つ残っている。
    if rem != 1 {
        p.push(rem);
    }
    
    p
}

fn main() {
    // Step #1. Input
    input! {
        n: i64,
    }
    
    // Step #2. Get Answer
    let vec = prime_factors(n);
    let mut answer = 0;
    
    // Determine the smallest power of 2 greater than or equal to vec.len()
    for i in 0..=20 {
        // 1 << i … answer と i が i32 になり、比較のため vec.len() を as i32 に揃えている。
        if (1 << i) >= vec.len() as i32 {
            answer = i;
            break;
        }
    }
    
    // Output the answer
    println!("{}", answer);
}
