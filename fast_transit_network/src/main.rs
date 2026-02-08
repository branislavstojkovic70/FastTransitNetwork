mod graph;
mod algorithms;

use graph::graph::load_graph_from_file;
use algorithms::bfs::{bfs_sequential, bfs_parallel};
use std::time::Instant;

fn benchmark_bfs(graph_path: &str, source: usize) {
    println!("\n{}", "=".repeat(70));
    println!("Graph: {}", graph_path);
    println!("{}", "=".repeat(70));

    let graph = match load_graph_from_file(graph_path) {
        Ok(g) => {
            println!("Graph loaded successfully.");
            g
        }
        Err(e) => {
            eprintln!("Error loading graph: {}", e);
            return;
        }
    };

    graph.print_info();

    if !graph.is_valid_node(source) {
        eprintln!("Source node {} is invalid.", source);
        return;
    }

    println!("\nSource node: {}", source);
    println!("{}", "-".repeat(70));

    print!("Sequential BFS... ");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    let start = Instant::now();
    let dist_seq = bfs_sequential(&graph, source);
    let time_seq = start.elapsed();

    let reachable_seq = dist_seq.iter().filter(|&&d| d >= 0).count();
    println!("{:?} | Reachable: {}/{}", time_seq, reachable_seq, graph.num_nodes);

    let thread_counts = vec![2, 4, 8, 16, 32];

    for num_threads in thread_counts {
        print!("Parallel BFS ({} threads)... ", num_threads);
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let start = Instant::now();
        let dist_par = bfs_parallel(&graph, source, num_threads);
        let time_par = start.elapsed();

        let reachable_par = dist_par.iter().filter(|&&d| d >= 0).count();
        let speedup = time_seq.as_secs_f64() / time_par.as_secs_f64();
        let correct = dist_seq == dist_par;
        let status = if correct { "OK" } else { "MISMATCH" };

        println!(
            "{:?} | Reachable: {}/{} | Speedup: {:.2}x | {}",
            time_par, reachable_par, graph.num_nodes, speedup, status
        );

        if !correct {
            println!("  Differences:");
            for (v, (&d_seq, &d_par)) in dist_seq.iter().zip(dist_par.iter()).enumerate() {
                if d_seq != d_par {
                    println!("    Node {}: seq={}, par={}", v, d_seq, d_par);
                    if v > 10 {
                        println!(
                            "    ... and {} more",
                            dist_seq
                                .iter()
                                .zip(dist_par.iter())
                                .filter(|&(a, b)| a != b)
                                .count()
                                - 10
                        );
                        break;
                    }
                }
            }
        }
    }
}

fn main() {
    println!("\n{}", "=".repeat(70));
    println!("FastTransitNetwork - BFS Benchmark");
    println!("{}", "=".repeat(70));
    
    let test_cases = vec![
        ("test_graph.txt", 0),
        ("scripts/data/small/random_1k.txt", 0),
        ("scripts/data/small/random_10k.txt", 0),
        ("scripts/data/medium/random_100k.txt", 0),
        ("scripts/data/medium/scale_free_100k.txt", 0),
        ("scripts/data/medium/grid_100k.txt", 0),
        ("scripts/data/medium/chain_100k.txt", 0),
        ("scripts/data/heavy/random_100m.txt", 0),
        ("scripts/data/heavy/scale_free_100m.txt", 0),
        ("scripts/data/heavy/chain_100m.txt", 0),
        ("scripts/data/heavy/grid_100m.txt", 0),
    ];

    for (graph_path, source) in test_cases {
        if std::path::Path::new(graph_path).exists() {
            benchmark_bfs(graph_path, source);
        } else {
            println!("\nSkipping {} (file not found)", graph_path);
        }
    }

    println!("\n{}", "=".repeat(70));
    println!("Benchmark finished.");
    println!("{}", "=".repeat(70));
}