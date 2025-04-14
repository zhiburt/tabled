use tabled::{
    settings::{style::Style, themes::BorderCorrection, Alignment, Span},
    Table,
};

fn main() {
    let data = [["simple cell"; 5]; 5];

    let mut table = Table::new(data);
    table
        .modify((0, 0), (Span::column(5), "span all 5 columns"))
        .modify((1, 0), (Span::column(4), "span 4 columns"))
        .modify((2, 0), (Span::column(2), "span 2 columns"))
        .modify((2, 4), (Span::row(4), "span\n4\nrows"))
        .modify(
            (3, 1),
            (Span::row(2), Span::column(2), "span\n2\nrows\nand\ncolumns"),
        )
        .modify((2, 3), (Span::row(2), "span 2 rows"))
        .with(Style::modern())
        .with(BorderCorrection::span())
        .with(Alignment::center_vertical());

    println!("{table}");
}
