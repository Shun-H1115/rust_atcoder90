use proconio::input;

fn prime_factors(n: i64) -> Vec<i64> {
    let mut rem = n;
    let mut p = Vec::new();
    
    // Finding prime factors up to sqrt(n)
    for i in 2..=((n as f64).sqrt() as i64) {
        while rem % i == 0 {
            rem /= i;
            p.push(i);
        }
    }
    
    // If there's a remaining factor greater than sqrt(n)
    if rem != 1 {
        p.push(rem);
    }
    
    p
}

fn main() {
    // Step #1. Input
    input! {
        n: i64,
    }
    
    // Step #2. Get Answer
    let vec = prime_factors(n);
    let mut answer = 0;
    
    // Determine the smallest power of 2 greater than or equal to vec.len()
    for i in 0..=20 {
        if (1 << i) >= vec.len() as i32 {
            answer = i;
            break;
        }
    }
    
    // Output the answer
    println!("{}", answer);
}
