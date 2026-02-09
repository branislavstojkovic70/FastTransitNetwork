mod graph;
mod algorithms;
mod utils;

use graph::graph::load_graph_from_file;
use algorithms::pagerank::{
    pagerank_sequential, 
    pagerank_parallel, 
    pagerank_parallel_optimized,
    pagerank_stats, 
    PageRankConfig
};
use std::time::Instant;

fn benchmark_pagerank(graph_path: &str) {
    println!("\n{}", "=".repeat(70));
    println!("Graph: {}", graph_path);
    println!("{}", "=".repeat(70));
    
    let graph = match load_graph_from_file(graph_path) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    
    graph.print_info();
    
    let config = PageRankConfig {
        alpha: 0.85,
        max_iterations: 50,
        tolerance: 1e-6,
    };
    
    println!("\nConfig: alpha={}, max_iter={}, tol={:.0e}", 
             config.alpha, config.max_iterations, config.tolerance);
    println!("{}", "-".repeat(70));

    print!("Sequential PageRank... ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    
    let start = Instant::now();
    let ranks_seq = pagerank_sequential(&graph, &config);
    let time_seq = start.elapsed();
    
    println!("{:?}", time_seq);

    for num_threads in [2, 4, 8, 16, 32] {
        print!("Parallel PageRank ({} threads)... ", num_threads);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let start = Instant::now();
        let ranks_par = pagerank_parallel(&graph, &config, num_threads);
        let time_par = start.elapsed();
        
        let speedup = time_seq.as_secs_f64() / time_par.as_secs_f64();

        let max_diff: f64 = ranks_seq.iter()
            .zip(ranks_par.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);
        
        let correct = max_diff < 1e-4;
        let status = if correct { "OK" } else { "ERROR" };
        
        println!("{:?} | Speedup: {:.2}x | Max diff: {:.2e} | {}", 
                 time_par, speedup, max_diff, status);
    }

    if graph.num_nodes >= 100_000 {
        println!("\n--- Optimized Version ---");
        for num_threads in [8, 16, 32] {
            print!("Optimized Parallel ({} threads)... ", num_threads);
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
            
            let start = Instant::now();
            let ranks_opt = pagerank_parallel_optimized(&graph, &config, num_threads);
            let time_opt = start.elapsed();
            
            let speedup = time_seq.as_secs_f64() / time_opt.as_secs_f64();
            
            let max_diff: f64 = ranks_seq.iter()
                .zip(ranks_opt.iter())
                .map(|(a, b)| (a - b).abs())
                .fold(0.0, f64::max);
            
            let correct = max_diff < 1e-4;
            
            println!("{:?} | Speedup: {:.2}x | {}", 
                     time_opt, speedup, if correct { "OK" } else { "ERROR" });
        }
    }
    
    println!();
    pagerank_stats(&ranks_seq);
}

fn main() {
    let graphs = vec![
        "test_graph.txt",
        "scripts/data/small/random_1k.txt",
        "scripts/data/medium/random_100k.txt",
        "scripts/data/heavy/random_100m.txt",
    ];
    
    for graph in graphs {
        if std::path::Path::new(graph).exists() {
            benchmark_pagerank(graph);
        }
    }
}