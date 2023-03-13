# (`static_table`) Cook tables at compiler time

The library provides a macros to build a pretty tables at compile time.

To find a few features and settings which you can use with the macros please check out the documentation ([docs.rs](https://docs.rs/static_table)).

### Get started

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

Output

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

You can even use the macros in the documentation

```rust
/// Add joins 2 integers together to get a sum.
/// 
/// ```
#[doc = static_table::static_table!([
    ["a", "b", "result"],
    ["1", '2', '3'],
    ["2", '2', '4']
])]
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
```
