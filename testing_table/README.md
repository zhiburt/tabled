# `testing_table`

A library which provides handy macros for table testing.

Includes

- `test_table!`
- `static_table!`
- `assert_table!`
- `assert_width!`

An example.

```rust
test_table!(
    test_tabled,
    tabled::Table::new([[1, 2, 3]]),
    "+---+---+---+"
    "| 0 | 1 | 2 |"
    "+---+---+---+"
    "| 1 | 2 | 3 |"
    "+---+---+---+"
);
```