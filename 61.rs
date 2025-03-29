use proconio::input;

fn main() {
    // Step 1: Input
    input! {
        q: usize,
        queries: [(i32, i32); q],
    }

    // Step 2: Simulate operations
    const OFFSET: usize = 500_000;
    let mut a = vec![0; 1 << 20];
    let mut cl = OFFSET;
    let mut cr = OFFSET;

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
                println!("{}", a[cl + (x - 1) as usize]);
            }
            _ => (),
        }
    }
}
