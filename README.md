# A benchmark table

|    | tabled | tabled-color | comfy-table | cli-table | term-table |
|:--:|:------:|:------------:|:-----------:|:---------:|:----------:|
| test_empty_table/512     | 96.7±1.71ms  | 96.5±1.41ms  | 111.4±0.81ms | **56.6±0.51ms**  | 128.1±0.37ms |
| test_const_table/512     | 134.7±0.78ms | 137.3±2.43ms | **129.4±0.87ms** | 129.7±0.97ms | 248.2±0.74ms |
| test_multiline_table/512 | 418.4±1.47ms | 403.9±1.22ms | **374.0±1.53ms** | 431.5±2.22ms | 935.5±4.42ms |
| test_dynamic_table/512   | 110.3±1.03ms | 112.0±1.07ms | 109.8±0.72ms | **106.0±0.90ms** | 216.6±0.76ms |

#### System

```
Kernel: 5.18.9-arch1-1 
CPU: 11th Gen Intel i7-11850H (16) @ 4.800GHz
```
