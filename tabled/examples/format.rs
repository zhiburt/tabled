//! This example demonstrates using the [`Format`] [`CellOption`] factory to alter
//! the cells of a [`Table`].
//!
//! * Note how [`Format::content()`] gives access to the respective cell content for replacement.
//!   And [`Format::positioned()`] additionally provides the index coordinates of that cell.
//!
//! * Note how the [std] [`format!`] macro is used to update the values of the affected cells.

use tabled::{
    settings::{
        object::{Columns, Object, Rows},
        Format, Style,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Commit {
    id: &'static str,
    header: &'static str,
    message: &'static str,
}

fn main() {
    let data = [
        Commit {
            header: "bypass open-source transmitter",
            message: "index neural panel",
            id: "8ae4e8957caeaa467acbce963701e227af00a1c7",
        },
        Commit {
            header: "program online alarm",
            message: "copy bluetooth card",
            id: "48c76de71bd685486d97dc8f4f05aa6fcc0c3f86",
        },
        Commit {
            header: "CSV",
            message: "reboot mobile capacitor",
            id: "6ffc2a2796229fc7bf59471ad907f58b897005d0",
        },
    ];

    let mut table = Table::new(data);
    table.with(Style::psql());
    table.modify(
        Columns::first().not(Rows::first()),
        Format::content(|s| s.chars().take(5).collect()),
    );

    println!("{table}");
}
