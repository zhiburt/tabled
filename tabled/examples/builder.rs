//! This example demonstrates an alternative method for creating a [`Table`].
//! [`Builder`] is an efficient implementation of the [builder design pattern](https://en.wikipedia.org/wiki/Builder_pattern).
//!
//! > The intent of the Builder design pattern is to separate the construction of a complex object from its representation.
//! > -- <cite>Wikipedia</cite>
//!
//! * Note how [Builder] can be used to define a table's shape manually
//!  and can be populated through iteration if it is mutable. This flexibility
//! is useful when you don't have direct control over the datasets you intend to [table](tabled).

use tabled::{
    builder::Builder,
    settings::{object::Rows, Modify, Panel, Style, Width},
};

fn main() {
    let message = r#"The terms "the ocean" or "the sea" used without specification refer to the interconnected body of salt water covering the majority of the Earth's surface"#;
    let link = r#"https://en.wikipedia.org/wiki/Ocean"#;

    let oceans = ["Atlantic", "Pacific", "Indian", "Southern", "Arctic"];

    let mut builder = Builder::default();
    builder.set_header(["#", "Ocean"]);
    for (i, ocean) in oceans.iter().enumerate() {
        builder.push_record([i.to_string(), ocean.to_string()]);
    }

    let table = builder
        .build()
        .with(Panel::header(message))
        .with(Panel::header(link))
        .with(Panel::horizontal(2, "=".repeat(link.len())))
        .with(Modify::new(Rows::single(1)).with(Width::wrap(link.len())))
        .with(Style::markdown())
        .to_string();

    println!("{table}");
}
