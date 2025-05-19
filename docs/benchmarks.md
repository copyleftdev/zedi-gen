# Benchmark Report

Below are the latest benchmark results for key synthetic-data operations, measured with Criterion.rs (sample size: 10, plotting disabled).

| Benchmark               | Time per 1000 items                  |
|-------------------------|--------------------------------------|
| `generate_1000_claims`  | [8.3510 ms 8.3963 ms 8.4517 ms]      |
| `generate_1000_people`  | [1.1423 ms 1.1447 ms 1.1495 ms]      |
| `generate_1000_providers` | [816.42 µs 821.38 µs 826.86 µs]     |

*Change statistics (vs. previous run):*

```
generate_1000_claims    change: [-68.098% -67.737% -67.422%] (p < 0.05) - Performance improved
generate_1000_people    change: [-0.313% +0.5895% +1.523%] (p > 0.05) - No significant change
generate_1000_providers change: [+1.874% +2.897% +3.9904%] (p < 0.05) - Performance regressed
```

*Benchmark run command:*

```bash
# Disable compiler warnings and plotting to focus on measurement
RUSTFLAGS='-Awarnings' cargo bench
```