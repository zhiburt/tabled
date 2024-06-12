# Benchmark's

We profile only actuall table construction.

**Be WARE** that it's being run agains a specific (but general) use case.
Some libraries **might** perform better in certain scenarios or certain use cases.

## Result

|                          | cli_table      | comfy_table    | prettytable_rs      | term_table     | tabled             | tabled_color     | tabled_compact     | tabled_iter    |
|--------------------------|----------------|----------------|---------------------|----------------|--------------------|------------------|--------------------|----------------|
| test_const_table/1       | 2.8±0.04µs     | 5.4±0.08µs     | 1473.2±19.23ns      | 4.9±0.04µs     | 1500.4±53.51ns     | 1535.0±22.53ns   | **1253.3±21.35ns** | 2.9±0.02µs     |
| test_const_table/8       | 56.5±0.81µs    | 61.3±1.66µs    | 30.6±1.88µs         | 102.1±1.19µs   | 21.6±0.40µs        | **20.7±0.44µs**  | 21.5±0.48µs        | 43.0±0.56µs    |
| test_const_table/32      | 750.8±4.82µs   | 805.3±4.59µs   | 406.9±18.39µs       | 1332.3±31.11µs | 272.0±2.25µs       | **260.8±6.25µs** | 294.2±8.08µs       | 522.4±5.24µs   |
| test_const_table/128     | 10.0±0.07ms    | 12.6±0.30ms    | 6.9±0.29ms          | 19.2±0.31ms    | **4.1±0.03ms**     | 4.3±0.09ms       | 4.2±0.13ms         | 8.1±0.19ms     |
| test_const_table/512     | 212.7±18.45ms  | 225.1±15.51ms  | 251.4±23.59ms       | 322.4±10.87ms  | 72.6±0.88ms        | **70.5±1.24ms**  | 92.0±12.93ms       | 128.3±0.96ms   |
| test_dynamic_table/1     | 3.0±0.23µs     | 6.5±0.71µs     | 1288.9±269.46ns     | 3.4±0.16µs     | 1392.4±118.82ns    | 1482.2±84.78ns   | **1084.4±89.54ns** | 2.7±0.08µs     |
| test_dynamic_table/8     | 58.0±6.54µs    | 67.5±4.11µs    | 22.7±2.07µs         | 80.4±1.77µs    | 19.5±0.68µs        | 18.6±2.68µs      | **17.6±0.70µs**    | 46.0±1.86µs    |
| test_dynamic_table/32    | 652.7±18.54µs  | 863.2±22.43µs  | 326.8±13.28µs       | 1046.0±22.34µs | 270.6±15.57µs      | **264.0±6.53µs** | 269.7±16.20µs      | 527.3±10.58µs  |
| test_dynamic_table/128   | 10.5±0.79ms    | 13.5±0.54ms    | 5.5±0.17ms          | 18.1±0.48ms    | **3.8±0.11ms**     | 3.9±0.18ms       | 4.9±0.38ms         | 7.7±0.19ms     |
| test_dynamic_table/512   | 176.4±3.46ms   | 214.9±9.37ms   | 202.8±7.90ms        | 313.9±6.36ms   | 70.5±1.36ms        | **67.4±1.15ms**  | 75.7±1.75ms        | 141.0±2.28ms   |
| test_empty_table/1       | 1840.6±83.30ns | 5.9±0.35µs     | 889.1±38.82ns       | 2.7±0.08µs     | 1041.8±58.11ns     | 1121.5±61.40ns   | **710.2±33.80ns**  | 1911.7±93.26ns |
| test_empty_table/8       | 26.6±0.67µs    | 60.9±3.88µs    | 12.6±0.79µs         | 55.3±3.04µs    | 9.7±0.43µs         | 9.1±0.33µs       | **8.9±0.43µs**     | 16.0±0.59µs    |
| test_empty_table/32      | 300.5±3.95µs   | 706.2±8.68µs   | 145.7±8.98µs        | 636.7±3.12µs   | 111.3±4.77µs       | 106.4±1.03µs     | **104.4±0.67µs**   | 170.0±1.35µs   |
| test_empty_table/128     | 4.3±0.03ms     | 11.3±0.22ms    | 2.9±0.46ms          | 9.8±0.09ms     | **1528.6±43.02µs** | 1557.2±18.06µs   | 1636.9±24.27µs     | 2.4±0.02ms     |
| test_empty_table/512     | 74.7±0.77ms    | 176.8±2.60ms   | 132.1±2.29ms        | 160.1±2.56ms   | **25.0±0.58ms**    | 25.6±0.37ms      | 25.9±1.40ms        | 36.9±0.89ms    |
| test_multiline_table/1   | 9.7±0.21µs     | 16.0±0.90µs    | **4.0±0.28µs**      | 13.5±0.50µs    | 4.9±0.10µs         | 5.0±0.16µs       | *2.3±0.23µs*       | 10.2±0.64µs    |
| test_multiline_table/8   | 213.5±4.17µs   | 292.9±6.89µs   | **84.1±2.37µs**     | 410.5±22.56µs  | 117.2±2.14µs       | 111.8±1.97µs     | *55.1±1.23µs*      | 235.6±1.70µs   |
| test_multiline_table/32  | 2.9±0.14ms     | 3.6±0.03ms     | **1222.1±54.94µs**  | 5.2±0.11ms     | 1628.7±28.60µs     | 1670.8±17.86µs   | *795.9±12.55µs*    | 3.6±0.07ms     |
| test_multiline_table/128 | 44.5±2.40ms    | 60.5±2.60ms    | **19.6±1.03ms**     | 84.0±4.32ms    | 27.2±0.76ms        | 25.5±0.84ms      | *12.1±0.31ms*      | 53.1±0.90ms    |
| test_multiline_table/512 | 730.6±52.51ms  | 1011.1±49.42ms | 469.6±21.87ms       | 1327.6±82.87ms | **423.7±17.54ms**  | 429.0±6.26ms     | *189.0±10.82ms*    | 802.9±33.90ms  |

* tabled_compact doesn't support multiline strings

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
