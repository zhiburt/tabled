//! The example can be run by this command
//! `cargo run --example span`
//!
//! The table from the example originally inspired https://github.com/vdmeer/asciitable#column-span/

use tabled::{object::Cell, ModifyObject, Span, Style, TableIteratorExt};

fn main() {
    let data = [["just 1 column"; 5]; 5];

    let span_cell = |r, c, span, text: &'static str| {
        Cell(r, c)
            .modify()
            .with(Span::column(span))
            .with(move |_: &str| text.to_string())
    };

    let table = data
        .table()
        .with(span_cell(0, 0, 5, "span all 5 columns"))
        .with(span_cell(1, 0, 4, "span 4 columns"))
        .with(span_cell(2, 0, 3, "span 3 columns"))
        .with(span_cell(2, 3, 2, "span 2 columns"))
        .with(span_cell(3, 0, 2, "span 2 columns"))
        .with(span_cell(3, 2, 3, "span 3 columns"))
        .with(span_cell(4, 1, 4, "span 4 columns"))
        .with(Style::modern())
        .with(Style::correct_spans());

    println!("{}", table);
}
