//! The example can be run by this command
//! `cargo run --example col_row_macros --features="macros"`

use tabled::{col, object::Segment, row, Alignment, Modify, Style, Table, Tabled};

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

    let table_a = Table::new(&validated).with(Style::ascii());
    let table_b = Table::new(&not_validated).with(Style::modern());
    let table_c = Table::new(&unsure).with(Style::ascii_rounded());

    println!("{}", row![table_c, table_b]);
    println!();
    println!("{}", col![table_c; 3]);
    println!();
    println!(
        "{}",
        col![row![table_a, table_b].with(Style::empty()), table_c]
            .with(Modify::new(Segment::all()).with(Alignment::center()))
    );
}
