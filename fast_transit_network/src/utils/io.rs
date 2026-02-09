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