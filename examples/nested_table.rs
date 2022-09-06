//! The example can be run by this command
//! `cargo run --example nested_table`
//!
//! The table is a take on the one from https://github.com/p-ranav/tabulate#nested-tables/

use std::iter::FromIterator;

use tabled::{
    builder::Builder,
    object::{Rows, Segment},
    style::HorizontalLine,
    Alignment, Modify, Padding, Style, Table, Width,
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
    .with(Style::ascii().off_horizontal())
    .with(
        Modify::new(Segment::all())
            .with(Padding::new(5, 5, 0, 0))
            .with(Alignment::center()),
    );

    println!("{}", t);
}

fn create_class(name: &str, fields: &[(&str, &str, &str)], methods: &[&str]) -> Table {
    let table_fields = Builder::from_iter(fields.iter().map(|(field, t, d)| {
        if d.is_empty() {
            [format!("+{}: {}", field, t)]
        } else {
            [format!("+{}: {} = {:?}", field, t, d)]
        }
    }))
    .build()
    .with(Style::ascii().off_horizontal().off_vertical());

    let table_methods = Builder::from_iter(methods.iter().map(|method| [format!("+{}()", method)]))
        .build()
        .with(Style::ascii().off_horizontal().off_vertical());

    let (table_fields, table_methods) = make_equal_width(table_fields, table_methods);

    let mut builder = Builder::default();
    builder
        .add_record([table_fields.to_string()])
        .add_record([table_methods.to_string()])
        .set_columns([name.to_string()]);
    builder
        .build()
        .with(
            Style::ascii()
                .horizontals([HorizontalLine::new(1, Style::ascii().get_horizontal())])
                .off_horizontal()
                .off_vertical(),
        )
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Rows::first()).with(Alignment::center()))
}

fn make_equal_width(mut table1: Table, mut table2: Table) -> (Table, Table) {
    // We want to make a fields table and methods table to have the same width.
    // To not set it to constant, we check a width of each of them and correct the other.
    //
    // it's safe to do .len() because we use ascii theme and no colors.

    let table1_width = table1.to_string().lines().next().unwrap().len();
    let table2_width = table2.to_string().lines().next().unwrap().len();
    match table1_width.cmp(&table2_width) {
        std::cmp::Ordering::Less => table1 = table1.with(Width::increase(table2_width)),
        std::cmp::Ordering::Greater => table2 = table2.with(Width::increase(table1_width)),
        std::cmp::Ordering::Equal => (),
    }

    (table1, table2)
}
