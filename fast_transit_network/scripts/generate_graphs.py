import random
import os


def ensure_dir(path):
    """Creates the directory (and parents) if it does not exist. No-op if path is empty."""
    if path:
        os.makedirs(path, exist_ok=True)


def generate_random_graph(num_nodes, num_edges, filename):
    """Generates a random directed graph with the given number of nodes and edges."""
    print(f"Random graph: {num_nodes:,} nodes, {num_edges:,} edges...", end=" ", flush=True)

    ensure_dir(os.path.dirname(filename))

    with open(filename, 'w') as f:
        f.write(f"// Random graph: {num_nodes} nodes, {num_edges} edges\n")

        edges_written = 0
        attempts = 0
        max_attempts = num_edges * 3
        seen = set()

        while edges_written < num_edges and attempts < max_attempts:
            src = random.randint(0, num_nodes - 1)
            dst = random.randint(0, num_nodes - 1)

            if src != dst and (src, dst) not in seen:
                f.write(f"{src} {dst}\n")
                seen.add((src, dst))
                edges_written += 1

            attempts += 1

    if edges_written < num_edges:
        print(f" (wrote only {edges_written:,} edges, graph may be too dense)", end=" ")
    print(f"OK {filename}")


def generate_random_graph_streaming(num_nodes, num_edges, filename):
    """Generates a random directed graph by streaming edges (no dedup; use for huge graphs)."""
    print(f"Random graph (streaming): {num_nodes:,} nodes, {num_edges:,} edges...", end=" ", flush=True)
    ensure_dir(os.path.dirname(filename))
    with open(filename, 'w') as f:
        f.write(f"// Random graph (streaming): {num_nodes} nodes, {num_edges} edges\n")
        for _ in range(num_edges):
            src = random.randint(0, num_nodes - 1)
            dst = random.randint(0, num_nodes - 1)
            if src != dst:
                f.write(f"{src} {dst}\n")
    print(f"OK {filename}")


def generate_scale_free_fast(num_nodes, avg_degree, filename):
    """Generates an approximate scale-free graph with hub nodes and random links."""
    print(f"Scale-free graph: {num_nodes:,} nodes, avg_deg={avg_degree}...", end=" ", flush=True)

    ensure_dir(os.path.dirname(filename))

    with open(filename, 'w') as f:
        f.write(f"// Approximate scale-free graph: {num_nodes} nodes\n")

        # Hub nodes (5% of nodes, at least 1, at most num_nodes)
        num_hubs = min(max(1, num_nodes // 20), num_nodes)

        # Each node connects to several random nodes and to one hub
        for node in range(num_nodes):
            # Connect to a random hub
            hub = random.randint(0, num_hubs - 1)
            if node != hub:
                f.write(f"{node} {hub}\n")

            # Connect to several random nodes
            for _ in range(avg_degree):
                target = random.randint(0, num_nodes - 1)
                if target != node:
                    f.write(f"{node} {target}\n")

    print(f"OK {filename}")


def generate_grid_graph(rows, cols, filename):
    """Generates a 2D grid graph (each cell connected to right and bottom neighbor)."""
    print(f"Grid graph: {rows}x{cols} = {rows*cols:,} nodes...", end=" ", flush=True)

    ensure_dir(os.path.dirname(filename))

    with open(filename, 'w') as f:
        f.write(f"// Grid graph: {rows}x{cols}\n")

        for i in range(rows):
            for j in range(cols):
                node = i * cols + j

                if j < cols - 1:
                    f.write(f"{node} {node + 1}\n")

                if i < rows - 1:
                    f.write(f"{node} {node + cols}\n")

    print(f"OK {filename}")


def generate_chain_graph(num_nodes, filename):
    """Generates a chain graph (0->1->2->...), worst case for parallelization."""
    print(f"Chain graph: {num_nodes:,} nodes...", end=" ", flush=True)

    ensure_dir(os.path.dirname(filename))

    with open(filename, 'w') as f:
        f.write(f"// Chain graph: {num_nodes} nodes\n")

        for i in range(num_nodes - 1):
            f.write(f"{i} {i + 1}\n")

    print(f"OK {filename}")


def main():
    """Runs the graph generator for small, medium, and large datasets."""
    print("=" * 70)
    print("Test Graph Generator - Optimized version")
    print("=" * 70)
    print()

    # Small graphs
    print("Small graphs:")
    generate_random_graph(1_000, 5_000, "data/small/random_1k.txt")
    generate_random_graph(10_000, 50_000, "data/small/random_10k.txt")
    generate_chain_graph(10_000, "data/small/chain_10k.txt")

    # Medium graphs
    print("\nMedium graphs:")
    generate_random_graph(100_000, 500_000, "data/medium/random_100k.txt")
    generate_scale_free_fast(100_000, 5, "data/medium/scale_free_100k.txt")
    generate_grid_graph(316, 316, "data/medium/grid_100k.txt")
    generate_chain_graph(100_000, "data/medium/chain_100k.txt")

    # Large graphs (optional, may take 30-60s)
    print("\nLarge graphs (may take 30-60s):")
    generate_random_graph(1_000_000, 5_000_000, "data/large/random_1m.txt")
    generate_scale_free_fast(1_000_000, 5, "data/large/scale_free_1m.txt")

    # Heavy: only these 4 (100M nodes; may take 10-60+ min and several GB)
    print("\nHeavy graphs (4 graphs, 100M nodes):")
    generate_random_graph_streaming(100_000_000, 500_000_000, "data/heavy/random_100m.txt")
    generate_scale_free_fast(100_000_000, 5, "data/heavy/scale_free_100m.txt")
    generate_chain_graph(100_000_000, "data/heavy/chain_100m.txt")
    generate_grid_graph(10_000, 10_000, "data/heavy/grid_100m.txt")

    print()
    print("=" * 70)
    print("Done!")
    print("=" * 70)

    for root, dirs, files in os.walk("data"):
        for file in sorted(files):
            if file.endswith('.txt'):
                path = os.path.join(root, file)
                size = os.path.getsize(path) / (1024 * 1024)
                print(f"  {path:<45} {size:>8.2f} MB")


if __name__ == "__main__":
    main()
