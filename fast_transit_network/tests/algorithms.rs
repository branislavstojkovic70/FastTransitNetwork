use fast_transit_network::algorithms::bfs::{bfs_sequential, bfs_parallel};
use fast_transit_network::algorithms::wcc::{wcc_sequential, wcc_parallel, wcc_stats};
use fast_transit_network::algorithms::pagerank::{
    pagerank_sequential, pagerank_parallel, pagerank_parallel_optimized, PageRankConfig,
};
use fast_transit_network::graph::graph::{build_csr, load_graph_from_file, Graph};
use std::io::Write;

fn graph_3_node_path() -> (Graph, Vec<(usize, usize)>) {
    let edges = vec![(0, 1), (1, 2)];
    let g = build_csr(3, edges.clone());
    (g, edges)
}

fn graph_4_node_diamond() -> Graph {
    let edges = vec![(0, 1), (0, 2), (1, 3), (2, 3)];
    build_csr(4, edges)
}

fn graph_two_components() -> Graph {
    let edges = vec![(0, 1), (1, 0), (2, 3)];
    build_csr(4, edges)
}

fn graph_single_node() -> Graph {
    build_csr(1, vec![])
}

fn graph_empty() -> Graph {
    build_csr(0, vec![])
}

fn graph_two_nodes_one_edge() -> Graph {
    build_csr(2, vec![(0, 1)])
}

fn graph_two_node_cycle() -> Graph {
    build_csr(2, vec![(0, 1), (1, 0)])
}

fn graph_three_node_cycle() -> Graph {
    build_csr(3, vec![(0, 1), (1, 2), (2, 0)])
}

fn graph_5_node_path() -> Graph {
    let edges = vec![(0, 1), (1, 2), (2, 3), (3, 4)];
    build_csr(5, edges)
}

/// Star: 0 -> 1, 0 -> 2, 0 -> 3 (center 0)
fn graph_star_4() -> Graph {
    build_csr(4, vec![(0, 1), (0, 2), (0, 3)])
}

/// One isolated node (0), path 1->2->3
fn graph_isolated_plus_path() -> Graph {
    build_csr(4, vec![(1, 2), (2, 3)])
}

/// Four isolated nodes (no edges)
fn graph_four_isolated() -> Graph {
    build_csr(4, vec![])
}

/// Sink: 0 -> 1 -> 2, node 2 has no outgoing edges
fn graph_with_sink() -> Graph {
    build_csr(3, vec![(0, 1), (1, 2)])
}

/// Duplicate edges: 0->1, 0->1 (both kept in CSR)
fn graph_duplicate_edges() -> Graph {
    build_csr(2, vec![(0, 1), (0, 1)])
}

/// Large chain (100_001 nodes) for seq vs par correctness; tests are #[ignore] by default.
fn large_chain_graph() -> Graph {
    let n = 100_001_usize;
    let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
    build_csr(n, edges)
}

#[test]
fn bfs_small_path_distances() {
    let (graph, _) = graph_3_node_path();
    let dist = bfs_sequential(&graph, 0);
    assert_eq!(dist.len(), 3);
    assert_eq!(dist[0], 0);
    assert_eq!(dist[1], 1);
    assert_eq!(dist[2], 2);
}

#[test]
fn bfs_small_path_from_middle() {
    let (graph, _) = graph_3_node_path();
    let dist = bfs_sequential(&graph, 1);
    assert_eq!(dist[0], -1);
    assert_eq!(dist[1], 0);
    assert_eq!(dist[2], 1);
}

#[test]
fn bfs_diamond_distances() {
    let graph = graph_4_node_diamond();
    let dist = bfs_sequential(&graph, 0);
    assert_eq!(dist[0], 0);
    assert_eq!(dist[1], 1);
    assert_eq!(dist[2], 1);
    assert_eq!(dist[3], 2);
}

#[test]
fn bfs_single_node() {
    let graph = graph_single_node();
    let dist = bfs_sequential(&graph, 0);
    assert_eq!(dist.len(), 1);
    assert_eq!(dist[0], 0);
}

#[test]
fn bfs_invalid_source() {
    let (graph, _) = graph_3_node_path();
    let dist = bfs_sequential(&graph, 99);
    assert_eq!(dist.len(), 3);
    assert!(dist.iter().all(|&d| d == -1));
}

#[test]
fn bfs_empty_graph() {
    let graph = graph_empty();
    let dist = bfs_sequential(&graph, 0);
    assert_eq!(dist.len(), 0);
}

#[test]
fn bfs_two_nodes_one_edge() {
    let graph = graph_two_nodes_one_edge();
    let dist = bfs_sequential(&graph, 0);
    assert_eq!(dist[0], 0);
    assert_eq!(dist[1], 1);
}

#[test]
fn bfs_two_nodes_from_sink() {
    let graph = graph_two_nodes_one_edge();
    let dist = bfs_sequential(&graph, 1);
    assert_eq!(dist[0], -1);
    assert_eq!(dist[1], 0);
}

#[test]
fn bfs_two_node_cycle() {
    let graph = graph_two_node_cycle();
    let dist = bfs_sequential(&graph, 0);
    assert_eq!(dist[0], 0);
    assert_eq!(dist[1], 1);
}

#[test]
fn bfs_5_node_path() {
    let graph = graph_5_node_path();
    let dist = bfs_sequential(&graph, 0);
    for i in 0..5 {
        assert_eq!(dist[i], i as i32);
    }
}

#[test]
fn bfs_star_from_center() {
    let graph = graph_star_4();
    let dist = bfs_sequential(&graph, 0);
    assert_eq!(dist[0], 0);
    assert_eq!(dist[1], 1);
    assert_eq!(dist[2], 1);
    assert_eq!(dist[3], 1);
}

#[test]
fn bfs_star_from_leaf() {
    let graph = graph_star_4();
    let dist = bfs_sequential(&graph, 1);
    assert_eq!(dist[0], -1);
    assert_eq!(dist[1], 0);
    assert_eq!(dist[2], -1);
    assert_eq!(dist[3], -1);
}

#[test]
fn bfs_isolated_plus_path() {
    let graph = graph_isolated_plus_path();
    let dist = bfs_sequential(&graph, 1);
    assert_eq!(dist[0], -1);
    assert_eq!(dist[1], 0);
    assert_eq!(dist[2], 1);
    assert_eq!(dist[3], 2);
}

#[test]
fn bfs_deterministic_same_twice() {
    let (graph, _) = graph_3_node_path();
    let a = bfs_sequential(&graph, 0);
    let b = bfs_sequential(&graph, 0);
    assert_eq!(a, b);
}

#[test]
fn graph_neighbors_out_of_range() {
    let graph = graph_single_node();
    let n = graph.neighbors(0);
    assert!(n.is_empty());
    let n = graph.neighbors(1);
    assert!(n.is_empty());
}

#[test]
fn graph_empty_num_edges_zero() {
    let graph = graph_empty();
    assert_eq!(graph.num_nodes, 0);
    assert_eq!(graph.num_edges, 0);
}

#[test]
fn graph_duplicate_edges_kept() {
    let graph = graph_duplicate_edges();
    assert_eq!(graph.num_edges, 2);
    assert_eq!(graph.neighbors(0).len(), 2);
}

#[test]
fn graph_build_csr_no_edges() {
    let g = build_csr(3, vec![]);
    assert_eq!(g.num_nodes, 3);
    assert_eq!(g.num_edges, 0);
    assert!(g.neighbors(0).is_empty());
}

#[test]
fn graph_is_valid_node() {
    let graph = graph_3_node_path().0;
    assert!(graph.is_valid_node(0));
    assert!(graph.is_valid_node(2));
    assert!(!graph.is_valid_node(3));
}

#[test]
fn bfs_from_sink_only_self() {
    let graph = graph_with_sink();
    let dist = bfs_sequential(&graph, 2);
    assert_eq!(dist[0], -1);
    assert_eq!(dist[1], -1);
    assert_eq!(dist[2], 0);
}

#[test]
fn wcc_small_path_one_component() {
    let (graph, _) = graph_3_node_path();
    let comp = wcc_sequential(&graph);
    assert_eq!(comp.len(), 3);
    assert_eq!(comp[0], comp[1]);
    assert_eq!(comp[1], comp[2]);
    let stats = wcc_stats(&comp);
    assert_eq!(stats.num_components, 1);
    assert_eq!(stats.largest_component, 3);
}

#[test]
fn wcc_two_components() {
    let graph = graph_two_components();
    let comp = wcc_sequential(&graph);
    assert_eq!(comp.len(), 4);
    assert_eq!(comp[0], comp[1]);
    assert_eq!(comp[2], comp[3]);
    assert_ne!(comp[0], comp[2]);
    let stats = wcc_stats(&comp);
    assert_eq!(stats.num_components, 2);
}

#[test]
fn wcc_single_node() {
    let graph = graph_single_node();
    let comp = wcc_sequential(&graph);
    assert_eq!(comp.len(), 1);
    let stats = wcc_stats(&comp);
    assert_eq!(stats.num_components, 1);
}

#[test]
fn wcc_empty_graph() {
    let graph = graph_empty();
    let comp = wcc_sequential(&graph);
    assert_eq!(comp.len(), 0);
}

#[test]
fn wcc_two_node_cycle_one_component() {
    let graph = graph_two_node_cycle();
    let comp = wcc_sequential(&graph);
    assert_eq!(comp[0], comp[1]);
    assert_eq!(wcc_stats(&comp).num_components, 1);
}

#[test]
fn wcc_three_node_cycle_one_component() {
    let graph = graph_three_node_cycle();
    let comp = wcc_sequential(&graph);
    assert_eq!(comp[0], comp[1]);
    assert_eq!(comp[1], comp[2]);
    assert_eq!(wcc_stats(&comp).num_components, 1);
}

#[test]
fn wcc_diamond_one_component() {
    let graph = graph_4_node_diamond();
    let comp = wcc_sequential(&graph);
    let stats = wcc_stats(&comp);
    assert_eq!(stats.num_components, 1);
    assert_eq!(stats.largest_component, 4);
}

#[test]
fn wcc_four_isolated_four_components() {
    let graph = graph_four_isolated();
    let comp = wcc_sequential(&graph);
    let stats = wcc_stats(&comp);
    assert_eq!(stats.num_components, 4);
    assert_eq!(stats.largest_component, 1);
}

#[test]
fn wcc_star_one_component() {
    let graph = graph_star_4();
    let comp = wcc_sequential(&graph);
    assert_eq!(wcc_stats(&comp).num_components, 1);
}

#[test]
fn wcc_5_node_path_one_component() {
    let graph = graph_5_node_path();
    let comp = wcc_sequential(&graph);
    assert_eq!(wcc_stats(&comp).num_components, 1);
}

#[test]
fn wcc_isolated_plus_path_two_components() {
    let graph = graph_isolated_plus_path();
    let comp = wcc_sequential(&graph);
    assert_eq!(wcc_stats(&comp).num_components, 2);
}

#[test]
fn wcc_component_sizes_sum_to_nodes() {
    let graph = graph_two_components();
    let comp = wcc_sequential(&graph);
    let stats = wcc_stats(&comp);
    let sum: usize = stats.component_sizes.values().sum();
    assert_eq!(sum, 4);
}

#[test]
fn wcc_stats_smallest_largest() {
    let graph = graph_two_components();
    let comp = wcc_sequential(&graph);
    let stats = wcc_stats(&comp);
    assert_eq!(stats.smallest_component, 2);
    assert_eq!(stats.largest_component, 2);
}

#[test]
fn pagerank_small_path_sum_one() {
    let (graph, _) = graph_3_node_path();
    let config = PageRankConfig {
        alpha: 0.85,
        max_iterations: 100,
        tolerance: 1e-8,
    };
    let ranks = pagerank_sequential(&graph, &config);
    assert_eq!(ranks.len(), 3);
    let sum: f64 = ranks.iter().sum();
    assert!((sum - 1.0).abs() < 1e-6, "PageRank sum should be ~1, got {}", sum);
}

#[test]
fn pagerank_single_node() {
    let graph = graph_single_node();
    let config = PageRankConfig {
        alpha: 0.85,
        max_iterations: 10,
        tolerance: 1e-10,
    };
    let ranks = pagerank_sequential(&graph, &config);
    assert_eq!(ranks.len(), 1);
    assert!((ranks[0] - 1.0).abs() < 1e-10);
}

#[test]
fn pagerank_diamond_nonzero_all_nodes() {
    let graph = graph_4_node_diamond();
    let config = PageRankConfig {
        alpha: 0.85,
        max_iterations: 100,
        tolerance: 1e-8,
    };
    let ranks = pagerank_sequential(&graph, &config);
    assert_eq!(ranks.len(), 4);
    let sum: f64 = ranks.iter().sum();
    assert!((sum - 1.0).abs() < 1e-5);
    for (i, &r) in ranks.iter().enumerate() {
        assert!(r > 0.0, "Node {} should have positive rank", i);
    }
}

#[test]
fn pagerank_empty_graph() {
    let graph = graph_empty();
    let config = PageRankConfig::default();
    let ranks = pagerank_sequential(&graph, &config);
    assert!(ranks.is_empty());
}

#[test]
fn pagerank_two_nodes_one_edge() {
    let graph = graph_two_nodes_one_edge();
    let config = PageRankConfig {
        alpha: 0.85,
        max_iterations: 100,
        tolerance: 1e-8,
    };
    let ranks = pagerank_sequential(&graph, &config);
    assert_eq!(ranks.len(), 2);
    let sum: f64 = ranks.iter().sum();
    assert!((sum - 1.0).abs() < 1e-5);
}

#[test]
fn pagerank_two_node_cycle() {
    let graph = graph_two_node_cycle();
    let config = PageRankConfig {
        alpha: 0.85,
        max_iterations: 100,
        tolerance: 1e-8,
    };
    let ranks = pagerank_sequential(&graph, &config);
    assert_eq!(ranks.len(), 2);
    assert!((ranks[0] - 0.5).abs() < 1e-5);
    assert!((ranks[1] - 0.5).abs() < 1e-5);
}

#[test]
fn pagerank_alpha_half() {
    let (graph, _) = graph_3_node_path();
    let config = PageRankConfig {
        alpha: 0.5,
        max_iterations: 100,
        tolerance: 1e-8,
    };
    let ranks = pagerank_sequential(&graph, &config);
    let sum: f64 = ranks.iter().sum();
    assert!((sum - 1.0).abs() < 1e-5);
}

#[test]
fn pagerank_max_iterations_respected() {
    let (graph, _) = graph_3_node_path();
    let config = PageRankConfig {
        alpha: 0.85,
        max_iterations: 1,
        tolerance: 1e-15,
    };
    let ranks = pagerank_sequential(&graph, &config);
    assert_eq!(ranks.len(), 3);
    let sum: f64 = ranks.iter().sum();
    assert!((sum - 1.0).abs() < 1e-5);
}

#[test]
fn pagerank_high_tolerance_converges_quickly() {
    let (graph, _) = graph_3_node_path();
    let config = PageRankConfig {
        alpha: 0.85,
        max_iterations: 100,
        tolerance: 0.1,
    };
    let ranks = pagerank_sequential(&graph, &config);
    assert_eq!(ranks.len(), 3);
}

#[test]
fn pagerank_with_sink() {
    let graph = graph_with_sink();
    let config = PageRankConfig {
        alpha: 0.85,
        max_iterations: 100,
        tolerance: 1e-8,
    };
    let ranks = pagerank_sequential(&graph, &config);
    assert_eq!(ranks.len(), 3);
    let sum: f64 = ranks.iter().sum();
    assert!((sum - 1.0).abs() < 1e-5);
    for &r in &ranks {
        assert!(r > 0.0);
    }
}

#[test]
fn pagerank_5_node_path_sum_one() {
    let graph = graph_5_node_path();
    let config = PageRankConfig::default();
    let ranks = pagerank_sequential(&graph, &config);
    assert_eq!(ranks.len(), 5);
    let sum: f64 = ranks.iter().sum();
    assert!((sum - 1.0).abs() < 1e-5);
}

#[test]
fn pagerank_deterministic_same_twice() {
    let (graph, _) = graph_3_node_path();
    let config = PageRankConfig::default();
    let a = pagerank_sequential(&graph, &config);
    let b = pagerank_sequential(&graph, &config);
    assert_eq!(a.len(), b.len());
    for (x, y) in a.iter().zip(b.iter()) {
        assert!((x - y).abs() < 1e-12);
    }
}

#[test]
fn pagerank_default_config() {
    let graph = graph_single_node();
    let ranks = pagerank_sequential(&graph, &PageRankConfig::default());
    assert_eq!(ranks.len(), 1);
    assert!((ranks[0] - 1.0).abs() < 1e-10);
}

#[test]
fn pagerank_path_all_positive() {
    let graph = graph_5_node_path();
    let ranks = pagerank_sequential(&graph, &PageRankConfig::default());
    for (i, &r) in ranks.iter().enumerate() {
        assert!(r > 0.0, "node {} rank {}", i, r);
    }
}


#[test]
fn load_graph_from_file_valid() {
    let dir = std::env::temp_dir();
    let path = dir.join("ftn_test_graph.txt");
    let mut f = std::fs::File::create(&path).unwrap();
    writeln!(f, "0 1").unwrap();
    writeln!(f, "1 2").unwrap();
    writeln!(f, "2 0").unwrap();
    f.sync_all().unwrap();
    drop(f);
    let graph = load_graph_from_file(path.to_str().unwrap()).unwrap();
    assert_eq!(graph.num_nodes, 3);
    assert_eq!(graph.num_edges, 3);
    let _ = std::fs::remove_file(&path);
}

#[test]
fn graph_star_out_degree() {
    let graph = graph_star_4();
    assert_eq!(graph.neighbors(0).len(), 3);
    assert!(graph.neighbors(1).is_empty());
}

#[test]
fn wcc_duplicate_edges_still_one_component() {
    let graph = graph_duplicate_edges();
    let comp = wcc_sequential(&graph);
    assert_eq!(comp[0], comp[1]);
}

#[test]
fn load_graph_from_file_with_comments() {
    let dir = std::env::temp_dir();
    let path = dir.join("ftn_test_graph_comments.txt");
    let mut f = std::fs::File::create(&path).unwrap();
    writeln!(f, "// comment").unwrap();
    writeln!(f, "0 1").unwrap();
    writeln!(f, "# another").unwrap();
    writeln!(f, "1 0").unwrap();
    f.sync_all().unwrap();
    drop(f);
    let graph = load_graph_from_file(path.to_str().unwrap()).unwrap();
    assert_eq!(graph.num_nodes, 2);
    assert_eq!(graph.num_edges, 2);
    let _ = std::fs::remove_file(&path);
}

#[test]
#[ignore = "large graph ~100k nodes; use --include-ignored for full run"]
fn bfs_seq_par_same_distances() {
    let graph = large_chain_graph();
    let dist_seq = bfs_sequential(&graph, 0);
    let dist_par = bfs_parallel(&graph, 0, 4);
    assert_eq!(dist_seq.len(), dist_par.len());
    assert_eq!(dist_seq, dist_par, "BFS sequential and parallel must produce the same distances");
}

#[test]
#[ignore = "large graph ~100k nodes; use --include-ignored for full run"]
fn wcc_seq_par_same_partition() {
    let graph = large_chain_graph();
    let comp_seq = wcc_sequential(&graph);
    let comp_par = wcc_parallel(&graph, 4);
    assert_eq!(comp_seq.len(), comp_par.len());

    let stats_seq = wcc_stats(&comp_seq);
    let stats_par = wcc_stats(&comp_par);
    assert_eq!(stats_seq.num_components, stats_par.num_components);

    let mut sizes_seq: Vec<usize> = stats_seq.component_sizes.values().copied().collect();
    let mut sizes_par: Vec<usize> = stats_par.component_sizes.values().copied().collect();
    sizes_seq.sort();
    sizes_par.sort();
    assert_eq!(sizes_seq, sizes_par, "WCC: same component size distribution");
}

#[test]
#[ignore = "large graph ~100k nodes; use --include-ignored for full run"]
fn pagerank_seq_par_agree() {
    let graph = large_chain_graph();
    let config = PageRankConfig {
        alpha: 0.85,
        max_iterations: 50,
        tolerance: 1e-6,
    };
    let ranks_seq = pagerank_sequential(&graph, &config);
    let ranks_par = pagerank_parallel(&graph, &config, 4);
    assert_eq!(ranks_seq.len(), ranks_par.len());
    let max_diff: f64 = ranks_seq
        .iter()
        .zip(ranks_par.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0_f64, f64::max);
    assert!(max_diff < 1e-4, "PageRank seq vs par: max diff {} should be < 1e-4", max_diff);
}

#[test]
#[ignore = "large graph ~100k nodes; use --include-ignored for full run"]
fn pagerank_seq_par_opt_agree() {
    let graph = large_chain_graph();
    let config = PageRankConfig {
        alpha: 0.85,
        max_iterations: 50,
        tolerance: 1e-6,
    };
    let ranks_seq = pagerank_sequential(&graph, &config);
    let ranks_par_opt = pagerank_parallel_optimized(&graph, &config, 4);
    assert_eq!(ranks_seq.len(), ranks_par_opt.len());
    let max_diff: f64 = ranks_seq
        .iter()
        .zip(ranks_par_opt.iter())
        .map(|(a, b)| (a - b).abs())
        .fold(0.0_f64, f64::max);
    assert!(max_diff < 1e-4, "PageRank seq vs par-opt: max diff {} should be < 1e-4", max_diff);
}
