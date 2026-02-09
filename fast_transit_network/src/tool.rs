mod cli;

use clap::Parser;
use fast_transit_network::graph::graph::load_graph_from_file;
use fast_transit_network::algorithms::bfs::{bfs_sequential, bfs_parallel};
use fast_transit_network::algorithms::wcc::run_wcc_and_save;
use fast_transit_network::utils::io::write_bfs_result;
use std::time::Instant;

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
        
        cli::Commands::Benchmark { input, threads } => {
            println!("Loading graph from: {}", input);
            let graph = load_graph_from_file(&input)?;
            graph.print_info();
            
            let thread_counts: Vec<usize> = threads
                .split(',')
                .map(|s| s.trim().parse().unwrap())
                .collect();
            
            println!("\n{}", "=".repeat(70));
            println!("BFS BENCHMARK");
            println!("{}", "=".repeat(70));

            let start = Instant::now();
            let dist_seq = bfs_sequential(&graph, 0);
            let time_seq = start.elapsed();
            println!("Sequential: {:?}", time_seq);

            for &num_threads in &thread_counts {
                let start = Instant::now();
                let dist_par = bfs_parallel(&graph, 0, num_threads);
                let time_par = start.elapsed();
                
                let speedup = time_seq.as_secs_f64() / time_par.as_secs_f64();
                let correct = dist_seq == dist_par;
                
                println!("Parallel ({}): {:?} | Speedup: {:.2}x | {}", 
                         num_threads, time_par, speedup, 
                         if correct { "OK" } else { "ERROR" });
            }
            
            println!("\n{}", "=".repeat(70));
            println!("WCC BENCHMARK");
            println!("{}", "=".repeat(70));

            use fast_transit_network::algorithms::wcc::{wcc_sequential, wcc_parallel, wcc_stats};

            let start = Instant::now();
            let comp_seq = wcc_sequential(&graph);
            let time_seq = start.elapsed();
            let stats_seq = wcc_stats(&comp_seq);
            println!("Sequential: {:?} | {} components", time_seq, stats_seq.num_components);

            for &num_threads in &thread_counts {
                let start = Instant::now();
                let comp_par = wcc_parallel(&graph, num_threads);
                let time_par = start.elapsed();
                
                let stats_par = wcc_stats(&comp_par);
                let speedup = time_seq.as_secs_f64() / time_par.as_secs_f64();
                let correct = stats_seq.num_components == stats_par.num_components;
                
                println!("Parallel ({}): {:?} | {} components | Speedup: {:.2}x | {}", 
                         num_threads, time_par, stats_par.num_components, speedup,
                         if correct { "OK" } else { "ERROR" });
            }
            
            Ok(())
        }
    }
}
