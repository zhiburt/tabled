//! This example demonstrates instantiating a [`Table`] from an [`IntoIterator`] compliant object.
//!
//! * Note how [`Range`] [expression syntax](https://doc.rust-lang.org/reference/expressions/range-expr.html)
//!   is used to idiomatically represent the English alphabet.

use tabled::{Table, Tabled};

#[derive(Tabled)]
struct Repository {
    name: String,
    owner: String,
    #[tabled(inline)]
    head: Option<Commit>,
    #[tabled(inline)]
    updated_at: Option<usize>,
}

#[derive(Tabled)]
struct Commit {
    at: u64,
    hash: String,
}

fn main() {
    let repos = vec![
        Repository {
            name: String::from("django-sheets"),
            owner: String::from("georgewhewell"),
            head: None,
            updated_at: Some(0),
        },
        Repository {
            name: String::from("undervolt"),
            owner: String::from("georgewhewell"),
            head: None,
            updated_at: None,
        },
        Repository {
            name: String::from("msp-elixir"),
            owner: String::from("georgewhewell"),
            head: Some(Commit {
                at: 0,
                hash: String::from("4342f01"),
            }),
            updated_at: None,
        },
    ];

    let table = Table::new(repos);

    println!("{table}");
}
