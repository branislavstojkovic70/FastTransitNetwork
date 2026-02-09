use clap::Parser;
use fast_transit_network::graph::graph::load_graph_from_file;
use fast_transit_network::algorithms::bfs::{bfs_sequential, bfs_parallel};
use fast_transit_network::algorithms::wcc::{wcc_sequential, wcc_parallel, wcc_stats, run_wcc_and_save};
use fast_transit_network::algorithms::pagerank::{pagerank_sequential, pagerank_parallel, run_pagerank_and_save, PageRankConfig};
use fast_transit_network::utils::io::write_bfs_result;
use fast_transit_network::utils::benchmark::{BenchmarkLogger, BenchmarkResult};
use fast_transit_network::cli;
use std::time::Instant;
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    
    match cli.command {
        cli::Commands::Bfs { input, source, mode, threads, out } => {
            println!("Loading graph from: {}", input);
            let graph = load_graph_from_file(&input)?;
            graph.print_info();
            
            println!("\nRunning BFS from source {}...", source);
            let start = Instant::now();
            
            let dist = match mode.as_str() {
                "seq" => bfs_sequential(&graph, source),
                "par" => bfs_parallel(&graph, source, threads),
                _ => {
                    eprintln!("Invalid mode: {}. Use 'seq' or 'par'", mode);
                    std::process::exit(1);
                }
            };
            
            let elapsed = start.elapsed();
            
            let reachable = dist.iter().filter(|&&d| d >= 0).count();
            println!("Completed in {:?}", elapsed);
            println!("Reachable nodes: {}/{}", reachable, graph.num_nodes);
            
            write_bfs_result(&dist, &out)?;
            println!("Results saved to: {}", out);
            
            Ok(())
        }
        
        cli::Commands::Wcc { input, mode, threads, out } => {
            println!("Loading graph from: {}", input);
            let graph = load_graph_from_file(&input)?;
            graph.print_info();
            
            let stats_path = out.replace(".txt", "_stats.txt");
            
            run_wcc_and_save(&graph, &mode, threads, &out, &stats_path)?;
            
            Ok(())
        }
        
        cli::Commands::Pagerank { input, mode, threads, out, alpha, iters, eps } => {
            println!("Loading graph from: {}", input);
            let graph = load_graph_from_file(&input)?;
            graph.print_info();
            
            let config = PageRankConfig {
                alpha,
                max_iterations: iters,
                tolerance: eps,
            };
            
            println!("\nPageRank Config:");
            println!("  Alpha: {}", config.alpha);
            println!("  Max iterations: {}", config.max_iterations);
            println!("  Tolerance: {:.2e}", config.tolerance);
            
            run_pagerank_and_save(&graph, &config, &mode, threads, &out)?;
            
            Ok(())
        }
        
        cli::Commands::Benchmark { input, threads, out } => {
            println!("Loading graph from: {}", input);
            let graph = load_graph_from_file(&input)?;
            graph.print_info();
            
            // Kreiraj benchmark logger
            let mut logger = BenchmarkLogger::new();
            
            // Ekstrakt ime grafa iz putanje
            let graph_name = Path::new(&input)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or(&input)
                .replace(".txt", "");
            
            let thread_counts: Vec<usize> = threads
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            
            println!("\n{}", "=".repeat(70));
            println!("BFS BENCHMARK");
            println!("{}", "=".repeat(70));
            
            // BFS Sequential
            let start = Instant::now();
            let dist_seq = bfs_sequential(&graph, 0);
            let time_seq = start.elapsed();
            let time_seq_ms = time_seq.as_secs_f64() * 1000.0;
            println!("Sequential: {:?}", time_seq);
            
            // Log sequential result
            logger.add_result(BenchmarkResult {
                algorithm: "BFS".to_string(),
                graph_name: graph_name.clone(),
                graph_nodes: graph.num_nodes,
                graph_edges: graph.num_edges,
                mode: "seq".to_string(),
                threads: 1,
                time_ms: time_seq_ms,
                speedup: 1.0,
                correct: true,
            });
            
            // BFS Parallel
            for &num_threads in &thread_counts {
                let start = Instant::now();
                let dist_par = bfs_parallel(&graph, 0, num_threads);
                let time_par = start.elapsed();
                let time_par_ms = time_par.as_secs_f64() * 1000.0;
                
                let speedup = time_seq_ms / time_par_ms;
                let correct = dist_seq == dist_par;
                
                println!("Parallel ({}): {:?} | Speedup: {:.2}x | {}", 
                         num_threads, time_par, speedup, 
                         if correct { "OK" } else { "ERROR" });
                
                // Log parallel result
                logger.add_result(BenchmarkResult {
                    algorithm: "BFS".to_string(),
                    graph_name: graph_name.clone(),
                    graph_nodes: graph.num_nodes,
                    graph_edges: graph.num_edges,
                    mode: "par".to_string(),
                    threads: num_threads,
                    time_ms: time_par_ms,
                    speedup,
                    correct,
                });
            }
            
            println!("\n{}", "=".repeat(70));
            println!("WCC BENCHMARK");
            println!("{}", "=".repeat(70));
            
            // WCC Sequential
            let start = Instant::now();
            let comp_seq = wcc_sequential(&graph);
            let time_seq = start.elapsed();
            let time_seq_ms = time_seq.as_secs_f64() * 1000.0;
            let stats_seq = wcc_stats(&comp_seq);
            println!("Sequential: {:?} | {} components", time_seq, stats_seq.num_components);
            
            // Log sequential result
            logger.add_result(BenchmarkResult {
                algorithm: "WCC".to_string(),
                graph_name: graph_name.clone(),
                graph_nodes: graph.num_nodes,
                graph_edges: graph.num_edges,
                mode: "seq".to_string(),
                threads: 1,
                time_ms: time_seq_ms,
                speedup: 1.0,
                correct: true,
            });
            
            // WCC Parallel
            for &num_threads in &thread_counts {
                let start = Instant::now();
                let comp_par = wcc_parallel(&graph, num_threads);
                let time_par = start.elapsed();
                let time_par_ms = time_par.as_secs_f64() * 1000.0;
                
                let stats_par = wcc_stats(&comp_par);
                let speedup = time_seq_ms / time_par_ms;
                let correct = stats_seq.num_components == stats_par.num_components;
                
                println!("Parallel ({}): {:?} | {} components | Speedup: {:.2}x | {}", 
                         num_threads, time_par, stats_par.num_components, speedup,
                         if correct { "OK" } else { "ERROR" });
                
                // Log parallel result
                logger.add_result(BenchmarkResult {
                    algorithm: "WCC".to_string(),
                    graph_name: graph_name.clone(),
                    graph_nodes: graph.num_nodes,
                    graph_edges: graph.num_edges,
                    mode: "par".to_string(),
                    threads: num_threads,
                    time_ms: time_par_ms,
                    speedup,
                    correct,
                });
            }
            
            println!("\n{}", "=".repeat(70));
            println!("PAGERANK BENCHMARK");
            println!("{}", "=".repeat(70));
            
            let config = PageRankConfig {
                alpha: 0.85,
                max_iterations: 50,
                tolerance: 1e-6,
            };
            
            // PageRank Sequential
            let start = Instant::now();
            let ranks_seq = pagerank_sequential(&graph, &config);
            let time_seq = start.elapsed();
            let time_seq_ms = time_seq.as_secs_f64() * 1000.0;
            println!("Sequential: {:?}", time_seq);
            
            // Log sequential result
            logger.add_result(BenchmarkResult {
                algorithm: "PageRank".to_string(),
                graph_name: graph_name.clone(),
                graph_nodes: graph.num_nodes,
                graph_edges: graph.num_edges,
                mode: "seq".to_string(),
                threads: 1,
                time_ms: time_seq_ms,
                speedup: 1.0,
                correct: true,
            });
            
            // PageRank Parallel
            for &num_threads in &thread_counts {
                let start = Instant::now();
                let ranks_par = pagerank_parallel(&graph, &config, num_threads);
                let time_par = start.elapsed();
                let time_par_ms = time_par.as_secs_f64() * 1000.0;
                
                let speedup = time_seq_ms / time_par_ms;
                
                let max_diff: f64 = ranks_seq.iter()
                    .zip(ranks_par.iter())
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0, f64::max);
                
                let correct = max_diff < 1e-4;
                
                println!("Parallel ({}): {:?} | Speedup: {:.2}x | {}", 
                         num_threads, time_par, speedup,
                         if correct { "OK" } else { "ERROR" });
                
                // Log parallel result
                logger.add_result(BenchmarkResult {
                    algorithm: "PageRank".to_string(),
                    graph_name: graph_name.clone(),
                    graph_nodes: graph.num_nodes,
                    graph_edges: graph.num_edges,
                    mode: "par".to_string(),
                    threads: num_threads,
                    time_ms: time_par_ms,
                    speedup,
                    correct,
                });
            }
            
            println!("\n{}", "=".repeat(70));
            if let Some(parent) = Path::new(&out).parent() {
                std::fs::create_dir_all(parent)?;
            }
            logger.save_to_csv(&out)?;
            println!("✓ Results saved to: {}", out);
            
            // Print summary
            logger.print_summary();
            
            println!("\n{}", "=".repeat(70));
            println!("Next steps:");
            println!("  1. Generate plots:");
            println!("     python3 scripts/visualize_benchmark.py {}", out);
            println!("  2. Check plots in: scripts/results/plots/");
            println!("{}", "=".repeat(70));
            
            Ok(())
        }
    }
}