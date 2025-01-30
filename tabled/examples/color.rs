//! This example demonstrates using the [`Color`] [setting](tabled::settings) to
//! stylize text, backgrounds, and borders.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * Note how [`Format::content()`] is used to break out [`CellOption`]
//!   specifications. This is helpful for organizing extensive [`Table`] configurations.

use tabled::{
    settings::{
        object::{Columns, Rows},
        style::{BorderColor, Style},
        Color,
    },
    Table, Tabled,
};

#[derive(Tabled)]
struct Bsd {
    distribution: String,
    first_release: usize,
    is_active: bool,
}

impl Bsd {
    fn new(dist: &str, first_release: usize, is_active: bool) -> Self {
        Self {
            distribution: dist.to_string(),
            first_release,
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

    let border = BorderColor::new()
        .bottom(Color::FG_RED)
        .corner_bottom_left(Color::FG_MAGENTA)
        .corner_bottom_right(Color::FG_MAGENTA);

    let mut table = Table::new(data);
    table
        .with(Style::psql())
        .modify(Rows::first(), border)
        .modify(Columns::single(0), Color::FG_RED | Color::BG_BRIGHT_WHITE)
        .modify(Columns::single(1), Color::FG_GREEN)
        .modify(Columns::single(2), Color::FG_BLUE);

    println!("{table}");
}
