use std::io::{self, Read, Write};

fn read_token<T: std::str::FromStr>() -> T {
    // 1トークンだけ標準入力から読み取る（空白区切り）
    let mut buf = Vec::with_capacity(32);
    let mut byte = [0u8; 1];
    let mut started = false;
    loop {
        let n = io::stdin().read(&mut byte).unwrap();
        if n == 0 {
            break;
        }
        let b = byte[0];
        if b.is_ascii_whitespace() {
            if started {
                break;
            } else {
                continue;
            }
        } else {
            started = true;
            buf.push(b);
        }
    }
    let s = unsafe { std::str::from_utf8_unchecked(&buf) };
    s.parse().ok().unwrap()
}

fn ask(pos: i64) -> i64 {
    // クエリを出力して必ず flush
    println!("? {}", pos);
    io::stdout().flush().unwrap();
    // 応答を1トークン読み取る
    read_token::<i64>()
}

fn solve() {
    let n: i64 = read_token();
    let mut answer: i64 = 0;
    for i in 1..=n {
        let t = ask(i);
        if t > answer {
            answer = t;
        }
    }
    println!("! {}", answer);
    io::stdout().flush().unwrap();
}

fn main() {
    // 高速化用の行バッファリングは使わず、逐次読み取り
    let t: i32 = read_token();
    for _ in 0..t {
        solve();
    }
}