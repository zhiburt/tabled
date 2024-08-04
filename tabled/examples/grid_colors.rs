//! This example demonstrates using [`Color`] as a [`CellOption`] modifier to stylize
//! the cells of a [`Table`].
//!
//! * Note how the [`Color`] [setting](tabled::settings) is used to simplify creating
//!   reusable themes for backgrounds.

use tabled::{
    settings::{Color, Style},
    Table, Tabled,
};

#[derive(Tabled)]
struct Bsd {
    distribution: String,
    year_of_first_release: usize,
    is_active: bool,
}

impl Bsd {
    fn new(distribution: &str, year_of_first_release: usize, is_active: bool) -> Self {
        Self {
            distribution: distribution.to_string(),
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

    let mut table = Table::new(data);
    table
        .with(Style::psql())
        .modify((0, 0), Color::BG_BLUE)
        .modify((1, 1), Color::BG_GREEN)
        .modify((2, 2), Color::BG_RED);

    println!("{table}");
}
