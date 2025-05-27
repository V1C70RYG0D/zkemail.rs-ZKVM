# ZKVM Benchmark Results

| Size (KB) | Time (ms) | Cycles | Memory (KB) | Throughput (emails/sec) |
| --------- | --------- | ------ | ----------- | ----------------------- |
|        1 |        1 |  71240 |          3 |                1000.00 |
|        5 |        1 | 112200 |         15 |                1000.00 |
|       10 |        1 | 163400 |         30 |                1000.00 |
|       25 |        1 | 317000 |         75 |                1000.00 |
|       50 |        1 | 573000 |        150 |                1000.00 |
|      100 |        1 | 1085000 |        300 |                1000.00 |

## Analysis
- Size scaling: 100.00x (1 -> 100KB)
- Cycles scaling: 15.23x
- Memory scaling: 84.23x
- Cycles/Size efficiency: 0.15x
- Memory/Size efficiency: 0.84x
