use tabled::{
    settings::{themes::BorderCorrection, Alignment, Span, Style},
    Table,
};

fn main() {
    let data = [
        ("0-0", "0-1", "0-2"),
        ("1-0", "1-1", "1-2"),
        ("2-0", "2-1", "2-2"),
        ("3-0", "3-1", "3-2"),
        ("4-0", "4-1", "4-2"),
        ("5-0", "5-1", "5-2"),
        ("6-0", "6-1", "6-2"),
    ];

    let mut table = Table::new(data);

    table.with(Style::modern_rounded());

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
    table.with(BorderCorrection::span());
    table.with(Alignment::center());

    println!("{table}");
}
