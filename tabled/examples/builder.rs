//! This example demonstrates an alternative method for creating a [`Table`].
//! [`Builder`] is an efficient implementation of the [builder design pattern](https://en.wikipedia.org/wiki/Builder_pattern).
//!
//! > The intent of the Builder design pattern is to separate the construction of a complex object from its representation.
//! > -- <cite>Wikipedia</cite>
//!
//! * Note how [Builder] can be used to define a table's shape manually
//!   and can be populated through iteration if it is mutable. This flexibility
//!   is useful when you don't have direct control over the datasets you intend to [table](tabled).

use tabled::{builder::Builder, settings::Style};

fn main() {
    let oceans = "Atlantic, Pacific, Indian, Southern, Arctic";

    let mut builder = Builder::default();

    builder.push_record(["#", "Ocean"]);
    for (i, ocean) in oceans.split(", ").enumerate() {
        builder.push_record([i.to_string(), ocean.to_string()]);
    }

    let mut table = builder.build();
    table.with(Style::markdown().remove_horizontals());

    println!("{table}");
}
