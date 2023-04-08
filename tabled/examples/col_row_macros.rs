//! This example demonstrates using the [`col!`] and [`row!`] macros to easily
//! organize multiple tables together into a single, new [`Table`] display.
//!
//! * 🚩 This example requires the `macros` feature.
//!
//! * Note how both macros can be used in combination to layer
//! several table arrangements together.
//!
//! * Note how [`col!`] and [`row!`] support idiomatic argument duplication
//! with the familiar `[T; N]` syntax.

use tabled::{
    col, row,
    settings::{Alignment, Style},
    Table, Tabled,
};

#[derive(Tabled)]
struct Person {
    name: String,
    age: u8,
    is_validated: bool,
}

impl Person {
    fn new(name: &str, age: u8, is_validated: bool) -> Self {
        Self {
            name: name.into(),
            age,
            is_validated,
        }
    }
}

fn main() {
    let validated = [Person::new("Sam", 31, true), Person::new("Sarah", 26, true)];

    let not_validated = [
        Person::new("Jack Black", 51, false),
        Person::new("Michelle Goldstein", 44, true),
    ];

    let unsure = [
        Person::new("Jon Doe", 255, false),
        Person::new("Mark Nelson", 13, true),
        Person::new("Terminal Monitor", 0, false),
        Person::new("Adam Blend", 17, true),
    ];

    let table_a = Table::new(&validated).with(Style::ascii()).to_string();
    let table_b = Table::new(&not_validated).with(Style::modern()).to_string();
    let table_c = Table::new(&unsure).with(Style::ascii_rounded()).to_string();

    let row_table = row![table_c, table_b];

    let col_table = col![table_c; 3];

    let mut row_col_table = col![row![table_a, table_b].with(Style::empty()), table_c];
    row_col_table.with(Alignment::center());

    println!("{row_table}\n{col_table}\n{row_col_table}",);
}
