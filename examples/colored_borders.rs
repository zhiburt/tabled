//! The example can be run by this command
//! `cargo run --features color --example colored_borders`
//!
//! This example requires a `color` feature.

use std::iter::FromIterator;

use owo_colors::OwoColorize;

use tabled::{
    color::Color,
    object::Segment,
    style::Style,
    style::{BorderColor, RawStyle, Symbol},
    Border, Highlight, Table, Tabled,
};

#[derive(Tabled)]
struct CodeEditor {
    name: &'static str,
    first_release: &'static str,
    developer: &'static str,
}

impl CodeEditor {
    fn new(name: &'static str, first_release: &'static str, developer: &'static str) -> Self {
        Self {
            name,
            first_release,
            developer,
        }
    }
}

fn main() {
    let data = [
        CodeEditor::new("Sublime Text 3", "2008", "Sublime HQ"),
        CodeEditor::new("Visual Studio Code", "2015", "Microsoft"),
        CodeEditor::new("Notepad++", "2003", "Don Ho"),
        CodeEditor::new("GNU Emacs", "1984", "Richard Stallman"),
        CodeEditor::new("Neovim", "2015", "Vim community"),
    ];

    let mut style = RawStyle::from(Style::extended());
    style.set_color_top(Color::FG_RED);
    style.set_color_bottom(Color::FG_CYAN);
    style.set_color_left(Color::FG_BLUE);
    style.set_color_right(Color::FG_GREEN);
    style.set_color_corner_top_left(Color::FG_BLUE);
    style.set_color_corner_top_right(Color::FG_RED);
    style.set_color_corner_bottom_left(Color::FG_CYAN);
    style.set_color_corner_bottom_right(Color::FG_GREEN);
    style.set_color_intersection_bottom(Color::FG_CYAN);
    style.set_color_intersection_top(Color::FG_RED);
    style.set_color_intersection_right(Color::FG_GREEN);
    style.set_color_intersection_left(Color::FG_BLUE);
    style.set_color_intersection(Color::FG_MAGENTA);
    style.set_color_horizontal(Color::FG_MAGENTA);
    style.set_color_vertical(Color::FG_MAGENTA);

    let mut table = Table::from_iter(&data);
    table.with(style);

    println!("{}", table);
}
