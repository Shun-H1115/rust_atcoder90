// ==========================================================================
// 典型90 #047  Monochromatic Diagonal  (★7)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_au
// ==========================================================================
// 【アルゴリズム】 ローリングハッシュ（色を 0,1,2 にし、「和が 0 mod 3」になる条件を文字列一致判定に帰着）
// 【計算量】       O(N)
// 【学習計画】     第8週以降（★7。ハッシュの考え方だけ通勤で押さえればOK）
//
// 【このファイルで覚えるRust文法】
//   - match 式で文字 → 数値変換（Py の dict {'R': 0, 'G': 1, 'B': 2}）
//   - s.chars().map(|c| match c { ... }).collect() … Py: [conv[c] for c in s]
//   - ローリングハッシュを i64 と大きな素数 MOD で実装
//
// 【Pythonで書くと（考え方の対応）】
//   conv = {'R': 0, 'G': 1, 'B': 2}
//   a = [conv[c] for c in S]; t = [conv[c] for c in T]
//   for i in range(3):
//       b = [(i - x) % 3 for x in t]
//       # S の接頭辞のハッシュと b の接尾辞のハッシュを1文字ずつ伸ばしながら比較 …
//
// 【注意・改善ポイント】
//   ! seq2 に mut は不要（warning）。
//   ! 単一 MOD のハッシュは衝突の可能性がある。心配なら MOD = 2^61-1 と u128 の掛け算を使う方法が定番。
// ==========================================================================

use proconio::input;

const MOD: i64 = 699_999_953; // large prime number

fn main() {
    input! {
        n: usize,
        s: String,
        t: String,
    }
    
    // match の各腕が値を返す。_ はそれ以外（'B'）。
    let seq1: Vec<i64> = s.chars().map(|c| match c {
        'R' => 0,
        'G' => 1,
        _ => 2,
    }).collect();

    let seq3: Vec<i64> = t.chars().map(|c| match c {
        'R' => 0,
        'G' => 1,
        _ => 2,
    }).collect();
    
    let mut answer = 0;
    
    for i in 0..3 {
        // i は i64 と推論される（x: i64 との演算）。+3 してから % 3 で負を回避。
        let mut seq2: Vec<i64> = seq3.iter().map(|&x| (i - x + 3) % 3).collect();

        // Forward hash
        let mut power3 = 1;
        let mut hash1 = 0;
        let mut hash2 = 0;
        
        for j in 0..n {
            // S の先頭 j+1 文字を3進数とみなしたハッシュ。
            hash1 = (hash1 * 3 + seq1[j]) % MOD;
            // seq2 の末尾から j+1 文字を逆順に読んだハッシュ（重み power3 を掛けて足す）。
            hash2 = (hash2 + power3 * seq2[n - j - 1]) % MOD;
            if hash1 == hash2 {
                answer += 1;
            }
            power3 = power3 * 3 % MOD;
        }

        // Reverse hash
        power3 = 1;
        hash1 = 0;
        hash2 = 0;
        
        // 逆方向（T を右にずらす）の場合。完全一致の j = n-1 は上で数えたので n-1 まで。
        for j in 0..(n - 1) {
            hash1 = (hash1 + power3 * seq1[n - j - 1]) % MOD;
            hash2 = (hash2 * 3 + seq2[j]) % MOD;
            if hash1 == hash2 {
                answer += 1;
            }
            power3 = power3 * 3 % MOD;
        }
    }
    
    println!("{}", answer);
}
