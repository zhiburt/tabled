use tabled::{
    settings::{themes::BorderCorrection, Alignment, Span, Style},
    Table,
};

fn main() {
    let data = [
        ("0,0", "0,1", "0,2", "0,3", ("0,4", "0,5", "0,6")),
        ("1,0", "1,1", "1,2", "1,3", ("1,4", "1,5", "1,6")),
        ("2,0", "2,1", "2,2", "2,3", ("2,4", "2,5", "2,6")),
        ("3,0", "3,1", "3,2", "3,3", ("3,4", "3,5", "3,6")),
    ];

    let mut table = Table::new(data);

    table.with(Style::modern_rounded());

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
    table.with(BorderCorrection::span());
    table.with(Alignment::center_vertical());

    println!("{table}");
}
