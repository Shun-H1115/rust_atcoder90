use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [i32; n],
    }

    let mut shifts = 0;

    for _ in 0..q {
        input! {
            t: i32,
            x: usize,
            y: usize,
        }

        match t {
            1 => {
                // Perform a swap
                let xi = (x - 1 + shifts) % n;
                let yi = (y - 1 + shifts) % n;
                a.swap(xi, yi);
            }
            2 => {
                // Shift operation
                shifts = (shifts + n - 1) % n;
            }
            3 => {
                // Print the result
                let xi = (x - 1 + shifts) % n;
                println!("{}", a[xi]);
            }
            _ => {}
        }
    }
}
