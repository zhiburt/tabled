use tabled::{
    settings::{
        object::{Columns, Rows},
        style::{Border, HorizontalLine, LineText, Style},
        Alignment, Theme,
    },
    Table,
};

fn main() {
    let data = [
        [1, 2, 3, 4, 5],
        [5, 6, 7, 8, 9],
        [9, 10, 11, 12, 13],
        [13, 14, 15, 16, 17],
    ];

    let mut table = Table::new(data);

    let mut theme = Theme::from_style(Style::modern());
    theme.remove_horizontal_lines();
    theme.insert_horizontal_line(
        1,
        HorizontalLine::inherit(Style::modern()).left(Border::inherit(Style::modern()).get_left()),
    );

    table
        .with(theme)
        .with(LineText::new("Columns", Rows::first()).offset(1))
        .with(LineText::new("Numbers", Rows::single(1)).offset(1))
        .with(LineText::new("end", Rows::last() + 1).offset(1))
        .with(LineText::new("Data", Columns::last() + 1).align(Alignment::center_vertical()));

    println!("{table}");
}
