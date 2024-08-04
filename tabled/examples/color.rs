//! This example demonstrates using the [`Color`] [setting](tabled::settings) to
//! stylize text, backgrounds, and borders.
//!
//! * 🚩 This example requires the `color` feature.
//!
//! * Note how [`Format::content()`] is used to break out [`CellOption`]
//!   specifications. This is helpful for organizing extensive [`Table`] configurations.

use std::convert::TryFrom;

use owo_colors::OwoColorize;

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

    let clr_red = Color::try_from(' '.red().to_string()).unwrap();
    let clr_red_light = Color::try_from(' '.red().on_bright_white().to_string()).unwrap();
    let clr_blue = Color::try_from(' '.blue().to_string()).unwrap();
    let clr_green = Color::try_from(' '.green().to_string()).unwrap();
    let clr_purple = Color::try_from(' '.purple().to_string()).unwrap();

    let border = BorderColor::new()
        .bottom(clr_red)
        .corner_bottom_left(clr_purple.clone())
        .corner_bottom_right(clr_purple);

    let mut table = Table::new(data);
    table
        .with(Style::psql())
        .modify(Rows::first(), border)
        .modify(Columns::single(0), clr_red_light)
        .modify(Columns::single(1), clr_green)
        .modify(Columns::single(2), clr_blue);

    println!("{table}");
}
