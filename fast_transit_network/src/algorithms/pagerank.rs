use crate::graph::graph::Graph;
use crate::utils::io::{write_pagerank_result, write_pagerank_stats, write_pagerank_top_nodes};
use anyhow::Result;
use rayon::prelude::*;

pub struct PageRankConfig {
    pub alpha: f64,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl Default for PageRankConfig {
    fn default() -> Self {
        Self {
            alpha: 0.85,
            max_iterations: 100,
            tolerance: 1e-6,
        }
    }
}

pub fn pagerank_sequential(graph: &Graph, config: &PageRankConfig) -> Vec<f64> {
    let n = graph.num_nodes;
    if n == 0 {
        return vec![];
    }

    let initial_value = 1.0 / n as f64;
    let mut rank = vec![initial_value; n];
    let mut new_rank = vec![0.0; n];
    let teleport = (1.0 - config.alpha) / n as f64;

    for iteration in 0..config.max_iterations {
        new_rank.fill(teleport);

        for u in 0..n {
            let neighbors = graph.neighbors(u);

            if neighbors.is_empty() {
                let contribution = config.alpha * rank[u] / n as f64;
                for v in 0..n {
                    new_rank[v] += contribution;
                }
            } else {
                let contribution = config.alpha * rank[u] / neighbors.len() as f64;
                for &v in neighbors {
                    new_rank[v] += contribution;
                }
            }
        }

        let delta: f64 = rank
            .iter()
            .zip(new_rank.iter())
            .map(|(old, new)| (old - new).abs())
            .sum();

        std::mem::swap(&mut rank, &mut new_rank);

        if delta < config.tolerance {
            println!(
                "PageRank converged after {} iterations (delta: {:.2e})",
                iteration + 1,
                delta
            );
            break;
        }

        if iteration == config.max_iterations - 1 {
            println!(
                "PageRank reached max iterations ({}) without full convergence (delta: {:.2e})",
                config.max_iterations, delta
            );
        }
    }

    rank
}

pub fn pagerank_parallel(
    graph: &Graph,
    config: &PageRankConfig,
    num_threads: usize,
) -> Vec<f64> {
    const THRESHOLD: usize = 10_000;
    if graph.num_nodes < THRESHOLD {
        return pagerank_sequential(graph, config);
    }

    let actual_threads = num_threads.min(8);
    let n = graph.num_nodes;
    let min_chunk = n / actual_threads; 

    let initial_value = 1.0 / n as f64;
    let mut rank = vec![initial_value; n];
    let mut new_rank = vec![0.0; n];
    let teleport = (1.0 - config.alpha) / n as f64;

    let sink_nodes: Vec<usize> = (0..n)
        .filter(|&u| graph.out_degree[u] == 0)
        .collect();

    rayon::ThreadPoolBuilder::new()
        .num_threads(actual_threads)
        .build()
        .expect("rayon pool")
        .install(|| {
    for iteration in 0..config.max_iterations {
        let sink_sum: f64 = sink_nodes.par_iter().map(|&u| rank[u]).sum();
        let sink_contribution = config.alpha * sink_sum / n as f64;
        let base_rank = teleport + sink_contribution;

        let contributions = (0..n)
            .into_par_iter()
            .with_min_len(min_chunk.max(1)) 
            .fold(
                || vec![0.0; n],
                |mut local_rank, u| {
                    let neighbors = graph.neighbors(u);
                    if !neighbors.is_empty() {
                        let contribution = config.alpha * rank[u] / neighbors.len() as f64;
                        for &v in neighbors {
                            local_rank[v] += contribution;
                        }
                    }
                    local_rank
                }
            )
            .reduce(
                || vec![0.0; n],
                |mut acc, local| {
                    acc.par_iter_mut()
                        .zip(local.par_iter())
                        .for_each(|(a, &l)| *a += l);
                    acc
                }
            );

        new_rank
            .par_iter_mut()
            .zip(contributions.par_iter())
            .for_each(|(r, &c)| {
                *r = base_rank + c;
            });

        let delta: f64 = rank
            .par_iter()
            .zip(new_rank.par_iter())
            .map(|(old, new)| (old - new).abs())
            .sum();

        std::mem::swap(&mut rank, &mut new_rank);

        if delta < config.tolerance {
            println!(
                "PageRank converged after {} iterations (delta: {:.2e})",
                iteration + 1,
                delta
            );
            break;
        }

        if iteration == config.max_iterations - 1 {
            println!(
                "PageRank reached max iterations without convergence (delta: {:.2e})",
                delta
            );
        }
    }

    rank
    })
}

pub fn pagerank_parallel_optimized(
    graph: &Graph,
    config: &PageRankConfig,
    num_threads: usize,
) -> Vec<f64> {
    pagerank_parallel(graph, config, num_threads)
}

pub fn pagerank_stats(ranks: &[f64]) {
    if ranks.is_empty() {
        return;
    }

    let sum: f64 = ranks.iter().sum();
    let min = ranks.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = ranks.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean = sum / ranks.len() as f64;

    println!("PageRank Statistics:");
    println!("  Sum: {:.6} (should be ~1.0)", sum);
    println!("  Min: {:.6e}", min);
    println!("  Max: {:.6e}", max);
    println!("  Mean: {:.6e}", mean);

    let mut indexed_ranks: Vec<(usize, f64)> = ranks
        .iter()
        .enumerate()
        .map(|(i, &r)| (i, r))
        .collect();
    indexed_ranks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    println!("\nTop 10 nodes by PageRank:");
    for (i, (node, rank)) in indexed_ranks.iter().take(10).enumerate() {
        println!("  {}. Node {}: {:.6e}", i + 1, node, rank);
    }
}

pub fn run_pagerank_and_save(
    graph: &Graph,
    config: &PageRankConfig,
    mode: &str,
    num_threads: usize,
    output_path: &str,
) -> Result<()> {
    use std::time::Instant;
    
    let start = Instant::now();
    
    let ranks = match mode {
        "seq" => pagerank_sequential(graph, config),
        "par" | "par-opt" => pagerank_parallel(graph, config, num_threads),
        _ => return Err(anyhow::anyhow!("Invalid mode: {}", mode)),
    };
    
    let elapsed = start.elapsed();
    println!("PageRank completed in {:?}", elapsed);

    write_pagerank_result(&ranks, output_path)?;
    println!("Results saved to: {}", output_path);
    
    let top_path = output_path.replace(".txt", "_top100.txt");
    write_pagerank_top_nodes(&ranks, &top_path, 100)?;
    println!("Top 100 nodes saved to: {}", top_path);
    
    let stats_path = output_path.replace(".txt", "_stats.txt");
    write_pagerank_stats(&ranks, &stats_path)?;
    println!("Statistics saved to: {}", stats_path);

    pagerank_stats(&ranks);
    
    Ok(())
}