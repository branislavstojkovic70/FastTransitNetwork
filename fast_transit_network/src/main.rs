mod graph;
mod algorithms;
use graph::*;
use algorithms::*;

fn main() {    
    match load_graph_from_file("test_graph.txt") {
        Ok(graph) => {
            graph.print_info();
            
            let source = 0;
            let dist = bfs_sequential(&graph, source);
            print_bfs_result(&dist, source);
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}