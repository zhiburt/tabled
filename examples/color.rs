//! The example can be run by this command
//! `cargo run --features color --example color`
//!
//! This example requires a color feature

use std::convert::TryFrom;

use owo_colors::OwoColorize;

use papergrid::{BorderColor, ColoredBorder};
use tabled::{
    object::{Columns, Object, Rows},
    style::{Style, Symbol},
    Alignment, ModifyObject, Table, Tabled,
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

    let table =
        Table::new(&data)
            .with(Style::psql())
            .with(BorderColor::try_from(" ".yellow().to_string()).unwrap())
            .with(Rows::first().modify().with(
                ColoredBorder::default().bottom(Symbol::ansi('-'.red().to_string()).unwrap()),
            ))
            .with(
                Rows::first().not(Columns::first()).modify().with(
                    ColoredBorder::default()
                        .bottom_left_corner(Symbol::ansi('+'.purple().to_string()).unwrap()),
                ),
            )
            .with(Rows::first().modify().with(Alignment::center()))
            .with(Rows::new(1..).modify().with(Alignment::left()))
            .with(
                Columns::single(0)
                    .modify()
                    .with(|s: &str| s.blue().to_string()),
            )
            .with(
                Columns::single(1)
                    .modify()
                    .with(|s: &str| s.green().to_string()),
            )
            .with(
                Columns::single(2)
                    .modify()
                    .with(|s: &str| s.red().to_string()),
            );

    println!("{}", table);
}
