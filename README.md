# tabled

This library provides an interface to pretty print vectors of structs

## Get started

The common and probably the best way to begin is to annotate your type with
`#[derive(Tabled)]`. You can also implement it on your own as well.

There's an example. Precisely it can be printed and you
will see the content of `expected` variable as an output.

```rust
use tabled::{Tabled, table};

#[derive(Tabled)]
struct Language {
    name: String,
    designed_by: String,
    invented_year: usize,
}

let languages = vec![
    Language{
        name: "C".to_owned(),
        designed_by: "Dennis Ritchie".to_owned(),
        invented_year: 1972
    },
    Language{
        name: "Rust".to_owned(),
        designed_by: "Graydon Hoare".to_owned(),
        invented_year: 2010},
];

let table = table(languages);
let expected = "+------+----------------+---------------+\n\
                | name |  designed_by   | invented_year |\n\
                +------+----------------+---------------+\n\
                |  C   | Dennis Ritchie |     1972      |\n\
                +------+----------------+---------------+\n\
                | Rust | Graydon Hoare  |     2010      |\n\
                +------+----------------+---------------+\n";

assert_eq!(expected, table);
```

It should have a clue in what why print the field
accordingly each field should implement `std::fmt::Display`
The example below is not compiled

```rust,compile_fail
# use tabled::Tabled;
#[derive(Tabled)]
struct SomeType {
    field1: SomeOtherType,
}

struct SomeOtherType;
```
This crate implement the trait for default types.
Therefore you can use this to print one column vectors

```rust
use tabled::{Tabled, table};

let some_numbers = vec![1, 2, 3];
let table = table(some_numbers);
```
