use proconio::input;

fn main() {
    // Step #1. Input
    input! {
        n: usize, // Number of vertices
        m: usize, // Number of edges
    }
    
    let mut g = vec![Vec::new(); n]; // Adjacency list

    // Step #2. Read edges
    for _ in 0..m {
        input! {
            a: usize,
            b: usize,
        }
        g[a - 1].push(b - 1); // Convert to 0-based index
        g[b - 1].push(a - 1); // Undirected graph
    }

    // Step #3. Count vertices with exactly one smaller neighbor
    let mut answer = 0;
    for i in 0..n {
        let cnt = g[i].iter().filter(|&&j| j < i).count(); // Count neighbors smaller than i
        if cnt == 1 {
            answer += 1; // Increment answer if exactly one smaller neighbor
        }
    }

    // Step #4. Output the result
    println!("{}", answer);
}
