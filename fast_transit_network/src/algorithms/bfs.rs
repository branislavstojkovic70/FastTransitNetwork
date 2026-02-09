use crate::graph::graph::Graph;
use std::collections::VecDeque;
use rayon::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering};

/// Below this many nodes, use sequential BFS to avoid thread-pool and atomic overhead.
const PAR_MIN_NODES: usize = 50_000;
/// Minimum frontier size to use parallel iteration; below this we process the level sequentially.
const PAR_MIN_FRONTIER: usize = 1024;

/// Sequential BFS: returns distance from source for each node (-1 if unreachable).
pub fn bfs_sequential(graph: &Graph, source: usize) -> Vec<i32> {
    let mut dist = vec![-1; graph.num_nodes];
    
    if !graph.is_valid_node(source) {
        eprintln!("Invalid source node: {}", source);
        return dist;
    }
    
    let mut queue = VecDeque::new();
    dist[source] = 0;
    queue.push_back(source);
    
    while let Some(u) = queue.pop_front() {
        for &v in graph.neighbors(u) {
            if dist[v] == -1 {
                dist[v] = dist[u] + 1;
                queue.push_back(v);
            }
        }
    }
    
    dist
}

/// Parallel level-synchronous BFS. Falls back to sequential for small graphs; uses threads only when the current frontier is large.
pub fn bfs_parallel(graph: &Graph, source: usize, num_threads: usize) -> Vec<i32> {
    if graph.num_nodes < PAR_MIN_NODES {
        return bfs_sequential(graph, source);
    }
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build()
        .expect("rayon thread pool")
        .install(|| bfs_parallel_impl(graph, source))
}

fn bfs_parallel_impl(graph: &Graph, source: usize) -> Vec<i32> {
    if !graph.is_valid_node(source) {
        eprintln!("Invalid source node: {}", source);
        return vec![-1; graph.num_nodes];
    }

    let dist: Vec<AtomicI32> = (0..graph.num_nodes)
        .map(|_| AtomicI32::new(-1))
        .collect();

    dist[source].store(0, Ordering::Relaxed);

    let mut current_frontier = vec![source];
    let mut next_frontier = Vec::new();
    let mut level = 0;

    while !current_frontier.is_empty() {
        let use_parallel = current_frontier.len() >= PAR_MIN_FRONTIER;

        if use_parallel {
            let local_next: Vec<Vec<usize>> = current_frontier
                .par_iter()
                .map(|&u| {
                    let mut local_neighbors = Vec::new();
                    for &v in graph.neighbors(u) {
                        if dist[v]
                            .compare_exchange(-1, level + 1, Ordering::Relaxed, Ordering::Relaxed)
                            .is_ok()
                        {
                            local_neighbors.push(v);
                        }
                    }
                    local_neighbors
                })
                .collect();
            next_frontier.clear();
            for local in local_next {
                next_frontier.extend(local);
            }
        } else {
            next_frontier.clear();
            for &u in &current_frontier {
                for &v in graph.neighbors(u) {
                    if dist[v]
                        .compare_exchange(-1, level + 1, Ordering::Relaxed, Ordering::Relaxed)
                        .is_ok()
                    {
                        next_frontier.push(v);
                    }
                }
            }
        }

        next_frontier.sort_unstable();
        next_frontier.dedup();
        std::mem::swap(&mut current_frontier, &mut next_frontier);
        level += 1;
    }

    dist.into_iter().map(|d| d.into_inner()).collect()
}

/// Prints BFS result: levels and reachable node count.
pub fn print_bfs_result(dist: &[i32], source: usize) {
    println!("\nBFS from node {}:", source);

    let mut by_level: Vec<Vec<usize>> = Vec::new();
    for (v, &d) in dist.iter().enumerate() {
        if d >= 0 {
            let level = d as usize;
            while by_level.len() <= level {
                by_level.push(Vec::new());
            }
            by_level[level].push(v);
        }
    }

    for (level, nodes) in by_level.iter().enumerate() {
        if level < 5 || level == by_level.len() - 1 {
            println!("  Level {}: {} nodes", level, nodes.len());
        } else if level == 5 {
            println!("  ...");
        }
    }

    let reachable = dist.iter().filter(|&&d| d >= 0).count();
    println!("Reachable: {}/{}", reachable, dist.len());
}