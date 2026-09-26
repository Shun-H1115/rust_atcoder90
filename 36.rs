// ==========================================================================
// 典型90 #036  Max Manhattan Distance  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_aj
// ==========================================================================
// 【アルゴリズム】 45度回転（(x, y) → (x+y, y-x) でマンハッタン距離がチェビシェフ距離 max(|dX|, |dY|) になる）
// 【計算量】       O(N + Q)
// 【学習計画】     第5週（★5前半）
//
// 【このファイルで覚えるRust文法】
//   - proconio を使わない標準入力の読み方: io::stdin().read_line(&mut s) + split_whitespace + parse
//   - parse() は型推論で変換先が決まる（let n: usize = ...parse().unwrap()）
//   - input.clear() … read_line は追記するので、毎回クリアが必要（Py の input() との違い）
//   - 1i64 << 60 … サフィックスで i64 に固定
//
// 【Pythonで書くと（考え方の対応）】
//   X = [x + y for x, y in P]; Y = [y - x for x, y in P]
//   for t in queries:
//       i = t - 1
//       print(max(X[i] - min(X), max(X) - X[i], Y[i] - min(Y), max(Y) - Y[i]))
//
// 【注意・改善ポイント】
//   ! 標準の read_line を1行ずつ呼ぶのは遅く、コードも長い。競プロでは proconio を使うのが基本。
//     このファイルは「proconio が無い環境（業務コード等）での入力処理」の参考として読むとよい。
// ==========================================================================

use std::cmp::{max, min};
use std::io;

fn main() {
    // 読み込み用のバッファ。String::new() は空文字列。
    let mut input = String::new();
    
    // Step #1. Input
    // read_line(&mut input) … バッファに1行追記。戻り値は Result なので unwrap。Py: input()
    io::stdin().read_line(&mut input).unwrap();
    // split_whitespace() … Py: input().split()（イテレータで返る）
    let mut input_iter = input.split_whitespace();
    // next() で次の要素（Option<&str>）→ unwrap → parse() で数値に。型は左辺の : usize で決まる。
    let n: usize = input_iter.next().unwrap().parse().unwrap();
    let q: usize = input_iter.next().unwrap().parse().unwrap();

    // 要素型は後の 1i64 << 60 との比較から i64 に推論される。
    let mut x = vec![0; n + 1];
    let mut y = vec![0; n + 1];
    let mut t = vec![0; q + 1];

    for i in 1..=n {
        // clear しないと前の行の後ろに追記されてしまう。
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let mut xy_iter = input.split_whitespace();
        x[i] = xy_iter.next().unwrap().parse().unwrap();
        y[i] = xy_iter.next().unwrap().parse().unwrap();
    }

    for i in 1..=q {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        // trim() … 末尾の改行を除く。Py: input().strip()
        t[i] = input.trim().parse().unwrap();
    }

    // Step #2. Rotate by 45 degrees
    let mut min_x = 1i64 << 60;
    let mut max_x = -(1i64 << 60);
    let mut min_y = 1i64 << 60;
    let mut max_y = -(1i64 << 60);

    for i in 1..=n {
        // 45度回転（√2 倍の拡大を含むが、距離の比較には影響しない）。
        let p1 = x[i] + y[i];
        let p2 = y[i] - x[i];
        x[i] = p1;
        y[i] = p2;

        min_x = min(min_x, x[i]);
        max_x = max(max_x, x[i]);
        min_y = min(min_y, y[i]);
        max_y = max(max_y, y[i]);
    }

    // Step #3. Answer Queries
    for i in 1..=q {
        let idx = t[i];
        let ret1 = (x[idx] - min_x).abs();
        let ret2 = (x[idx] - max_x).abs();
        let ret3 = (y[idx] - min_y).abs();
        let ret4 = (y[idx] - max_y).abs();

        // max は2引数なので入れ子にする。[ret1, ret2, ret3, ret4].iter().max().unwrap() とも書ける。
        let answer = max(max(ret1, ret2), max(ret3, ret4));
        println!("{}", answer);
    }
}
