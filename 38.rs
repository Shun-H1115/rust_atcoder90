// ==========================================================================
// 典型90 #038  Large LCM  (★3)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_al
// ==========================================================================
// 【アルゴリズム】 LCM = A / gcd × B。1e18 を超えるかを掛け算せずに割り算で判定
// 【計算量】       O(log min(A, B))
// 【学習計画】     第2週（★3前半）
//
// 【このファイルで覚えるRust文法】
//   - u64 の最大は約 1.8e19。A/g × B は最大 1e36 になりうるので直接掛けると溢れる
//   - 割り算で判定: b <= threshold / c ⇔ c * b <= threshold（切り捨て除算でも成り立つ）
//   - 別解1: c.checked_mul(b) → 溢れたら None
//   - 別解2: u128 で計算（(c as u128) * (b as u128)）
//
// 【Pythonで書くと（考え方の対応）】
//   from math import gcd
//   l = a // gcd(a, b) * b
//   print(l if l <= 10**18 else 'Large')   # Py は多倍長なので溢れを気にしなくてよい
// ==========================================================================

use proconio::input;

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    input! {
        a: u64,
        b: u64,
    }

    // 1e18。数値リテラルの _ は読みやすさのための区切り。
    let threshold: u64 = 1_000_000_000_000_000_000;
    // 先に割ってから掛けるのが LCM の定石（a * b / gcd だと途中で溢れやすい）。
    let c = a / gcd(a, b);

    // c * b > 1e18 を掛け算なしで判定。
    if b <= threshold / c {
        println!("{}", c * b);
    } else {
        println!("Large");
    }
}
