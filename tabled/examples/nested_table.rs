// Inspired by https://github.com/p-ranav/tabulate#nested-tables/

use std::iter::FromIterator;

use tabled::{
    builder::Builder,
    settings::{
        object::Rows,
        style::{HorizontalLine, Style},
        Alignment, Padding, Width,
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

    let data = [[animal], [create_connection(2)], [duck]];

    let mut table = Builder::from_iter(data).build();

    table
        .with(Style::ascii().remove_horizontal())
        .with(Padding::new(3, 3, 0, 0))
        .with(Alignment::center());

    println!("{table}");
}

fn create_connection(size: usize) -> String {
    std::iter::once("▲")
        .chain(std::iter::repeat_n("|", size))
        .collect::<Vec<_>>()
        .join("\n")
}

fn create_class(name: &str, fields: &[(&str, &str, &str)], methods: &[&str]) -> String {
    let fields = fields
        .iter()
        .map(|(field, t, d)| [format_field(field, t, d)]);
    let mut table_fields = Builder::from_iter(fields).build();
    table_fields.with(Style::ascii().remove_horizontal().remove_vertical());

    let methods = methods.iter().map(|method| [format_method(method)]);
    let mut table_methods = Builder::from_iter(methods).build();
    table_methods.with(Style::ascii().remove_horizontal().remove_vertical());

    let (table_fields, table_methods) = make_equal_width(table_fields, table_methods);

    let data = [
        [name.to_string()],
        [table_fields.to_string()],
        [table_methods.to_string()],
    ];

    let mut table = Builder::from_iter(data).build();

    let style = Style::ascii()
        .horizontals([(1, HorizontalLine::inherit(Style::ascii()))])
        .remove_horizontal()
        .remove_vertical();

    table
        .with(style)
        .with(Alignment::left())
        .modify(Rows::first(), Alignment::center());

    table.to_string()
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
    let table1_width = table1.total_width();
    let table2_width = table2.total_width();

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
