use crate::graph::graph::Graph;
use super::union_find::UnionFind;

/// Sequential WCC: finds weakly connected components (treats graph as undirected).
pub fn wcc_sequential(graph: &Graph) -> Vec<usize> {
    let mut uf = UnionFind::new(graph.num_nodes);

    for u in 0..graph.num_nodes {
        for &v in graph.neighbors(u) {
            uf.union(u, v);
        }
    }

    uf.get_components()
}

/// Computes statistics for WCC result (component counts and sizes).
pub fn wcc_stats(components: &[usize]) -> WccStats {
    use std::collections::HashMap;
    
    let mut comp_sizes: HashMap<usize, usize> = HashMap::new();
    
    for &comp in components {
        *comp_sizes.entry(comp).or_insert(0) += 1;
    }
    
    let num_components = comp_sizes.len();
    let largest_component = *comp_sizes.values().max().unwrap_or(&0);
    let smallest_component = *comp_sizes.values().min().unwrap_or(&0);
    
    WccStats {
        num_components,
        largest_component,
        smallest_component,
        component_sizes: comp_sizes,
    }
}

pub struct WccStats {
    pub num_components: usize,
    pub largest_component: usize,
    pub smallest_component: usize,
    pub component_sizes: std::collections::HashMap<usize, usize>,
}

impl WccStats {
    pub fn print(&self) {
        println!("WCC Statistics:");
        println!("  Total components: {}", self.num_components);
        println!("  Largest component: {} nodes", self.largest_component);
        println!("  Smallest component: {} nodes", self.smallest_component);
        
        if self.num_components <= 10 {
            println!("\nComponent sizes:");
            let mut sizes: Vec<_> = self.component_sizes.iter().collect();
            sizes.sort_by_key(|&(_, size)| std::cmp::Reverse(size));
            
            for (comp_id, size) in sizes {
                println!("  Component {}: {} nodes", comp_id, size);
            }
        } else {
            println!("\nTop 5 largest components:");
            let mut sizes: Vec<_> = self.component_sizes.iter().collect();
            sizes.sort_by_key(|&(_, size)| std::cmp::Reverse(size));
            
            for (comp_id, size) in sizes.iter().take(5) {
                println!("  Component {}: {} nodes", comp_id, size);
            }
        }
    }
}