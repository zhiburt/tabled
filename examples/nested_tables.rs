//! The example can be run by this command
//! `cargo run --example nested_tables`
//!
//! The table is a take on the one from https://github.com/p-ranav/tabulate#nested-tables

use std::iter::FromIterator;

use tabled::{
    builder::Builder,
    object::{Rows, Segment},
    Alignment, MinWidth, Modify, Padding, Style, Table,
};

fn main() {
    let animal = create_class(
        "Animal",
        &[("age", "Int", ""), ("gender", "String", "")],
        &["isMammal", "mate"],
    );

    let duck = create_class(
        "Duck",
        &[("beakColor", "String", "yellow")],
        &["swim", "quack"],
    );

    let t = Builder::from_iter([
        [animal.to_string()],
        [String::from("▲")],
        [String::from("|")],
        [String::from("|")],
        [duck.to_string()],
    ])
    .build()
    .with(Style::ascii().header_off().horizontal_off())
    .with(Modify::new(Segment::all()).with(Padding::new(5, 5, 0, 0)));

    println!("{}", t);
}

fn create_class(name: &str, fields: &[(&str, &str, &str)], methods: &[&str]) -> Table {
    let clean_ascii_style = Style::ascii().header_off().horizontal_off().vertical_off();

    let mut table_fields = Builder::from_iter(fields.iter().map(|(field, t, d)| {
        if d.is_empty() {
            [format!("+{}: {}", field, t)]
        } else {
            [format!("+{}: {} = {:?}", field, t, d)]
        }
    }))
    .build()
    .with(clean_ascii_style.clone());

    let mut table_methods =
        Builder::from_iter(methods.iter().map(|method| [format!("+{}()", method)]))
            .build()
            .with(clean_ascii_style);

    // We want to make a fields table and methods table have the same width.
    // To not set it to constant, we check a width of each of them and correct the other.
    //
    // it's safe to do .len() because we use ascii theme.
    let table_fields_width = table_fields.to_string().lines().next().unwrap().len();
    let table_methods_width = table_methods.to_string().lines().next().unwrap().len();
    match table_fields_width.cmp(&table_methods_width) {
        std::cmp::Ordering::Less => {
            table_fields = table_fields.with(MinWidth::new(table_methods_width))
        }
        std::cmp::Ordering::Greater => {
            table_methods = table_methods.with(MinWidth::new(table_fields_width))
        }
        std::cmp::Ordering::Equal => (),
    }

    Builder::default()
        .add_record([table_fields.to_string()])
        .add_record([table_methods.to_string()])
        .set_columns([name])
        .build()
        .with(Style::ascii().horizontal_off().vertical_off())
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Rows::first()).with(Alignment::center()))
}
