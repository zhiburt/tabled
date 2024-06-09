# (`static_table`) Cook tables at compiler time

The library provides a macros to build a pretty tables at compile time.

There's 2 types of tables you can build.

1. Standard adjusted tables (`static_table::static_table`).
2. Not adjusted, floating tables (`static_table::pool_table`).

To find a more features and settings which you can use with the macros please check out the documentation ([docs.rs](https://docs.rs/static_table)).

## Get started

Add the library to your `Cargo.toml`.

```toml
# Cargo.toml
[dependencies]
static_table = "0.2"
```

An example of `static_table` usage.

<table>
<tr>
<th> Example </th>
<th> Result </th>
</tr>
<tr>
<td>


```rust
use static_table::static_table;

static LANG_LIST: &str = static_table!([
    ["name", "designed by", "first release"],
    ["C", "Dennis Ritchie", "1972"],
    ["Go", "Rob Pike", "2009"],
    ["Rust", "Graydon Hoare", "2010"],
    ["Hare", "Drew DeVault", "2022"],
]);

fn main() {
    println!("{LANG_LIST}")
}
```

</td>
<td style="vertical-align: top;">

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
| Hare | Drew DeVault   | 2022          |
+------+----------------+---------------+
```

</td>
</tr>
</table>

An example of `pool_table` usage.

<table>
<tr>
<th> Example </th>
<th> Result </th>
</tr>
<tr>
<td>

```rust
use static_table::pool_table;

static LANG_LIST: &str = pool_table!([
    ["name", "designed by", "first release"],
    ["C", "Dennis Ritchie", "1972"],
    ["Go", "Rob Pike", "2009"],
    ["Rust", "Graydon Hoare", "2010"],
    ["Hare", "Drew DeVault", "2022"],
]);

fn main() {
    println!("{LANG_LIST}")
}
```

</td>
<td style="vertical-align: top;">

```text
+------+-------------+---------------+
| name | designed by | first release |
+------+-------------+-----+---------+
| C    | Dennis Ritchie    | 1972    |
+------+--+---------------++---------+
| Go      | Rob Pike      | 2009     |
+---------+---------------+-+--------+
| Rust    | Graydon Hoare   | 2010   |
+---------+-----------------+--------+
| Hare    | Drew DeVault    | 2022   |
+---------+-----------------+--------+
```

</td>
</tr>
</table>

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

## Binary size concern

It's something you shall be aware of.
Using `static_table` MIGHT increase a binary size, because the table will be stored as actuall symbols in a static section of a binary file (ELF, PE etc.).

I have run a few tests in this regard.
And a binary which used `static_table` has SUBSTATIANALY smaller size than
a binary with a build table at runtime using `lazy_static`/`once_cell`.
I am not sure though why it is a case.

```table
                debug mode      release mode
static_table    13497232        4501576
runtime table   12031120        4156024
```