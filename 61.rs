// ==========================================================================
// 典型90 #061  Deck  (★2)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bi
// ==========================================================================
// 【アルゴリズム】 両端追加の配列（中央から左右に伸ばす。VecDeque でも可）
// 【計算量】       O(Q)
// 【学習計画】     第1週（★2）
//
// 【このファイルで覚えるRust文法】
//   - オフセット付き配列で deque を模擬（大きな配列の中央から開始）
//   - （別解）VecDeque::push_front / push_back と dq[x - 1] で添字アクセス可能
//   - match の _ => () … 何もしない腕（() は Py の None のような「空の値」）
//
// 【Pythonで書くと（考え方の対応）】
//   from collections import deque
//   dq = deque()
//   for t, x in Q:
//       if t == 1: dq.appendleft(x)
//       elif t == 2: dq.append(x)
//       else: print(dq[x-1])
// ==========================================================================

use proconio::input;

fn main() {
    // Step 1: Input
    input! {
        q: usize,
        queries: [(i32, i32); q],
    }

    // Step 2: Simulate operations
    const OFFSET: usize = 500_000;
    // 要素型は a[cl] = x（i32）から推論される。1 << 20 ≒ 100万要素。
    let mut a = vec![0; 1 << 20];
    let mut cl = OFFSET;
    let mut cr = OFFSET;

    // queries を消費しながら (t, x) に分解。
    for (t, x) in queries {
        match t {
            1 => {
                cl -= 1;
                a[cl] = x;
            }
            2 => {
                a[cr] = x;
                cr += 1;
            }
            3 => {
                // x は i32 なので (x - 1) as usize で添字に変換。
                println!("{}", a[cl + (x - 1) as usize]);
            }
            // () はユニット型。何もしないことを表す。{} と書いても同じ。
            _ => (),
        }
    }
}
