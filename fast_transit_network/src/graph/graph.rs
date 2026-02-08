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
    
    pub fn neighbors(&self, v: usize) -> &[usize] {
        &self.neighbors[self.offsets[v]..self.offsets[v + 1]]
    }
    
    pub fn is_valid_node(&self, v: usize) -> bool {
        v < self.num_nodes
    }
    
    pub fn print_info(&self) {
        println!("Graph Info:");
        println!("  Nodes: {}", self.num_nodes);
        println!("  Edges: {}", self.num_edges);
        println!("  Avg degree: {:.2}", self.num_edges as f64 / self.num_nodes as f64);
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