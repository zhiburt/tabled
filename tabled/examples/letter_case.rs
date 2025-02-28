use std::iter::FromIterator;

use tabled::{
    builder::Builder,
    settings::{object::Columns, style::Style, Color, Format, LetterCase},
};

fn main() {
    let board = [
        ["Hello World", "Hello World", "Hello World"],
        [" hello   world ", " hello   world ", " hello   world "],
        ["   ", "   ", "   "],
        ["", "", ""],
    ];

    let mut table = Builder::from_iter(board).build();
    table.with(Style::modern());
    table.with(Format::content(|text| Color::BG_RED.colorize(text)));
    table.modify(Columns::first(), LetterCase::Upper);
    table.modify(Columns::single(1), LetterCase::Lower);
    table.modify(Columns::last(), LetterCase::Title);

    println!("{table}");
}
