# (`static_table`) Cook tables at compiler time

The library provides a macros to build a pretty tables at compile time.

## Get started

### Example

```rust
use static_table::static_table;

static LANG_LIST: &str = static_table!([
    ["name", "designed by", "first release"],
    ["C", "Dennis Ritchie", "1972"],
    ["Go", "Rob Pike", "2009"],
    ["Rust", "Graydon Hoare", "2010"],
]);

fn main() {
    println!("{LANG_LIST}")
}
```

### Output

```text
+------+----------------+---------------+
| name | designed by    | first release |
+------+----------------+---------------+
| C    | Dennis Ritchie | 1972          |
+------+----------------+---------------+
| Go   | Rob Pike       | 2009          |
+------+----------------+---------------+
| Rust | Graydon Hoare  | 2010          |
+------+----------------+---------------+
```

To find a few features and settings which you can use with the macros please check out the documentation ([docs.rs](https://docs.rs/static_table)).
