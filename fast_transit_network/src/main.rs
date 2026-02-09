mod graph;
mod algorithms;
mod utils;

use graph::graph::load_graph_from_file;
use algorithms::pagerank::{pagerank_sequential, pagerank_stats, PageRankConfig};
use std::time::Instant;

fn test_pagerank() {
    println!("\n{}", "=".repeat(70));
    println!("TEST: PageRank Sequential");
    println!("{}", "=".repeat(70));
    
    let graph = load_graph_from_file("test_graph.txt")
        .expect("Failed to load graph");
    
    graph.print_info();
    
    let config = PageRankConfig::default();
    
    println!("\nPageRank Config:");
    println!("  Alpha: {}", config.alpha);
    println!("  Max iterations: {}", config.max_iterations);
    println!("  Tolerance: {:.2e}", config.tolerance);
    
    println!("\nRunning PageRank...");
    let start = Instant::now();
    let ranks = pagerank_sequential(&graph, &config);
    let elapsed = start.elapsed();
    
    println!("Completed in {:?}", elapsed);
    println!();
    pagerank_stats(&ranks);
    
    println!("\nAll node ranks:");
    for (node, rank) in ranks.iter().enumerate() {
        println!("  Node {}: {:.6}", node, rank);
    }
}

fn main() {
    test_pagerank();
}