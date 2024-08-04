//! This example demonstrates using the [`Highlight`] [`TableOption`] to
//! decorate sections of a [`Table`] with a unique [`Border`].
//!
//! * Note how [`Highlight`] arguments can be chained together to
//!   create cross-sections and non-symmetrical shapes.

use tabled::{
    settings::{
        object::{Columns, Object, Rows},
        style::Style,
        Highlight,
    },
    Table,
};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let target = Columns::first()
        .not(Rows::last())
        .and(Rows::last() - 1)
        .and(Rows::last().intersect(Columns::last()));

    let mut table = Table::new(data);
    table.with(Style::modern());
    table.with(Highlight::outline(target, '*'));

    println!("{table}");
}
