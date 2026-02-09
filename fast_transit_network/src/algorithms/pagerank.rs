use crate::graph::graph::Graph;

/// PageRank parameters: damping factor, iteration limit, and convergence tolerance.
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

/// Sequential PageRank: returns a probability vector over nodes (sum ≈ 1). Stops when L1 change is below tolerance or max iterations reached.
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

/// Prints PageRank statistics (sum, min, max, mean) and top 10 nodes by rank.
pub fn pagerank_stats(ranks: &[f64]) {
    if ranks.is_empty() {
        return;
    }

    let sum: f64 = ranks.iter().sum();
    let min = ranks
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let max = ranks
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
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
