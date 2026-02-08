mod graph;
mod algorithms;

use graph::graph::load_graph_from_file;
use algorithms::wcc::{wcc_sequential, wcc_stats};

fn test_wcc() {
    println!("\n{}", "=".repeat(70));
    println!("TEST: WCC Sequential");
    println!("{}", "=".repeat(70));

    let graph = load_graph_from_file("test_graph.txt")
        .expect("Failed to load graph");

    graph.print_info();

    println!("\nComputing WCC...");
    let components = wcc_sequential(&graph);

    println!("\nResults:");
    for (node, comp) in components.iter().enumerate() {
        println!("  Node {}: component {}", node, comp);
    }

    let stats = wcc_stats(&components);
    println!();
    stats.print();
}

fn main() {
    test_wcc();
}