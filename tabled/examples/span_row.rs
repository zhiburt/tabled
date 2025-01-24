//! [COMMENTED] A list of examples for a span use.

use tabled::{
    settings::{style::BorderSpanCorrection, Alignment, Span, Style},
    Table,
};

fn main() {
    let data = [
        (1, 2, 3, 5, (6, 7, 8)),
        (9, 10, 11, 12, (13, 14, 15)),
        (16, 17, 18, 19, (20, 21, 22)),
        (23, 24, 25, 26, (27, 28, 29)),
    ];
    let mut table = Table::new(data);

    table.with(Style::modern_rounded());

    // Here we show how original table looks.
    println!("{table}");

    // Then there's a list of Span appliences and it's affects.

    // Spread cell by 1 row to the right.
    table.modify((0, 0), Span::row(2));

    // Spread cell all the way to the right,
    // Which essentially covers all columns.
    table.modify((0, 1), Span::row(isize::MAX));

    // Spread cell by 1 row to the top.
    table.modify((1, 2), Span::row(-1));

    // Spread cell all the way to the top.
    table.modify((2, 3), Span::row(isize::MIN));

    // Spread cell to cover the whole column.
    table.modify((0, 4), Span::row(0));

    // Spread cell to cover the whole column.
    table.modify((1, 5), Span::row(0));

    // Spread cell to cover the whole column.
    table.modify((2, 6), Span::row(0));

    // Set a default span for a cell,
    // Essentially removing a span setting for it.
    table.modify((0, 4), Span::row(1));

    // Correct the style to look good
    table.with(BorderSpanCorrection);
    table.with(Alignment::center_vertical());

    println!("{table}");
}
