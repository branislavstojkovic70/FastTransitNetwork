use crate::graph::graph::Graph;
use std::collections::VecDeque;

pub fn bfs_sequential(graph: &Graph, source: usize) -> Vec<i32> {
    let mut dist = vec![-1; graph.num_nodes];
    
    if !graph.is_valid_node(source) {
        eprintln!("Invalid source node: {}", source);
        return dist;
    }
    
    let mut queue = VecDeque::new();
    dist[source] = 0;
    queue.push_back(source);
    
    while let Some(u) = queue.pop_front() {
        for &v in graph.neighbors(u) {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }
    
    dist
}

pub fn print_bfs_result(dist: &[i32], source: usize) {
    println!("\nBFS from node {}:", source);
    for (v, &d) in dist.iter().enumerate() {
        if d >= 0 {
            println!("  Node {}: distance = {}", v, d);
        } else {
            println!("  Node {}: unreachable", v);
        }
    }

    let reachable = dist.iter().filter(|&&d| d >= 0).count();
    println!("Reachable nodes: {}/{}", reachable, dist.len());
}