use std::iter::FromIterator;

use tabled::{
    settings::{
        object::{Columns, Object, Rows},
        Color, Highlight,
    },
    Table,
};

fn main() {
    let data = vec![["A", "B", "C"], ["D", "E", "F"], ["G", "H", "I"]];

    let mut table = Table::from_iter(data);
    table.with(Highlight::colored(
        Rows::first().and(Columns::one(1)),
        Color::FG_GREEN,
    ));

    println!("{table}");
}
