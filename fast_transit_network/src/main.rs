mod graph;

use graph::*;

fn main() {
    let edges = vec![
        (0, 1),
        (0, 2),
        (1, 2),
        (1, 3),
    ];
    
    let graph = build_csr(4, edges);
    graph.print_info();
    
    println!("\nNeighbors of each node:");
    for v in 0..graph.num_nodes {
        println!("  Node {}: {:?}", v, graph.neighbors(v));
    }
}