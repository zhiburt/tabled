//! The example can be run by this command
//! `cargo run --features color --example color`
//!
//! This example requires a color feature

use std::convert::TryFrom;

use owo_colors::OwoColorize;

use tabled::{
    object::{Columns, Rows},
    style::{BorderColored, Color, Style, Symbol},
    ModifyObject, Table, Tabled,
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

    let red = |s: &str| s.red().on_bright_white().to_string();
    let blue = |s: &str| s.blue().to_string();
    let green = |s: &str| s.green().to_string();
    let red_split = |c: char| Symbol::ansi(c.red().to_string()).unwrap();
    let purple_split = |c: char| Symbol::ansi(c.purple().to_string()).unwrap();
    let yellow_color = Color::try_from(' '.yellow().to_string()).unwrap();

    let first_row_style = Rows::first().modify().with(
        BorderColored::default()
            .bottom(red_split('-'))
            .bottom_left_corner(purple_split('+'))
            .bottom_right_corner(purple_split('+')),
    );

    let table = Table::new(&data)
        .with(Style::psql())
        .with(yellow_color)
        .with(first_row_style)
        .with(Columns::single(0).modify().with(red))
        .with(Columns::single(1).modify().with(green))
        .with(Columns::single(2).modify().with(blue));

    println!("{}", table);
}
