//! We can set colors for particular cells by using [`Grid`]
//!
//! The example can be run by this command
//! `cargo run --example grid_colors`
//!
//! [`Grid`]: tabled::grid::Grid

use std::collections::HashMap;

use tabled::{
    grid::Grid,
    settings::{color::Color, style::Style},
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

    let mut table = Table::new(data);
    table.with(Style::psql());

    let colors = HashMap::from([
        ((0, 0), Color::BG_BLUE),
        ((1, 1), Color::BG_GREEN),
        ((2, 2), Color::BG_RED),
    ]);

    let grid = Grid::from(table).with_colors(colors);

    println!("{grid}");
}
