use std::fs::File;
use std::io::{BufWriter, Write};
use anyhow::Result;

/// Writes BFS results (node, distance) to a file, one pair per line.
pub fn write_bfs_result(dist: &[i32], output_path: &str) -> Result<()> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    
    for (node, &distance) in dist.iter().enumerate() {
        writeln!(writer, "{} {}", node, distance)?;
    }
    
    Ok(())
}

/// Writes WCC results (node, component_id) to a file, one pair per line.
pub fn write_wcc_result(components: &[usize], output_path: &str) -> Result<()> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    
    for (node, &comp) in components.iter().enumerate() {
        writeln!(writer, "{} {}", node, comp)?;
    }
    
    Ok(())
}

/// Writes WCC statistics (component counts and sizes) to a file.
pub fn write_wcc_stats(
    components: &[usize], 
    stats_path: &str
) -> Result<()> {
    use std::collections::HashMap;
    
    let file = File::create(stats_path)?;
    let mut writer = BufWriter::new(file);
    
    let mut comp_sizes: HashMap<usize, usize> = HashMap::new();
    for &comp in components {
        *comp_sizes.entry(comp).or_insert(0) += 1;
    }
    
    writeln!(writer, "# WCC Statistics")?;
    writeln!(writer, "total_components: {}", comp_sizes.len())?;
    writeln!(writer, "largest_component: {}", 
             comp_sizes.values().max().unwrap_or(&0))?;
    writeln!(writer, "smallest_component: {}", 
             comp_sizes.values().min().unwrap_or(&0))?;
    writeln!(writer)?;
    writeln!(writer, "# Component ID, Size")?;
    
    let mut sizes: Vec<_> = comp_sizes.iter().collect();
    sizes.sort_by_key(|&(_, size)| std::cmp::Reverse(size));
    
    for (comp_id, size) in sizes {
        writeln!(writer, "{} {}", comp_id, size)?;
    }
    
    Ok(())
}

/// Writes PageRank results (node, rank) to a file, one pair per line.
pub fn write_pagerank_result(ranks: &[f64], output_path: &str) -> Result<()> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "# Node PageRank")?;
    
    for (node, &rank) in ranks.iter().enumerate() {
        writeln!(writer, "{} {:.10e}", node, rank)?;
    }
    
    Ok(())
}

/// Writes top N nodes by PageRank to a file (rank position, node id, score).
pub fn write_pagerank_top_nodes(
    ranks: &[f64],
    output_path: &str,
    top_n: usize,
) -> Result<()> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);

    let mut indexed_ranks: Vec<(usize, f64)> = ranks
        .iter()
        .enumerate()
        .map(|(i, &r)| (i, r))
        .collect();
    indexed_ranks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    writeln!(writer, "# Rank Node PageRank")?;
    
    for (rank_position, (node, rank)) in indexed_ranks.iter().take(top_n).enumerate() {
        writeln!(writer, "{} {} {:.10e}", rank_position + 1, node, rank)?;
    }
    
    Ok(())
}

/// Writes PageRank statistics (sum, min, max, mean, node count) to a file.
pub fn write_pagerank_stats(ranks: &[f64], stats_path: &str) -> Result<()> {
    let file = File::create(stats_path)?;
    let mut writer = BufWriter::new(file);
    
    let sum: f64 = ranks.iter().sum();
    let min = ranks.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = ranks.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let mean = sum / ranks.len() as f64;
    
    writeln!(writer, "# PageRank Statistics")?;
    writeln!(writer, "sum: {:.10}", sum)?;
    writeln!(writer, "min: {:.10e}", min)?;
    writeln!(writer, "max: {:.10e}", max)?;
    writeln!(writer, "mean: {:.10e}", mean)?;
    writeln!(writer, "nodes: {}", ranks.len())?;
    
    Ok(())
}