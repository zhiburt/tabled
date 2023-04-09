//! This example demonstrates using the [`Color`] [setting](tabled::settings) to
//! stylize text, backgrounds, and borders.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * Note how [`Format::content()`] is used to break out [`CellOption`]
//! specifications. This is helpful for organizing extensive [`Table`] configurations.

use std::convert::TryFrom;

use owo_colors::OwoColorize;

use tabled::{
    settings::{
        object::{Columns, Rows},
        style::{BorderColor, Style},
        Color, Format, Modify,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Bsd {
    distribution: &'static str,
    year_of_first_release: usize,
    is_active: bool,
}

impl Bsd {
    fn new(distribution: &'static str, year_of_first_release: usize, is_active: bool) -> Self {
        Self {
            distribution,
            year_of_first_release,
            is_active,
        }
    }
}

fn main() {
    let data = vec![
        Bsd::new("BSD", 1978, false),
        Bsd::new("SunOS", 1982, false),
        Bsd::new("NetBSD", 1993, true),
        Bsd::new("FreeBSD", 1993, true),
        Bsd::new("OpenBSD", 1995, true),
    ];

    let red = Format::content(|s| s.red().on_bright_white().to_string());
    let blue = Format::content(|s| s.blue().to_string());
    let green = Format::content(|s| s.green().to_string());

    let color_red = Color::try_from(' '.red().to_string()).unwrap();
    let color_purple = Color::try_from(' '.purple().to_string()).unwrap();

    let yellow_color = Color::try_from(' '.yellow().to_string()).unwrap();

    let first_row_style = Modify::new(Rows::first()).with(
        BorderColor::default()
            .bottom(color_red)
            .corner_bottom_left(color_purple.clone())
            .corner_bottom_right(color_purple),
    );

    let mut table = Table::new(data);
    table
        .with(Style::psql())
        .with(yellow_color)
        .with(first_row_style)
        .with(Modify::new(Columns::single(0)).with(red))
        .with(Modify::new(Columns::single(1)).with(green))
        .with(Modify::new(Columns::single(2)).with(blue));

    println!("{table}");
}
