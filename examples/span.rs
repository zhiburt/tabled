//! The example can be run by this command
//! `cargo run --example span`
//!
//! The table from the example originally inspired https://github.com/vdmeer/asciitable#column-span

use tabled::{object::Cell, ModifyObject, Span, Style, TableIteratorExt};

fn main() {
    let data = [["just 1 column"; 5]; 5];

    let cell_span = |r, c, span| Cell(r, c).modify().with(Span::column(span));

    let table = data
        .table()
        .with(cell_span(0, 0, 5).with(|_: &str| "span all 5 columns".to_string()))
        .with(cell_span(1, 0, 4).with(|_: &str| "span 4 columns".to_string()))
        .with(cell_span(2, 0, 3).with(|_: &str| "span 3 columns".to_string()))
        .with(cell_span(2, 3, 2).with(|_: &str| "span 2 columns".to_string()))
        .with(cell_span(3, 0, 2).with(|_: &str| "span 3 columns".to_string()))
        .with(cell_span(3, 2, 3).with(|_: &str| "span 3 columns".to_string()))
        .with(cell_span(4, 1, 4).with(|_: &str| "span 4 columns".to_string()))
        .with(Style::modern())
        .with(Style::correct_spans());

    println!("{}", table);
}
