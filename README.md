# FastTransitNetwork

A high-performance **graph analytics** toolkit providing parallel implementations of BFS, Weakly Connected Components (WCC), and PageRank. Built in Rust with optional Python tooling for benchmark visualization.

**Project for Master studies — FTN High Performance Computing.**

---

## Features

- **BFS** (Breadth-First Search) — sequential and parallel (multi-threaded)
- **WCC** (Weakly Connected Components) — sequential and parallel
- **PageRank** — sequential, parallel, and parallel-optimized (`par-opt`)
- **Benchmark** — run all algorithms across multiple thread counts and export CSV
- **Visualization** — Python scripts to generate speedup plots and summary tables from benchmark results
- **Graph generation** — Python scripts for random, scale-free, grid, and chain graphs

---

## Requirements

- **Rust** (latest stable; e.g. `rustup` toolchain)
- **Python 3.8+** (only for graph generation and benchmark visualization)
- Optional: **Python venv** and `pip` for installing `pandas`, `matplotlib`, `tabulate` (recommended on Linux to avoid system Python restrictions)

---

## Quick Start

All commands below are run from the **`fast_transit_network/`** directory (project root for the Rust crate).

### 1. Build

```bash
cd fast_transit_network
cargo build --release
```

### 2. Generate graph data (optional)

```bash
cd scripts && python3 generate_graphs.py
```

This creates `scripts/data/small/`, `scripts/data/medium/`, `scripts/data/large/`, and `scripts/data/heavy/` with edge-list files (e.g. `random_1k.txt`, `random_100k.txt`).

### 3. Run algorithms

```bash
# Help
./target/release/tool --help

# BFS (sequential and parallel)
./target/release/tool bfs --input scripts/data/small/random_1k.txt --source 0 --mode seq --out bfs_out.txt
./target/release/tool bfs --input scripts/data/small/random_1k.txt --source 0 --mode par --threads 8 --out bfs_par.txt

# WCC
./target/release/tool wcc --input scripts/data/small/random_1k.txt --mode par --threads 8 --out wcc_out.txt

# PageRank (seq, par, or par-opt)
./target/release/tool pagerank --input scripts/data/small/random_1k.txt --mode par --threads 8 --out pr_out.txt
```

### 4. Benchmark and visualize

```bash
# Run benchmark (writes CSV to scripts/results/benchmark_results.csv by default)
./target/release/tool benchmark --input scripts/data/small/random_1k.txt --threads 2,4,8,16

# Python visualization (requires pandas, matplotlib, tabulate)
python3 -m venv .venv
source .venv/bin/activate   # Windows: .venv\Scripts\activate
pip install -r requirements.txt
python scripts/visualize_benchmark.py scripts/results/benchmark_results.csv
```

Plots and tables are written to **`scripts/results/plots/`**.

---

## Input format

Graphs are **directed edge lists**, one edge per line:

```
source_node target_node
```

- Node IDs are non-negative integers. The loader infers the number of nodes from the maximum node index in the file.
- Lines starting with `//` are treated as comments and skipped.
- Example: `0 1` and `1 2` define two edges.

---

## Output and results

| Command   | Output files |
|----------|---------------|
| **BFS**  | `--out`: one line per node `node_id distance` (-1 if unreachable). |
| **WCC**  | `--out`: `node_id component_id`; `*_stats.txt`: component sizes and counts. |
| **PageRank** | `--out`: `node_id rank`; `*_top100.txt`: top 100 nodes; `*_stats.txt`: sum, min, max, mean. |
| **Benchmark** | CSV at `scripts/results/benchmark_results.csv` (or path given by `--out`). |

---

## Project structure

```
FastTransitNetwork/
├── README.md
├── fast_transit_network/
│   ├── Cargo.toml
│   ├── commands.txt          # Step-by-step command reference
│   ├── requirements.txt      # Python deps for visualization
│   ├── src/
│   │   ├── main.rs           # PageRank-only benchmark binary
│   │   ├── tool.rs           # CLI entry (bfs, wcc, pagerank, benchmark)
│   │   ├── cli.rs
│   │   ├── graph/            # Graph type and loader (CSR)
│   │   ├── algorithms/       # BFS, WCC, PageRank, union-find
│   │   └── utils/            # I/O, benchmark logging
│   └── scripts/
│       ├── generate_graphs.py
│       ├── visualize_benchmark.py
│       ├── data/             # Generated graphs (optional)
│       └── results/          # Benchmark CSV and plots
└── .gitignore
```


## License and attribution

Academic project for **FTN High Performance Computing** (Master studies)
