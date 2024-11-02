use proconio::input;

fn base8_to_long(n: &str) -> i64 {
    // Converts a base-8 string to a decimal (base-10) integer.
    let mut res = 0;
    for ch in n.chars() {
        res = res * 8 + (ch as i64 - '0' as i64);
    }
    res
}

fn long_to_base9(mut n: i64) -> String {
    // Converts a decimal (base-10) integer to a base-9 string.
    if n == 0 {
        return "0".to_string();
    }
    let mut res = String::new();
    while n > 0 {
        res.insert(0, char::from_digit((n % 9) as u32, 10).unwrap());
        n /= 9;
    }
    res
}

fn main() {
    input! {
        mut n: String,  // Base-8 number as a string
        k: usize,       // Number of transformations
    }

    for _ in 0..k {
        // Convert from base-8 to decimal (base-10)
        let mut num_in_base10 = base8_to_long(&n);
        
        // Convert from decimal to base-9
        n = long_to_base9(num_in_base10);
        
        // Replace all '8' characters with '5'
        n = n.chars().map(|c| if c == '8' { '5' } else { c }).collect();
    }

    println!("{}", n);
}
