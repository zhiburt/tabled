//! The example can be run by this command
//! `cargo run --example width`

use tabled::{Alignment, Full, MaxWidth, Modify, Style, Table};

fn main() {
    let data = [
        ["Hello World", "123123123231"],
        ["Hello World", "zxczczxcxczxczxc"],
        ["Hello World", "[[[[[[[[[[[[[[[[["],
    ];

    let table = Table::new(&data).with(Style::GITHUB_MARKDOWN).with(
        Modify::new(Full)
            .with(MaxWidth::truncating(10, "..."))
            .with(Alignment::left()),
    );

    println!("{}", table);

    let table = table.with(Modify::new(Full).with(MaxWidth::wrapping(5)));

    println!("{}", table);
}
