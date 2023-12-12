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

    let table_validated = Table::new(validated).with(Style::ascii()).to_string();
    let table_not_validated = Table::new(not_validated).with(Style::modern()).to_string();
    let table_unsure = Table::new(unsure).with(Style::ascii_rounded()).to_string();

    let output1 = row![table_validated, table_not_validated];
    let output2 = col![table_unsure; 3];

    let output3 = col![
        row![table_validated, table_not_validated].with(Style::empty()),
        table_unsure
    ]
    .with(Alignment::center())
    .to_string();

    println!("{output1}");
    println!("{output2}");
    println!("{output3}");
}
