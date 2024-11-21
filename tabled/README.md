[<img alt="github" src="https://img.shields.io/badge/github-zhiburt/tabled-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/zhiburt/tabled/)
[<img alt="crates.io" src="https://img.shields.io/crates/v/tabled.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/tabled)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-tabled-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/tabled)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/zhiburt/tabled/ci.yml?branch=master&style=for-the-badge" height="20">](https://github.com/zhiburt/tabled/actions)
[<img alt="coverage" src="https://img.shields.io/coveralls/github/zhiburt/tabled/master?style=for-the-badge" height="20">](https://coveralls.io/github/zhiburt/tabled)
[<img alt="dependency status" src="https://deps.rs/repo/github/zhiburt/tabled/status.svg?style=for-the-badge" height="20">](https://deps.rs/repo/github/zhiburt/tabled)

# tabled

An easy to use library for pretty printing tables of Rust `struct`s and `enum`s.

There are more examples and you can find in this [`README`](https://github.com/zhiburt/tabled/blob/master/README.md).

## Usage

To print a list of structs or enums as a table your types should implement the the `Tabled` trait or derive it with a `#[derive(Tabled)]` macro.
Most of the default types implement the trait out of the box.

Most of a table configuration can be found in [`tabled::settings`](https://docs.rs/tabled/latest/tabled/settings/index.html) module.

```rust
use tabled::{Table, Tabled};
use testing_table::assert_table;

#[derive(Tabled)]
struct Language<'a> {
    name: &'a str,
    designed_by: &'a str,
    invented_year: usize,
}

let languages = vec![
    Language { name: "C",    designed_by: "Dennis Ritchie", invented_year: 1972 },
    Language { name: "Go",   designed_by: "Rob Pike",       invented_year: 2009 },
    Language { name: "Rust", designed_by: "Graydon Hoare",  invented_year: 2010 },
    Language { name: "Hare", designed_by: "Drew DeVault",   invented_year: 2022 },
];

let table = Table::new(languages);

assert_table!(
    table,
    "+------+----------------+---------------+"
    "| name | designed_by    | invented_year |"
    "+------+----------------+---------------+"
    "| C    | Dennis Ritchie | 1972          |"
    "+------+----------------+---------------+"
    "| Go   | Rob Pike       | 2009          |"
    "+------+----------------+---------------+"
    "| Rust | Graydon Hoare  | 2010          |"
    "+------+----------------+---------------+"
    "| Hare | Drew DeVault   | 2022          |"
    "+------+----------------+---------------+"
);
```

The same example but we are building a table step by step.

```rust
use tabled::{builder::Builder, settings::Style};
use testing_table::assert_table;

let mut builder = Builder::new();
builder.push_record(["C", "Dennis Ritchie", "1972"]);
builder.push_record(["Go", "Rob Pike", "2009"]);
builder.push_record(["Rust", "Graydon Hoare", "2010"]);
builder.push_record(["Hare", "Drew DeVault", "2022"]);

let mut table = builder.build();
table.with(Style::ascii_rounded());

assert_table!(
    table,
    ".------------------------------."
    "| C    | Dennis Ritchie | 1972 |"
    "| Go   | Rob Pike       | 2009 |"
    "| Rust | Graydon Hoare  | 2010 |"
    "| Hare | Drew DeVault   | 2022 |"
    "'------------------------------'"
);
```