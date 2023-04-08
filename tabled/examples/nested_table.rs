//! This example demonstrates how [`Tables`](Table) can be comprised of other tables.
//!
//! * This first nested [`Table`] example showcases the [`Builder`] approach.
//!
//! * Note how a great deal of manual customizations have been applied to create a
//! highly unique display.
//!
//! * 🎉 Inspired by https://github.com/p-ranav/tabulate#nested-tables/

use std::iter::FromIterator;

use tabled::{
    builder::Builder,
    settings::{
        object::{Rows, Segment},
        style::{HorizontalLine, Style},
        Alignment, Modify, Padding, Width,
    },
    Table,
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

    let mut table = Builder::from_iter([
        [animal.to_string()],
        [String::from("▲")],
        [String::from("|")],
        [String::from("|")],
        [duck.to_string()],
    ])
    .build();
    table.with(Style::ascii().remove_horizontal()).with(
        Modify::new(Segment::all())
            .with(Padding::new(5, 5, 0, 0))
            .with(Alignment::center()),
    );

    println!("{table}");
}

fn create_class(name: &str, fields: &[(&str, &str, &str)], methods: &[&str]) -> Table {
    let fields = fields
        .iter()
        .map(|(field, t, d)| [format_field(field, t, d)]);
    let mut table_fields = Builder::from_iter(fields).build();
    table_fields.with(Style::ascii().remove_horizontal().remove_vertical());

    let methods = methods.iter().map(|method| [format_method(method)]);
    let mut table_methods = Builder::from_iter(methods).build();
    table_methods.with(Style::ascii().remove_horizontal().remove_vertical());

    let (table_fields, table_methods) = make_equal_width(table_fields, table_methods);

    let mut table = Builder::from_iter([
        [name.to_string()],
        [table_fields.to_string()],
        [table_methods.to_string()],
    ])
    .build();

    table
        .with(
            Style::ascii()
                .horizontals([HorizontalLine::new(1, Style::ascii().get_horizontal())])
                .remove_horizontal()
                .remove_vertical(),
        )
        .with(Modify::new(Segment::all()).with(Alignment::left()))
        .with(Modify::new(Rows::first()).with(Alignment::center()));

    table
}

fn format_field(field: &&str, field_type: &&str, default_value: &&str) -> String {
    if default_value.is_empty() {
        format!("+{field}: {field_type}")
    } else {
        format!("+{field}: {field_type} = {default_value:?}")
    }
}

fn format_method(method: &str) -> String {
    format!("+{method}()")
}

fn make_equal_width(mut table1: Table, mut table2: Table) -> (Table, Table) {
    // We want to make a fields table and methods table to have the same width.
    // To not set it to constant, we check a width of each of them and correct the other.
    //
    // it's safe to do .len() because we use ascii theme and no colors.

    let table1_width = table1.to_string().lines().next().unwrap().len();
    let table2_width = table2.to_string().lines().next().unwrap().len();

    match table1_width.cmp(&table2_width) {
        std::cmp::Ordering::Less => {
            table1.with(Width::increase(table2_width));
        }
        std::cmp::Ordering::Greater => {
            table2.with(Width::increase(table1_width));
        }
        std::cmp::Ordering::Equal => (),
    }

    (table1, table2)
}
