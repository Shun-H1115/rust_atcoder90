use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    }

    // Initialize the graph and in-degree array
    let mut graph = vec![Vec::new(); n];
    let mut in_degree = vec![0; n];

    // Read the edges and build the graph
    for _ in 0..m {
        input! {
            mut a: usize,
            mut b: usize,
        }
        a -= 1;
        b -= 1;
        graph[a].push(b);
        in_degree[b] += 1;
    }

    // Stack for nodes with in-degree 0 and other data structures
    let mut stack = Vec::new();
    let mut perm = vec![-1; n];
    let mut answer_list = Vec::new();

    // Start with nodes that have in-degree of 0
    for i in 0..n {
        if in_degree[i] == 0 {
            stack.push(i);
        }
    }

    // Recursive DFS function for generating topological orders
    fn dfs(
        depth: usize,
        n: usize,
        k: usize,
        graph: &Vec<Vec<usize>>,
        in_degree: &mut Vec<usize>,
        stack: &mut Vec<usize>,
        perm: &mut Vec<i32>,
        answer_list: &mut Vec<Vec<i32>>,
    ) -> bool {
        // If a topological order is complete, add it to the answer list
        if depth == n {
            answer_list.push(perm.clone());
            return true;
        }
        
        if stack.is_empty() {
            return false;
        }
        
        for i in (0..stack.len()).rev() {
            if answer_list.len() == k {
                break;
            }

            // Process the current node
            let x = stack[i];
            stack.remove(i);
            
            // Add children of x with in-degree 0 to the stack
            for &j in &graph[x] {
                in_degree[j] -= 1;
                if in_degree[j] == 0 {
                    stack.push(j);
                }
            }

            perm[depth] = x as i32;

            // Recursive call
            if !dfs(depth + 1, n, k, graph, in_degree, stack, perm, answer_list) {
                return false;
            }

            // Backtrack: revert changes
            for &j in &graph[x] {
                if in_degree[j] == 0 {
                    stack.pop();
                }
                in_degree[j] += 1;
            }
            stack.insert(i, x);
        }
        true
    }

    dfs(0, n, k, &graph, &mut in_degree, &mut stack, &mut perm, &mut answer_list);

    // Output the results or -1 if there aren't enough answers
    if answer_list.len() != k {
        println!("-1");
    } else {
        for v in answer_list {
            for (i, &x) in v.iter().enumerate() {
                if i != 0 {
                    print!(" ");
                }
                print!("{}", x + 1);
            }
            println!();
        }
    }
}
