// ==========================================================================
// 典型90 #068  Paired Information  (★5)
// 問題: https://atcoder.jp/contests/typical90/tasks/typical90_bp
// ==========================================================================
// 【アルゴリズム】 オフライン処理＋ポテンシャル（全クエリを先読みして A を1つ決め、未確定の区間を BTreeSet で管理）
// 【計算量】       O((N + Q) log N)
// 【学習計画】     第6週（★5後半）
//
// 【このファイルで覚えるRust文法】
//   - BTreeSet … 順序付き集合（Py には標準で無い。sortedcontainers.SortedSet 相当）
//   - set.range(p..).next() … p 以上の最小要素（lower_bound）。Rust の順序付きコンテナの強み
//   - for (t, x, y, v) in &queries … 参照で走査するので t, x, y, v はすべて &型。*t で値に
//   - x - 1 は &usize - i32リテラル でも動く（参照に対する算術演算は実装済み）
//
// 【Pythonで書くと（考え方の対応）】
//   # pot[i+1] = sum[i] - pot[i] （0番を 0 とした仮の値）
//   # 真の値 A_x = V のとき A_y = pot[y] ± (V - pot[x])（距離の偶奇で符号が決まる）
//   # x..y の間に未確定の和が1つでもあれば Ambiguous
// ==========================================================================

use proconio::input;
// BTreeSet は二分木ベースで要素がソート順に並ぶ。HashSet と違い range 検索ができる。
use std::collections::BTreeSet;

fn main() {
    // Read input
    input! {
        n: usize,
        q: usize,
        queries: [(i32, usize, usize, i64); q],
    }

    // Initialize values
    // sum[i] = A[i] + A[i+1] の値（未確定なら 0 のまま）。
    let mut sum = vec![0; n - 1];
    let mut t_vec = Vec::new();
    let mut x_vec = Vec::new();
    let mut y_vec = Vec::new();
    let mut v_vec = Vec::new();

    // Parse each query and update sum for type 0 queries
    // &queries で借用して走査 → 各要素は参照。*t, *v で参照外し。
    for (t, x, y, v) in &queries {
        if *t == 0 {
            // x - 1 … x は &usize だが、参照同士/参照と値の算術は自動で解決される。
            sum[x - 1] = *v;
        }
        t_vec.push(*t);
        x_vec.push(x - 1);
        y_vec.push(y - 1);
        v_vec.push(*v);
    }

    // Calculate potential values
    let mut pot = vec![0_i64; n];
    for i in 0..n - 1 {
        // 仮に A[0] = 0 として、和の条件から順に決めたポテンシャル。
        pot[i + 1] = sum[i] as i64 - pot[i];
    }

    // Set up a BTreeSet to track unset values
    let mut unset_indices = BTreeSet::new();
    for i in 0..=n {
        // i as i32 - 1 … 番兵 -1 と n-1 を含めるため符号付きにしている。
        unset_indices.insert(i as i32 - 1);
    }

    // Process each query and produce results
    for i in 0..q {
        if t_vec[i] == 0 {
            // &(x as i32) … remove は参照を受け取る。
            unset_indices.remove(&(x_vec[i] as i32));
        } else {
            let p = x_vec[i].min(y_vec[i]);
            // ここの q はクエリ数の q をシャドーイング（for の範囲 0..q は評価済みなので動作は正しいが紛らわしい）。
            let q = x_vec[i].max(y_vec[i]);
            // range(p..) … p 以上の要素のイテレータ。next() で最小のもの = lower_bound。
            let lower_bound = unset_indices.range(p as i32..).next();
            
            // Determine the output based on index ranges
            if let Some(&lb) = lower_bound {
                // p..q-1 の間に未確定の和がなければ x と y はつながっている。
                if lb > (q - 1) as i32 {
                    // 距離の偶奇で符号が変わる（交互に +/- が伝播する）。
                    let result = if (q - p) % 2 == 0 {
                        pot[y_vec[i]] + (v_vec[i] - pot[x_vec[i]])
                    } else {
                        pot[y_vec[i]] - (v_vec[i] - pot[x_vec[i]])
                    };
                    println!("{}", result);
                } else {
                    println!("Ambiguous");
                }
            }
        }
    }
}
