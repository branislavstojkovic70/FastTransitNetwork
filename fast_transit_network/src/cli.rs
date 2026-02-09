use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "fast_transit_network")]
#[command(about = "Graph analytics tool for FastTransitNetwork", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Run BFS (Breadth-First Search)
    Bfs {
        /// Input graph file (edge list format)
        #[arg(short, long)]
        input: String,
        
        /// Source node for BFS
        #[arg(short, long)]
        source: usize,
        
        /// Mode: seq or par
        #[arg(short, long, default_value = "seq")]
        mode: String,
        
        /// Number of threads (for parallel mode)
        #[arg(short, long, default_value_t = 4)]
        threads: usize,
        
        /// Output file path
        #[arg(short, long)]
        out: String,
    },
    
    /// Run WCC (Weakly Connected Components)
    Wcc {
        /// Input graph file (edge list format)
        #[arg(short, long)]
        input: String,
        
        /// Mode: seq or par
        #[arg(short, long, default_value = "seq")]
        mode: String,
        
        /// Number of threads (for parallel mode)
        #[arg(short, long, default_value_t = 4)]
        threads: usize,
        
        /// Output file path
        #[arg(short, long)]
        out: String,
    },
    
    /// Run PageRank
    Pagerank {
        /// Input graph file (edge list format)
        #[arg(short, long)]
        input: String,
        
        /// Mode: seq, par, or par-opt
        #[arg(short, long, default_value = "seq")]
        mode: String,
        
        /// Number of threads (for parallel mode)
        #[arg(short, long, default_value_t = 4)]
        threads: usize,
        
        /// Output file path
        #[arg(short, long)]
        out: String,
        
        /// Damping factor (alpha)
        #[arg(long, default_value_t = 0.85)]
        alpha: f64,
        
        /// Maximum iterations
        #[arg(long, default_value_t = 100)]
        iters: usize,
        
        /// Convergence tolerance
        #[arg(long, default_value_t = 1e-6)]
        eps: f64,
    },
    
    /// Run benchmark on all algorithms
    Benchmark {
        /// Input graph file
        #[arg(short, long)]
        input: String,
        
        /// Thread counts to test (comma-separated)
        #[arg(short, long, default_value = "2,4,8,16")]
        threads: String,
        
        /// Output CSV path for benchmark results (default: scripts/results/benchmark_results.csv)
        #[arg(short, long, default_value = "scripts/results/benchmark_results.csv")]
        out: String,
    },
}