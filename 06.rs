// ==========================================================================
// 典型90 #006  Smallest Subsequence  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_f
// ==========================================================================
// 【アルゴリズム】 貪欲法＋前計算（nex[i][c] = 位置i以降で文字cが最初に出る位置）
// 【計算量】       O(26N)
// 【学習計画】     第5週（★5前半）
//
// 【このファイルで覚えるRust文法】
//   - s.chars().collect::<Vec<char>>() … 文字列を添字アクセスできる配列に変換（Rust の String は s[i] で1文字取れない！）
//   - 文字とコードの変換: (c as u8 - b'a') as usize … Py: ord(c) - ord('a')
//   - (b'a' + j as u8) as char … Py: chr(ord('a') + j)
//   - if 式が値を返す: let x = if cond { a } else { b }; … Py の三項演算子
//
// 【Pythonで書くと（考え方の対応）】
//   nex = [[n]*26 for _ in range(n+1)]
//   for i in range(n-1, -1, -1):
//       for j in range(26):
//           nex[i][j] = i if ord(s[i]) - 97 == j else nex[i+1][j]
//
// 【注意・改善ポイント】
//   ! 34行目 s.len() - nex_pos - 1 + i は nex_pos == n のとき 0 - 1 で usize がアンダーフローする。
//     デバッグビルド(cargo run)では panic、リリースビルドではラップアラウンドで結果的に偶然正しく動く。
//     安全な書き方: 移項して `n - nex_pos + i - 1 >= k` の順に計算するか、先に `if nex_pos == n { continue; }`。
//     usize の引き算は Python エンジニアが最もハマる罠なので、この行で覚えておこう。
// ==========================================================================

use proconio::input;

fn main() {
    // Step #1. Input
    input! {
        n: usize,
        k: usize,
        s: String,
    }

    // Rust の String は UTF-8 のバイト列なので s[i] で文字を取れない。
    // Vec<char> にすれば s_chars[i] でアクセス可能。競プロでは最初にこれをやるのが定石。
    // （proconio なら s: Chars と書けば最初から Vec<char> で受け取れる）
    let s_chars: Vec<char> = s.chars().collect();
    // nex[i][c] の初期値は n（＝「存在しない」の番兵）。
    let mut nex = vec![vec![n; 26]; n + 1];

    // Step #2. Precompute positions for each character
    for i in 0..26 {
        nex[n][i] = n;
    }
    for i in (0..n).rev() {
        for j in 0..26 {
            // s_chars[i] as u8 - b'a' … b'a' は u8 型の 97。Py: ord(s[i]) - ord('a')
            nex[i][j] = if (s_chars[i] as u8 - b'a') as usize == j {
                i
            } else {
                nex[i + 1][j]
            };
        }
    }

    // Step #3. Greedily build the answer one character at a time
    let mut answer = String::new();
    let mut current_pos = 0;
    for i in 1..=k {
        for j in 0..26 {
            let nex_pos = nex[current_pos][j];
            // （注意）ここは usize の引き算。nex_pos == n だと途中で負になりアンダーフローする。上の【注意】参照。
            let max_possible_length = s.len() - nex_pos - 1 + i;
            if max_possible_length >= k {
                // j as u8 してから b'a' に足し、as char で文字に戻す。Py: chr(97 + j)
                answer.push((b'a' + j as u8) as char);
                current_pos = nex_pos + 1;
                // break … 内側の for を抜ける（Py と同じ）。
                break;
            }
        }
    }

    // Step #4. Output the result
    println!("{}", answer);
}
