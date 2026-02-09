| Algorithm   | Graph     | Nodes   | Edges   |   Seq Time (ms) |   Best Par Time (ms) |   Best Threads | Max Speedup   |
|:------------|:----------|:--------|:--------|----------------:|---------------------:|---------------:|:--------------|
| BFS         | random_1k | 1,000   | 5,000   |            0.03 |                 0.01 |             16 | 3.50x         |
| WCC         | random_1k | 1,000   | 5,000   |            0.06 |                 0.04 |              8 | 1.41x         |
| PageRank    | random_1k | 1,000   | 5,000   |            0.12 |                 0.11 |             16 | 1.14x         |