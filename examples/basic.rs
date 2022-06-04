use std::iter::FromIterator;

use tabled::{
    builder::Builder,
    object::{Cell, Object},
    Modify, Padding, Span,
};

fn main() {
    let data = [
        ["Version", "IHL", "TOS", "Total length", ""],
        ["Identification", "", "", "Flags", "Frgment offset"],
        ["TTL", "", "Protocol", "Header checksum", ""],
        ["Source address", "", "", "", ""],
        ["Destination address", "", "", "", ""],
        ["Options", "", "", "", ""],
        ["Data", "", "", "", ""],
    ];

    let table = Builder::from_iter(data)
        .build()
        .with(Modify::new(Cell(0, 3)).with(Span::column(2)))
        .with(Modify::new(Cell(1, 0)).with(Span::column(3)))
        .with(Modify::new(Cell(2, 0)).with(Span::column(2)))
        .with(Modify::new(Cell(2, 3)).with(Span::column(2)))
        .with(Modify::new(Cell(3, 0)).with(Span::column(5)))
        .with(Modify::new(Cell(4, 0)).with(Span::column(5)))
        .with(Modify::new(Cell(5, 0)).with(Span::column(5)))
        .with(Modify::new(Cell(6, 0)).with(Span::column(5)))
        .with(Modify::new(Cell(5, 0).and(Cell(6, 0))).with(Padding::new(1, 1, 1, 1)));

    println!("{}", table);
}
