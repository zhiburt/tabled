//! This example demonstrates using [`Highlight::reverse`] to draw a border
//! around a cutout from the side of the cells that surround it.
//!
//! * The bottom right corner is merged into one cell and cleared with [`Border::filled`].
//! * [`Highlight::reverse`] then redraws only the edges the cutout shares with the
//!   rest of the table, using the surrounding cells to pick the right corners
//!   and intersections.

use tabled::{
    builder::Builder,
    settings::{object::Segment, style::Style, themes::BorderCorrection, Border, Highlight, Span},
};

fn main() {
    let mut builder = Builder::new();
    builder.push_record(["", "a", "b", "c"]);
    builder.push_record(["1", "", "", ""]);
    builder.push_record(["2", "", "x", ""]);
    builder.push_record(["3", "", "", ""]);

    let cutout = Segment::new(2.., 2..);

    let mut table = builder.build();
    table
        .with(Style::modern())
        .modify((2, 2), (Span::column(2), Span::row(2)))
        .with(BorderCorrection::span())
        .modify(cutout.clone(), Border::filled(' '))
        .with(
            Highlight::new(cutout)
                .border(Border::inherit(Style::modern()))
                .reverse(),
        );

    println!("{table}");
}
