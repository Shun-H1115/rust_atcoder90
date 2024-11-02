use proconio::input;

const MOD: i64 = 699_999_953; // large prime number

fn main() {
    input! {
        n: usize,
        s: String,
        t: String,
    }
    
    let seq1: Vec<i64> = s.chars().map(|c| match c {
        'R' => 0,
        'G' => 1,
        _ => 2,
    }).collect();

    let seq3: Vec<i64> = t.chars().map(|c| match c {
        'R' => 0,
        'G' => 1,
        _ => 2,
    }).collect();
    
    let mut answer = 0;
    
    for i in 0..3 {
        let mut seq2: Vec<i64> = seq3.iter().map(|&x| (i - x + 3) % 3).collect();

        // Forward hash
        let mut power3 = 1;
        let mut hash1 = 0;
        let mut hash2 = 0;
        
        for j in 0..n {
            hash1 = (hash1 * 3 + seq1[j]) % MOD;
            hash2 = (hash2 + power3 * seq2[n - j - 1]) % MOD;
            if hash1 == hash2 {
                answer += 1;
            }
            power3 = power3 * 3 % MOD;
        }

        // Reverse hash
        power3 = 1;
        hash1 = 0;
        hash2 = 0;
        
        for j in 0..(n - 1) {
            hash1 = (hash1 + power3 * seq1[n - j - 1]) % MOD;
            hash2 = (hash2 * 3 + seq2[j]) % MOD;
            if hash1 == hash2 {
                answer += 1;
            }
            power3 = power3 * 3 % MOD;
        }
    }
    
    println!("{}", answer);
}
