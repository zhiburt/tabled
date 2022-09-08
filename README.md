# Benchmark's result

|                          | cli_table      | comfy_table   | tabled         | tabled_color   | term_table     |
|--------------------------|----------------|---------------|----------------|----------------|----------------|
| test_const_table/1       | 2.3±0.04µs     | 5.0±0.15µs    | 2.0±0.03µs     | **1877.8±25.43ns** | 4.5±0.08µs     |
| test_const_table/128     | 8.1±0.19ms     | 11.1±0.13ms   | 6.0±0.07ms     | **5.8±0.15ms**     | 17.4±0.42ms    |
| test_const_table/32      | 596.6±5.45µs   | 771.6±8.90µs  | **381.8±4.83µs**   | 385.5±4.35µs   | 1279.5±35.76µs |
| test_const_table/512     | 153.1±3.24ms   | 178.8±1.99ms  | 99.2±3.88ms    | **98.3±3.20ms**    | 271.1±1.90ms   |
| test_const_table/8       | 47.8±0.81µs    | 58.4±0.62µs   | 30.7±0.98µs    | **30.5±0.86µs**    | 90.9±1.17µs    |
| test_dynamic_table/1     | 2.1±0.03µs     | 4.7±0.19µs    | **1453.4±66.51ns** | 1506.0±23.59ns | 2.7±0.04µs     |
| test_dynamic_table/128   | 6.7±0.14ms     | 9.9±0.14ms    | 5.1±0.05ms     | **4.8±0.06ms**     | 13.9±0.17ms    |
| test_dynamic_table/32    | 486.1±8.53µs   | 679.5±13.02µs | 319.1±5.43µs   | **307.7±6.14µs**   | 834.7±21.18µs  |
| test_dynamic_table/512   | 136.1±8.08ms   | 164.4±2.18ms  | **88.2±3.37ms**    | 89.4±2.96ms    | 237.8±3.68ms   |
| test_dynamic_table/8     | 38.4±0.93µs    | 48.1±1.15µs   | 23.4±0.41µs    | **21.8±0.33µs**    | 58.1±1.92µs    |
| test_empty_table/1       | 1429.3±39.39ns | 4.5±0.14µs    | **1234.4±33.92ns** | 1313.5±39.65ns | 2.2±0.04µs     |
| test_empty_table/128     | **3.6±0.05ms**     | 9.7±0.13ms    | **3.6±0.05ms**     | 3.7±0.03ms     | 8.8±0.15ms     |
| test_empty_table/32      | 251.4±7.45µs   | 641.9±12.12µs | **226.7±7.98µs**   | 230.2±3.73µs   | 592.8±20.88µs  |
| test_empty_table/512     | 67.1±1.84ms    | 165.3±4.03ms  | 60.9±3.44ms    | **58.9±5.71ms**    | 139.9±4.70ms   |
| test_empty_table/8       | 21.0±0.58µs    | 49.0±1.04µs   | **17.7±0.51µs**    | 17.9±0.33µs    | 45.5±1.14µs    |
| test_multiline_table/1   | 7.3±0.32µs     | 11.5±0.12µs   | **4.7±0.10µs**     | 5.0±0.26µs     | 10.6±0.11µs    |
| test_multiline_table/128 | 28.4±0.63ms    | 44.7±0.68ms   | 24.9±0.26ms    | **24.2±0.33ms**    | 65.6±0.69ms    |
| test_multiline_table/32  | 1931.1±12.51µs | 2.9±0.05ms    | 1632.1±14.61µs | **1570.2±14.42µs** | 4.5±0.04ms     |
| test_multiline_table/512 | 485.1±3.10ms   | 762.2±5.13ms  | 422.8±3.41ms   | **390.8±6.28ms**   | 1026.2±2.90ms  |
| test_multiline_table/8   | 156.6±2.01µs   | 226.6±3.72µs  | **110.5±1.47µs**   | 111.1±1.01µs   | 304.7±3.16µs   |

## Table Generation

You can generate the table by this command (it relays on `critcmp`).

```bash
cargo run --manifest-path=readme/Cargo.toml
```

#### System

```
Kernel: 5.18.9-arch1-1 
CPU: 11th Gen Intel i7-11850H (16) @ 4.800GHz
```
