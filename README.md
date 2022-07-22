# A benchmark table

|    | tabled | tabled-color | comfy-table | cli-table | term-table |
|:--:|:------:|:------------:|:-----------:|:---------:|:----------:|
| test_empty_table/512     | 80.2±0.98ms  | 83.8±1.19ms  | 106.5±1.21ms | **54.9±0.59ms**  | 130.1±0.55ms |
| test_const_table/512     | 128.3±1.21ms | **123.6±0.97ms** | 128.1±1.06ms | 126.8±1.05ms | 261.4±1.48ms |
| test_multiline_table/512 | 366.7±1.36ms | 365.7±1.51ms | **358.9±1.21ms** | 416.8±1.84ms | 935.6±1.44ms |
| test_dynamic_table/512   | **103.8±1.06ms** | 103.4±1.14ms | 108.2±0.77ms | 104.5±0.82msms | 225.2±2.34ms |

#### System

```
Kernel: 5.18.9-arch1-1 
CPU: 11th Gen Intel i7-11850H (16) @ 4.800GHz
```
