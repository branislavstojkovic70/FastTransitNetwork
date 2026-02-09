mod graph;
mod algorithms;
mod utils; 

use graph::graph::load_graph_from_file;
use algorithms::wcc::{wcc_sequential, wcc_parallel, wcc_stats};
use std::time::Instant;

fn benchmark_wcc(graph_path: &str) {
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
    
    // Sequential
    print!("Sequential WCC... ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    
    let start = Instant::now();
    let comp_seq = wcc_sequential(&graph);
    let time_seq = start.elapsed();
    
    let stats = wcc_stats(&comp_seq);
    println!("{:?} | {} components", time_seq, stats.num_components);
    
    // Parallel
    for num_threads in [2, 4, 8, 16] {
        print!("Parallel WCC ({} threads)... ", num_threads);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();
        
        let start = Instant::now();
        let comp_par = wcc_parallel(&graph, num_threads);
        let time_par = start.elapsed();
        
        let stats_par = wcc_stats(&comp_par);
        let speedup = time_seq.as_secs_f64() / time_par.as_secs_f64();
        
        let correct = stats.num_components == stats_par.num_components;
        let status = if correct { "OK" } else { "ERROR" };
        
        println!("{:?} | {} comp | Speedup: {:.2}x | {}", 
                 time_par, stats_par.num_components, speedup, status);
    }
    
    println!();
    stats.print();
}

fn main() {
    let graphs = vec![
        "test_graph.txt",
        "scripts/data/medium/random_100k.txt",
        "scripts/data/heavy/random_100m.txt",
    ];
    
    for graph in graphs {
        if std::path::Path::new(graph).exists() {
            benchmark_wcc(graph);
        } else {
            println!("\nSkipping {} (file not found)", graph);
        }
    }
}