//! The example can be run by this command
//! `cargo run --features color --example color`
//!
//! This example requires a color feature

use owo_colors::OwoColorize;

use tabled::{
    object::{Columns, Rows},
    style::{Style, Symbol},
    Alignment, Modify, Table, Tabled,
};

#[derive(Tabled)]
struct Bsd {
    distribution: &'static str,
    year_of_first_release: usize,
    is_active: bool,
}

fn main() {
    let data = vec![
        Bsd {
            distribution: "SunOS",
            year_of_first_release: 1982,
            is_active: false,
        },
        Bsd {
            distribution: "NetBSD",
            year_of_first_release: 1993,
            is_active: true,
        },
        Bsd {
            distribution: "FreeBSD",
            year_of_first_release: 1993,
            is_active: true,
        },
        Bsd {
            distribution: "BSD",
            year_of_first_release: 1978,
            is_active: false,
        },
        Bsd {
            distribution: "OpenBSD",
            year_of_first_release: 1995,
            is_active: true,
        },
    ];

    let table = Table::new(&data)
        .with(
            Style::psql()
                .header_intersection(Symbol::ansi('+'.purple().to_string()).unwrap())
                .header(Symbol::ansi('-'.purple().to_string()).unwrap())
                .vertical(Symbol::ansi('|'.purple().to_string()).unwrap()),
        )
        .with(Modify::new(Rows::first()).with(Alignment::center()))
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()))
        .with(Modify::new(Columns::single(0)).with(|s: &str| s.blue().to_string()))
        .with(Modify::new(Columns::single(1)).with(|s: &str| s.green().to_string()))
        .with(Modify::new(Columns::single(2)).with(|s: &str| s.red().to_string()));

    println!("{}", table);
}
