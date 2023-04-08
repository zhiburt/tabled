//! This example demonstrates using the [attribute macro](https://doc.rust-lang.org/reference/procedural-macros.html#attribute-macros)
//! [`inline`] to expand enum fields to individual columns in a [`Table`] display.
//!
//! * Note how the optional [`inline`] argument is used to apply prefixes
//! to decomposed column headers. This is helpful for organizing tables
//! with repetative fields that would normally result in confusing headers.
//!
//! * Note that without inlining a struct or enum field, those objects
//! must implement the [`Display`] trait as they will be represented in
//! a single column with the value of their [`ToString`] output.

use tabled::{Table, Tabled};

#[derive(Tabled)]
enum Contact {
    #[tabled(inline("telegram::"))]
    Telegram {
        username: &'static str,
        #[tabled(inline("telegram::"))]
        number: Number,
    },
    #[tabled(inline)]
    Local(#[tabled(inline("local::"))] Number),
}

#[derive(Tabled)]
struct Number {
    number: &'static str,
    code: usize,
}

impl Number {
    fn new(number: &'static str, code: usize) -> Self {
        Self { number, code }
    }
}

fn main() {
    let data = [
        Contact::Local(Number::new("654321", 123)),
        Contact::Telegram {
            username: "no2Presley",
            number: Number::new("123456", 123),
        },
    ];

    let table = Table::new(data);

    println!("{table}");
}
