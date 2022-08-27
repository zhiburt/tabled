//! The example can be run by this command
//! `cargo run --example group`

use tabled::{group, Style, Table, Tabled};

#[derive(Tabled)]
struct Person {
    name: String,
    age: u8,
    is_validated: bool,
}

fn main() {
    let data_a = [
        Person {
            name: "Sam".into(),
            age: 31,
            is_validated: true,
        },
        Person {
            name: "Sarah".into(),
            age: 26,
            is_validated: true,
        },
    ];

    let data_b = [
        Person {
            name: "Jack Black".into(),
            age: 51,
            is_validated: false,
        },
        Person {
            name: "Michelle Goldstein".into(),
            age: 44,
            is_validated: true,
        },
    ];

    let data_c = [
        Person {
            name: "Jon Doe".into(),
            age: 255,
            is_validated: false,
        },
        Person {
            name: "Mark Nelson".into(),
            age: 13,
            is_validated: true,
        },
        Person {
            name: "Terminal Monitor".into(),
            age: 00,
            is_validated: false,
        },
        Person {
            name: "Adam Blend".into(),
            age: 17,
            is_validated: true,
        },
    ];

    let table_a = Table::new(&data_a).with(Style::ascii());
    let table_b = Table::new(&data_b).with(Style::modern());
    let table_c = Table::new(&data_c).with(Style::ascii_rounded());

    println!("{}", group!(table_c, table_b));
    println!("\n\n");
    println!("{}", group!(table_c; 3));
    println!("\n\n");
    println!("{}", group!(table_a, table_b, table_c; 2));
}
