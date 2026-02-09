use std::fs::File;
use std::io::{BufWriter, Write};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub algorithm: String,
    pub graph_name: String,
    pub graph_nodes: usize,
    pub graph_edges: usize,
    pub mode: String,
    pub threads: usize,
    pub time_ms: f64,
    pub speedup: f64,
    pub correct: bool,
}

pub struct BenchmarkLogger {
    results: Vec<BenchmarkResult>,
}

impl BenchmarkLogger {
    pub fn new() -> Self {
        Self {
            results: Vec::new(),
        }
    }
    
    pub fn add_result(&mut self, result: BenchmarkResult) {
        self.results.push(result);
    }
    
    pub fn save_to_csv(&self, output_path: &str) -> Result<()> {
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);
        
        // Header
        writeln!(writer, "algorithm,graph_name,nodes,edges,mode,threads,time_ms,speedup,correct")?;
        
        // Data
        for result in &self.results {
            writeln!(
                writer,
                "{},{},{},{},{},{},{:.6},{:.4},{}",
                result.algorithm,
                result.graph_name,
                result.graph_nodes,
                result.graph_edges,
                result.mode,
                result.threads,
                result.time_ms,
                result.speedup,
                result.correct
            )?;
        }
        
        Ok(())
    }
    
    pub fn print_summary(&self) {
        println!("\n{}", "=".repeat(70));
        println!("BENCHMARK SUMMARY");
        println!("{}", "=".repeat(70));
        
        for algo in ["BFS", "WCC", "PageRank"] {
            let algo_results: Vec<_> = self.results.iter()
                .filter(|r| r.algorithm == algo)
                .collect();
            
            if algo_results.is_empty() {
                continue;
            }
            
            println!("\n{} Results:", algo);
            
            for graph in algo_results.iter().map(|r| &r.graph_name).collect::<std::collections::HashSet<_>>() {
                let graph_results: Vec<_> = algo_results.iter()
                    .filter(|r| &r.graph_name == graph)
                    .collect();
                
                if let Some(seq) = graph_results.iter().find(|r| r.mode == "seq") {
                    println!("  {} ({} nodes):", graph, seq.graph_nodes);
                    println!("    Sequential: {:.2}ms", seq.time_ms);
                    
                    let max_speedup = graph_results.iter()
                        .filter(|r| r.mode == "par")
                        .map(|r| r.speedup)
                        .fold(0.0f64, f64::max);
                    
                    println!("    Max Speedup: {:.2}x", max_speedup);
                }
            }
        }
    }
}