[<img alt="github" src="https://img.shields.io/badge/github-zhiburt/tabled-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/zhiburt/tabled/)
[<img alt="crates.io" src="https://img.shields.io/crates/v/tabled.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tabled)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tabled-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/tabled)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/zhiburt/tabled/Continuous%20integration/master?style=for-the-badge" height="20">](https://github.com/zhiburt/tabled/actions)
[<img alt="coverage" src="https://img.shields.io/coveralls/github/zhiburt/tabled/master?style=for-the-badge" height="20">](https://coveralls.io/github/zhiburt/tabled)
[<img alt="dependency status" src="https://deps.rs/repo/github/zhiburt/tabled/status.svg?style=for-the-badge" height="20">](https://deps.rs/repo/github/zhiburt/tabled)

# tabled

An easy to use library for pretty printing tables of Rust `struct`s and `enum`s.

There are more examples and you can find in this [`README`](https://github.com/zhiburt/tabled/blob/master/README.md).

## Usage

To print a list of structs or enums as a table your types should implement the the `Tabled` trait or derive it with a `#[derive(Tabled)]` macro.
Most of the default types implement the trait out of the box.

```rust
use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Language {
    name: String,
    designed_by: String,
    invented_year: usize,
}

impl Language {
    fn new(name: &str, designed_by: &str, invented_year: usize) -> Self {
        Self {
            name: name.to_string(),
            designed_by: designed_by.to_string(),
            invented_year,
        }
    }
}

let languages = vec![
    Language::new("C", "Dennis Ritchie", 1972),
    Language::new("Go", "Rob Pike", 2009),
    Language::new("Rust", "Graydon Hoare", 2010),
    Language::new("Hare", "Drew DeVault", 2022),
];

let table = Table::new(languages).to_string();

let expected = "+------+----------------+---------------+\n\
                | name | designed_by    | invented_year |\n\
                +------+----------------+---------------+\n\
                | C    | Dennis Ritchie | 1972          |\n\
                +------+----------------+---------------+\n\
                | Go   | Rob Pike       | 2009          |\n\
                +------+----------------+---------------+\n\
                | Rust | Graydon Hoare  | 2010          |\n\
                +------+----------------+---------------+\n\
                | Hare | Drew DeVault   | 2022          |\n\
                +------+----------------+---------------+";

assert_eq!(table, expected);
```
