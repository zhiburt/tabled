//! [COMMENTED] A list of examples for a span use.

use tabled::{
    settings::{style::BorderSpanCorrection, Alignment, Span, Style},
    Table,
};

fn main() {
    let data = [
        (1, 2, 3),
        (4, 5, 6),
        (7, 8, 9),
        (10, 11, 12),
        (13, 14, 15),
        (16, 17, 18),
        (19, 20, 21),
    ];
    let mut table = Table::new(data);

    table.with(Style::modern_rounded());

    // Here we show how original table looks.
    println!("{table}");

    // Then there's a list of Span appliences and it's affects.

    // Spread cell by 1 column to the right.
    table.modify((0, 1), Span::column(2));

    // Spread cell all the way to the right,
    // Which essentially covers all row.
    table.modify((1, 0), Span::column(isize::MAX));

    // Spread cell by 1 column to the left.
    table.modify((2, 1), Span::column(-1));

    // Spread cell all the way to the left.
    table.modify((3, 2), Span::column(isize::MIN));

    // Spread cell to cover the whole row.
    table.modify((4, 0), Span::column(0));

    // Spread cell to cover the whole row.
    table.modify((5, 1), Span::column(0));

    // Spread cell to cover the whole row.
    table.modify((6, 2), Span::column(0));

    // Set a default span for a cell,
    // Essentially removing a span setting for it.
    table.modify((4, 0), Span::column(1));

    // Correct the style to look good
    table.with(BorderSpanCorrection);
    table.with(Alignment::center());

    println!("{table}");
}
