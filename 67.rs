// ==========================================================================
// 典型90 #067  Base 8 to 9  (★2)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bo
// ==========================================================================
// 【アルゴリズム】 シミュレーション（8進→10進→9進→8を5に置換 を K 回）
// 【計算量】       O(K × 桁数)
// 【学習計画】     第1週（★2）
//
// 【このファイルで覚えるRust文法】
//   - &str を1文字ずつ処理して数値化（Py: int(s, 8)。Rust にも i64::from_str_radix(&n, 8) がある）
//   - char::from_digit(d, 10) … 数字→文字（Py: str(d)）。Option<char> を返す
//   - res.insert(0, c) … 先頭に挿入（O(長さ)）。push してから最後に反転する方が速い
//   - n.chars().map(...).collect() … Py: s.replace('8', '5')（Rust にも n.replace('8', "5") がある）
//   - "0".to_string() … &str → String
//
// 【Pythonで書くと（考え方の対応）】
//   for _ in range(K):
//       x = int(N, 8)
//       s = ''
//       while x: s = str(x % 9) + s; x //= 9
//       N = (s or '0').replace('8', '5')
//   print(N)
// ==========================================================================

use proconio::input;

// 標準に i64::from_str_radix(n, 8).unwrap() があり、この関数は1行で置き換えられる。
fn base8_to_long(n: &str) -> i64 {
    // Converts a base-8 string to a decimal (base-10) integer.
    let mut res = 0;
    for ch in n.chars() {
        // ch as i64 - '0' as i64 … 文字コードの差で数字に。
        res = res * 8 + (ch as i64 - '0' as i64);
    }
    res
}

fn long_to_base9(mut n: i64) -> String {
    // Converts a decimal (base-10) integer to a base-9 string.
    if n == 0 {
        // &str リテラルを String に変換して返す。
        return "0".to_string();
    }
    let mut res = String::new();
    while n > 0 {
        // insert(0, ..) は毎回全体をずらすので遅め。push → 最後に res.chars().rev().collect() の方が定石。
        res.insert(0, char::from_digit((n % 9) as u32, 10).unwrap());
        n /= 9;
    }
    res
}

fn main() {
    input! {
        // mut n: String … ループで作り直すので可変で受け取る。
        mut n: String,  // Base-8 number as a string
        k: usize,       // Number of transformations
    }

    for _ in 0..k {
        // Convert from base-8 to decimal (base-10)
        // mut は不要（再代入していないので warning）。
        let mut num_in_base10 = base8_to_long(&n);
        
        // Convert from decimal to base-9
        n = long_to_base9(num_in_base10);
        
        // Replace all '8' characters with '5'
        // Py: n = n.replace('8', '5')。Rust でも n = n.replace('8', "5"); と書ける。
        n = n.chars().map(|c| if c == '8' { '5' } else { c }).collect();
    }

    println!("{}", n);
}
