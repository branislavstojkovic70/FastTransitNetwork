mod graph;
mod algorithms;

use graph::graph::load_graph_from_file;
use algorithms::bfs::{bfs_sequential, bfs_parallel};
use std::time::Instant;

use crate::algorithms::union_find::UnionFind;

fn test_union_find() {
    println!("\n{}", "=".repeat(70));
    println!("TEST: Union-Find structure");
    println!("{}", "=".repeat(70));

    let mut uf = UnionFind::new(6);

    println!("Initial state: 6 nodes, each in its own component");
    println!("Components: {}", uf.count_components());

    println!("\nUnions:");
    uf.union(0, 1);
    println!("  union(0, 1) -> Components: {}", uf.count_components());

    uf.union(1, 2);
    println!("  union(1, 2) -> Components: {}", uf.count_components());

    uf.union(3, 4);
    println!("  union(3, 4) -> Components: {}", uf.count_components());

    println!("\nFinal components:");
    let components = uf.get_components();
    for (node, comp) in components.iter().enumerate() {
        println!("  Node {}: component {}", node, comp);
    }

    println!("\nTotal components: {}", uf.count_components());
}

fn main() {
    test_union_find();
}