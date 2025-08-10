use tabled::{
    col, row,
    settings::{Alignment, Style},
};

fn main() {
    let mut table = col![
        "row 1",
        "row 2",
        row!["row 3"; 3],
        col!["row 4"; 2].with(Style::ascii_rounded()),
    ];

    table.with(Alignment::center());
    table.with(Style::modern());

    println!("{table}");
}
