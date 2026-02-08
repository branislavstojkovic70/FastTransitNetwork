use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Graph {
    pub num_nodes: usize,
    pub num_edges: usize,
    pub offsets: Vec<usize>,
    pub neighbors: Vec<usize>,
    pub out_degree: Vec<usize>,
}

impl Graph {
    pub fn new(num_nodes: usize) -> Self {
        Self {
            num_nodes,
            num_edges: 0,
            offsets: vec![0; num_nodes + 1],
            neighbors: Vec::new(),
            out_degree: vec![0; num_nodes],
        }
    }
    
    /// Returns the slice of out-neighbors of node `v`; empty if `v` is out of range.
    pub fn neighbors(&self, v: usize) -> &[usize] {
        if v >= self.num_nodes {
            return &self.neighbors[0..0];
        }
        &self.neighbors[self.offsets[v]..self.offsets[v + 1]]
    }
    
    pub fn is_valid_node(&self, v: usize) -> bool {
        v < self.num_nodes
    }
    
    pub fn print_info(&self) {
        println!("Graph Info:");
        println!("  Nodes: {}", self.num_nodes);
        println!("  Edges: {}", self.num_edges);
        let avg = if self.num_nodes == 0 {
            0.0
        } else {
            self.num_edges as f64 / self.num_nodes as f64
        };
        println!("  Avg degree: {:.2}", avg);
    }
}

/// Builds a CSR graph from a list of directed edges `(source, target)`.
///
/// Nodes must be in `0..num_nodes`. Duplicate edges are kept.
pub fn build_csr(num_nodes: usize, edges: Vec<(usize, usize)>) -> Graph {
    let mut graph = Graph::new(num_nodes);
    graph.num_edges = edges.len();

    for &(src, _dst) in &edges {
        graph.out_degree[src] += 1;
    }

    let mut offset = 0;
    for i in 0..num_nodes {
        graph.offsets[i] = offset;
        offset += graph.out_degree[i];
    }
    graph.offsets[num_nodes] = offset;

    graph.neighbors = vec![0; edges.len()];
    let mut current_pos = graph.offsets.clone();
    
    for (src, dst) in edges {
        graph.neighbors[current_pos[src]] = dst;
        current_pos[src] += 1;
    }
    
    graph
}

/// Loads a graph from a text file.
///
/// Format: first line is `num_nodes`; each following line is `src dst` (one edge per line).
/// Returns `Err` on I/O or parse errors.
pub fn load_graph_from_file(path: &str) -> Result<Graph> {
    let file = File::open(path).context("Failed to open file")?;
    let reader = BufReader::new(file);
    
    let mut edges = Vec::new();
    let mut max_id = 0;
    
    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        
        if line.is_empty() || line.starts_with("//") || line.starts_with("#") {
            continue;
        }
        
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            continue;
        }
        
        let src: usize = parts[0].parse()
            .context(format!("Invalid source: {}", parts[0]))?;
        let dst: usize = parts[1].parse()
            .context(format!("Invalid dest: {}", parts[1]))?;
        
        max_id = max_id.max(src).max(dst);
        edges.push((src, dst));
    }
    
    let num_nodes = max_id + 1;
    Ok(build_csr(num_nodes, edges))
}