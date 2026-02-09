#!/usr/bin/env python3
"""
Vizualizacija benchmark rezultata
"""

import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
import sys
import os

# Konfiguraj matplotlib za lepe grafike
plt.style.use('seaborn-v0_8-darkgrid')
plt.rcParams['figure.figsize'] = (12, 8)
plt.rcParams['font.size'] = 10

def plot_speedup_by_threads(df, output_dir):
    """Speedup vs broj threadova za svaki algoritam"""
    
    algorithms = df['algorithm'].unique()
    graphs = df['graph_name'].unique()
    
    for graph in graphs:
        fig, axes = plt.subplots(1, len(algorithms), figsize=(15, 5))
        if len(algorithms) == 1:
            axes = [axes]
        
        fig.suptitle(f'Speedup Analysis - {graph}', fontsize=16, fontweight='bold')
        
        for idx, algo in enumerate(algorithms):
            ax = axes[idx]
            
            data = df[(df['algorithm'] == algo) & 
                     (df['graph_name'] == graph) & 
                     (df['mode'] == 'par')]
            
            if data.empty:
                continue
            
            threads = data['threads'].values
            speedup = data['speedup'].values
            
            # Plot actual speedup
            ax.plot(threads, speedup, 'o-', linewidth=2, markersize=8, label='Actual')
            
            # Plot ideal speedup (linear)
            ax.plot(threads, threads, '--', linewidth=2, alpha=0.5, label='Ideal (Linear)')
            
            ax.set_xlabel('Number of Threads', fontweight='bold')
            ax.set_ylabel('Speedup', fontweight='bold')
            ax.set_title(f'{algo}', fontweight='bold')
            ax.grid(True, alpha=0.3)
            ax.legend()
            
            # Annotate max speedup
            max_speedup_idx = speedup.argmax()
            ax.annotate(f'{speedup[max_speedup_idx]:.2f}x',
                       xy=(threads[max_speedup_idx], speedup[max_speedup_idx]),
                       xytext=(10, 10), textcoords='offset points',
                       bbox=dict(boxstyle='round', facecolor='yellow', alpha=0.7),
                       fontweight='bold')
        
        plt.tight_layout()
        filename = f'speedup_{graph.replace("/", "_")}.png'
        plt.savefig(os.path.join(output_dir, filename), dpi=300, bbox_inches='tight')
        print(f'  ✓ {filename}')
        plt.close()

def plot_algorithm_comparison(df, output_dir):
    """Poređenje algoritama na istom grafu"""
    
    graphs = df['graph_name'].unique()
    
    for graph in graphs:
        graph_data = df[df['graph_name'] == graph]
        
        fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(14, 6))
        fig.suptitle(f'Algorithm Comparison - {graph}', fontsize=16, fontweight='bold')
        
        # Plot 1: Execution time
        algorithms = graph_data['algorithm'].unique()
        seq_times = []
        par_times = []
        algo_labels = []
        
        for algo in algorithms:
            seq = graph_data[(graph_data['algorithm'] == algo) & (graph_data['mode'] == 'seq')]
            par = graph_data[(graph_data['algorithm'] == algo) & (graph_data['mode'] == 'par')]
            
            if not seq.empty:
                seq_times.append(seq['time_ms'].values[0])
                algo_labels.append(algo)
                
                if not par.empty:
                    # Uzmi najbolji paralelni rezultat
                    best_par = par.loc[par['speedup'].idxmax()]
                    par_times.append(best_par['time_ms'])
                else:
                    par_times.append(0)
        
        x = np.arange(len(algo_labels))
        width = 0.35
        
        bars1 = ax1.bar(x - width/2, seq_times, width, label='Sequential', alpha=0.8)
        bars2 = ax1.bar(x + width/2, par_times, width, label='Parallel (Best)', alpha=0.8)
        
        ax1.set_ylabel('Time (ms)', fontweight='bold')
        ax1.set_title('Execution Time', fontweight='bold')
        ax1.set_xticks(x)
        ax1.set_xticklabels(algo_labels)
        ax1.legend()
        ax1.grid(True, alpha=0.3, axis='y')
        
        # Add value labels on bars
        for bars in [bars1, bars2]:
            for bar in bars:
                height = bar.get_height()
                if height > 0:
                    ax1.text(bar.get_x() + bar.get_width()/2., height,
                            f'{height:.1f}',
                            ha='center', va='bottom', fontsize=9)
        
        # Plot 2: Max speedup
        max_speedups = []
        
        for algo in algo_labels:
            par = graph_data[(graph_data['algorithm'] == algo) & (graph_data['mode'] == 'par')]
            if not par.empty:
                max_speedups.append(par['speedup'].max())
            else:
                max_speedups.append(1.0)
        
        bars = ax2.bar(algo_labels, max_speedups, alpha=0.8, color=['#1f77b4', '#ff7f0e', '#2ca02c'][:len(algo_labels)])
        
        ax2.set_ylabel('Speedup', fontweight='bold')
        ax2.set_title('Maximum Speedup Achieved', fontweight='bold')
        ax2.axhline(y=1, color='r', linestyle='--', alpha=0.5, label='Baseline')
        ax2.legend()
        ax2.grid(True, alpha=0.3, axis='y')
        
        # Add value labels
        for bar, speedup in zip(bars, max_speedups):
            height = bar.get_height()
            ax2.text(bar.get_x() + bar.get_width()/2., height,
                    f'{speedup:.2f}x',
                    ha='center', va='bottom', fontweight='bold', fontsize=11)
        
        plt.tight_layout()
        filename = f'comparison_{graph.replace("/", "_")}.png'
        plt.savefig(os.path.join(output_dir, filename), dpi=300, bbox_inches='tight')
        print(f'  ✓ {filename}')
        plt.close()

def plot_scalability_analysis(df, output_dir):
    """Analiza skalabilnosti - efikasnost sa porastom threadova"""
    
    fig, axes = plt.subplots(1, 3, figsize=(16, 5))
    fig.suptitle('Parallel Efficiency Analysis', fontsize=16, fontweight='bold')
    
    algorithms = ['BFS', 'WCC', 'PageRank']
    
    for idx, algo in enumerate(algorithms):
        ax = axes[idx]
        
        algo_data = df[(df['algorithm'] == algo) & (df['mode'] == 'par')]
        
        if algo_data.empty:
            continue
        
        graphs = algo_data['graph_name'].unique()
        
        for graph in graphs:
            graph_data = algo_data[algo_data['graph_name'] == graph]
            threads = graph_data['threads'].values
            speedup = graph_data['speedup'].values
            
            # Efficiency = speedup / threads
            efficiency = speedup / threads * 100  # u procentima
            
            ax.plot(threads, efficiency, 'o-', linewidth=2, markersize=6, label=graph)
        
        ax.set_xlabel('Number of Threads', fontweight='bold')
        ax.set_ylabel('Parallel Efficiency (%)', fontweight='bold')
        ax.set_title(f'{algo}', fontweight='bold')
        ax.axhline(y=100, color='r', linestyle='--', alpha=0.3, label='Perfect (100%)')
        ax.grid(True, alpha=0.3)
        ax.legend(fontsize=8)
        ax.set_ylim(0, 110)
    
    plt.tight_layout()
    filename = 'scalability_analysis.png'
    plt.savefig(os.path.join(output_dir, filename), dpi=300, bbox_inches='tight')
    print(f'  ✓ {filename}')
    plt.close()

def generate_summary_table(df, output_dir):
    """Generiši tabelu sa summary rezultatima"""
    
    summary_data = []
    
    for algo in df['algorithm'].unique():
        for graph in df['graph_name'].unique():
            data = df[(df['algorithm'] == algo) & (df['graph_name'] == graph)]
            
            if data.empty:
                continue
            
            seq = data[data['mode'] == 'seq']
            par = data[data['mode'] == 'par']
            
            if seq.empty:
                continue
            
            seq_time = seq['time_ms'].values[0]
            nodes = seq['nodes'].values[0]
            edges = seq['edges'].values[0]
            
            if not par.empty:
                best_par = par.loc[par['speedup'].idxmax()]
                max_speedup = best_par['speedup']
                best_threads = int(best_par['threads'])
                par_time = best_par['time_ms']
            else:
                max_speedup = 1.0
                best_threads = 1
                par_time = seq_time
            
            summary_data.append({
                'Algorithm': algo,
                'Graph': graph,
                'Nodes': f'{nodes:,}',
                'Edges': f'{edges:,}',
                'Seq Time (ms)': f'{seq_time:.2f}',
                'Best Par Time (ms)': f'{par_time:.2f}',
                'Best Threads': best_threads,
                'Max Speedup': f'{max_speedup:.2f}x'
            })
    
    summary_df = pd.DataFrame(summary_data)
    
    # Save to CSV
    csv_path = os.path.join(output_dir, 'summary_table.csv')
    summary_df.to_csv(csv_path, index=False)
    print(f'  ✓ summary_table.csv')
    
    # Save to markdown
    md_path = os.path.join(output_dir, 'summary_table.md')
    with open(md_path, 'w') as f:
        f.write(summary_df.to_markdown(index=False))
    print(f'  ✓ summary_table.md')

def main():
    if len(sys.argv) < 2:
        print("Usage: python3 visualize_benchmark.py <benchmark_results.csv>")
        sys.exit(1)
    
    csv_file = sys.argv[1]
    
    if not os.path.exists(csv_file):
        print(f"Error: File not found: {csv_file}")
        sys.exit(1)
    
    script_dir = os.path.dirname(os.path.abspath(__file__))
    output_dir = os.path.join(script_dir, 'results', 'plots')
    os.makedirs(output_dir, exist_ok=True)
    
    print("=" * 70)
    print("Benchmark Visualization")
    print("=" * 70)
    print(f"\nReading data from: {csv_file}")
    
    # Load data
    df = pd.read_csv(csv_file)
    
    print(f"Loaded {len(df)} benchmark results")
    print(f"\nAlgorithms: {', '.join(df['algorithm'].unique())}")
    print(f"Graphs: {', '.join(df['graph_name'].unique())}")
    
    print(f"\nGenerating plots in: {output_dir}/")
    
    # Generate plots
    print("\n1. Speedup by threads...")
    plot_speedup_by_threads(df, output_dir)
    
    print("\n2. Algorithm comparison...")
    plot_algorithm_comparison(df, output_dir)
    
    print("\n3. Scalability analysis...")
    plot_scalability_analysis(df, output_dir)
    
    print("\n4. Summary table...")
    generate_summary_table(df, output_dir)
    
    print("\n" + "=" * 70)
    print("Visualization complete!")
    print(f"Check results in: {output_dir}/")
    print("=" * 70)

if __name__ == "__main__":
    main()